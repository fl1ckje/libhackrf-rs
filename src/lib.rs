mod constants;
mod request;
mod tests;
mod transciever_mode;

use constants::*;
use request::*;
use transciever_mode::*;

use rusb::{
    request_type, DeviceDescriptor, DeviceHandle, Direction, GlobalContext, Recipient, RequestType,
    UsbContext, Version,
};
use std::time::Duration;

pub const MTU: usize = constants::MAX_TRANSMISSION_UNIT;

#[derive(Debug)]
pub struct Rx;

#[derive(Debug)]
pub struct Tx;

#[derive(Debug)]
pub struct Off;

pub struct HackRF<Mode> {
    device_handle: DeviceHandle<GlobalContext>,
    description: DeviceDescriptor,

    #[allow(dead_code)]
    mode: Mode,
    timeout: Duration,
}

impl HackRF<Off> {
    pub fn new() -> Option<HackRF<Off>> {
        let context: GlobalContext = GlobalContext {};
        let devices = match context.devices() {
            Ok(dev) => dev,
            Err(_) => return None,
        };

        for device in devices.iter() {
            let description = match device.device_descriptor() {
                Ok(dev) => dev,
                Err(_) => continue,
            };

            if description.vendor_id() == HACKRF_USB_VID
                && description.product_id() == HACKRF_ONE_USB_PID
            {
                match device.open() {
                    Ok(handle) => {
                        return Some(HackRF {
                            device_handle: handle,
                            description,
                            mode: Off,
                            timeout: Duration::from_secs(1),
                        })
                    }
                    Err(_) => continue,
                }
            }
        }
        None
    }
}

impl<Mode> HackRF<Mode> {
    fn read_control<const N: usize>(
        &self,
        request: Request,
        value: u16,
        index: u16,
    ) -> Result<[u8; N], Error> {
        let mut buffer: [u8; N] = [0; N];

        let n: usize = self.device_handle.read_control(
            request_type(Direction::In, RequestType::Vendor, Recipient::Device),
            request.into(),
            value,
            index,
            &mut buffer,
            self.timeout,
        )?;

        if n != buffer.len() {
            Err(Error::ControlTransfer {
                direction: Direction::In,
                actual: n,
                expected: buffer.len(),
            })
        } else {
            Ok(buffer)
        }
    }

    fn write_control(
        &mut self,
        request: Request,
        value: u16,
        index: u16,
        buffer: &[u8],
    ) -> Result<(), Error> {
        let n: usize = self.device_handle.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Device),
            request.into(),
            value,
            index,
            buffer,
            self.timeout,
        )?;
        if n != buffer.len() {
            Err(Error::ControlTransfer {
                direction: Direction::Out,
                actual: n,
                expected: buffer.len(),
            })
        } else {
            Ok(())
        }
    }

    fn check_api_version(&self, minimal: Version) -> Result<(), Error> {
        fn version_to_u32(version: Version) -> u32 {
            ((version.major() as u32) << 16)
                | ((version.minor() as u32) << 8)
                | (version.sub_minor() as u32)
        }

        let device_version: Version = self.device_version();
        let device_version_cmp: u32 = version_to_u32(device_version);
        let minimal_version_cmp: u32 = version_to_u32(minimal);

        if device_version_cmp >= minimal_version_cmp {
            Ok(())
        } else {
            Err(Error::Version {
                device: device_version,
                minimal,
            })
        }
    }
    pub fn device_version(&self) -> Version {
        self.description.device_version()
    }

    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout = duration;
    }

    pub fn board_id(&self) -> Result<u8, Error> {
        let data: [u8; 1] = self.read_control(Request::BoardIdRead, 0, 0)?;
        Ok(data[0])
    }

    pub fn version(&self) -> Result<String, Error> {
        let mut buffer: [u8; 16] = [0; 16];
        let n: usize = self.device_handle.read_control(
            request_type(Direction::In, RequestType::Vendor, Recipient::Device),
            Request::VersionStringRead.into(),
            0,
            0,
            &mut buffer,
            self.timeout,
        )?;
        Ok(String::from_utf8_lossy(&buffer[0..n]).into())
    }

    pub fn set_freq(&mut self, hz: u64) -> Result<(), Error> {
        let buffer: [u8; 8] = set_freq_helper(hz);
        self.write_control(Request::SetFreq, 0, 0, &buffer)
    }

    pub fn set_amp_enable(&mut self, en: bool) -> Result<(), Error> {
        self.write_control(Request::AmpEnable, en.into(), 0, &[])
    }

    pub fn set_baseband_filter_bandwidth(&mut self, hz: u32) -> Result<(), Error> {
        self.write_control(
            Request::BasebandFilterBandwidthSet,
            (hz & 0xFFFF) as u16,
            (hz >> 16) as u16,
            &[],
        )
    }

    pub fn set_sample_rate(&mut self, hz: u32, divider: u32) -> Result<(), Error> {
        let hz: u32 = hz.to_le();
        let div: u32 = divider.to_le();

        let buffer: [u8; 8] = [
            (hz & 0xFF) as u8,
            ((hz >> 8) & 0xFF) as u8,
            ((hz >> 16) & 0xFF) as u8,
            ((hz >> 24) & 0xFF) as u8,
            (div & 0xFF) as u8,
            ((div >> 8) & 0xFF) as u8,
            ((div >> 16) & 0xFF) as u8,
            ((div >> 24) & 0xFF) as u8,
        ];

        self.write_control(Request::SampleRateSet, 0, 0, &buffer)?;
        self.set_baseband_filter_bandwidth((0.75 * (hz as f32) / (div as f32)) as u32)
    }

    pub fn set_lna_gain(&mut self, value: u16) -> Result<(), Error> {
        if value > 40 {
            Err(Error::Argument)
        } else {
            let buffer: [u8; 1] = self.read_control(Request::SetLnaGain, 0, value & !0x07)?;
            if buffer[0] == 0 {
                Err(Error::Argument)
            } else {
                Ok(())
            }
        }
    }

    pub fn set_vga_gain(&mut self, value: u16) -> Result<(), Error> {
        if value > 62 {
            Err(Error::Argument)
        } else {
            let buffer: [u8; 1] = self.read_control(Request::SetVgaGain, 0, value & !0b1)?;
            if buffer[0] == 0 {
                Err(Error::Argument)
            } else {
                Ok(())
            }
        }
    }

    pub fn set_txvga_gain(&mut self, value: u16) -> Result<(), Error> {
        if value > 47 {
            Err(Error::Argument)
        } else {
            let buffer: [u8; 1] = self.read_control(Request::SetTxvgaGain, 0, value)?;
            if buffer[0] == 0 {
                Err(Error::Argument)
            } else {
                Ok(())
            }
        }
    }

    pub fn set_antenna_enable(&mut self, value: u8) -> Result<(), Error> {
        self.write_control(Request::AntennaEnable, value.into(), 0, &[])
    }

    pub fn set_clkout_enable(&mut self, value: bool) -> Result<(), Error> {
        self.check_api_version(Version::from_bcd(0x0103))?;
        self.write_control(Request::ClkoutEnable, value.into(), 0, &[])
    }

    pub fn reset(mut self) -> Result<HackRF<Off>, Error> {
        self.check_api_version(Version::from_bcd(0x0102))?;
        self.write_control(Request::Reset, 0, 0, &[])?;

        Ok(HackRF {
            device_handle: self.device_handle,
            description: self.description,
            mode: Off,
            timeout: self.timeout,
        })
    }

    fn set_transceiver_mode(&mut self, mode: TranscieverMode) -> Result<(), Error> {
        self.write_control(Request::SetTransceiverMode, mode.into(), 0, &[])
    }

    pub fn into_rx_mode(mut self) -> Result<HackRF<Rx>, Error> {
        self.set_transceiver_mode(TranscieverMode::Receive)?;
        self.device_handle.claim_interface(0)?;

        Ok(HackRF {
            device_handle: self.device_handle,
            description: self.description,
            mode: Rx,
            timeout: self.timeout,
        })
    }

    pub fn into_tx_mode(mut self) -> Result<HackRF<Tx>, Error> {
        self.set_transceiver_mode(TranscieverMode::Transmit)?;
        self.device_handle.claim_interface(0)?;
        Ok(HackRF {
            device_handle: self.device_handle,
            description: self.description,
            mode: Tx,
            timeout: self.timeout,
        })
    }
}

impl HackRF<Rx> {
    pub fn rx(&mut self) -> Result<Vec<u8>, Error> {
        let mut buffer: Vec<u8> = vec![0; MAX_TRANSMISSION_UNIT];
        let n: usize =
            self.device_handle
                .read_bulk(RX_ENDPOINT_ADDRESS, &mut buffer, self.timeout)?;
        buffer.truncate(n);

        Ok(buffer)
    }

    pub fn stop_rx(mut self) -> Result<HackRF<Off>, Error> {
        self.device_handle.release_interface(0)?;
        self.set_transceiver_mode(TranscieverMode::Off)?;

        Ok(HackRF {
            device_handle: self.device_handle,
            description: self.description,
            mode: Off,
            timeout: self.timeout,
        })
    }
}

impl HackRF<Tx> {
    pub fn tx(&mut self, mut buffer: Vec<u8>) -> Result<(), Error> {
        buffer.truncate(MAX_TRANSMISSION_UNIT);
        self.device_handle
            .write_bulk(TX_ENDPOINT_ADDRESS, &mut buffer, self.timeout)?;

        Ok(())
    }

    pub fn stop_tx(mut self) -> Result<HackRF<Off>, Error> {
        self.device_handle.release_interface(0)?;
        self.set_transceiver_mode(TranscieverMode::Off)?;

        Ok(HackRF {
            device_handle: self.device_handle,
            description: self.description,
            mode: Off,
            timeout: self.timeout,
        })
    }
}

fn set_freq_helper(hz: u64) -> [u8; 8] {
    let l_freq_mhz: u32 = u32::try_from(hz / MHZ).unwrap_or(u32::MAX).to_le();
    let l_freq_hz: u32 = u32::try_from(hz - u64::from(l_freq_mhz) * MHZ)
        .unwrap_or(u32::MAX)
        .to_le();

    [
        (l_freq_mhz & 0xFF) as u8,
        ((l_freq_mhz >> 8) & 0xFF) as u8,
        ((l_freq_mhz >> 16) & 0xFF) as u8,
        ((l_freq_mhz >> 24) & 0xFF) as u8,
        (l_freq_hz & 0xFF) as u8,
        ((l_freq_hz >> 8) & 0xFF) as u8,
        ((l_freq_hz >> 16) & 0xFF) as u8,
        ((l_freq_hz >> 24) & 0xFF) as u8,
    ]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    Usb(rusb::Error),
    ControlTransfer {
        direction: Direction,
        actual: usize,
        expected: usize,
    },

    Version {
        device: Version,
        minimal: Version,
    },
    Argument,
}

impl From<rusb::Error> for Error {
    fn from(error: rusb::Error) -> Self {
        Error::Usb(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl std::error::Error for Error {}