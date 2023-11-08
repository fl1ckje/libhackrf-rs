use rusb::constants::{LIBUSB_ENDPOINT_IN, LIBUSB_ENDPOINT_OUT};

pub const HACKRF_USB_VID: u16 = 0x1D50;
pub const HACKRF_ONE_USB_PID: u16 = 0x6089;
pub const RX_ENDPOINT_ADDRESS: u8 = LIBUSB_ENDPOINT_IN | 1;
pub const TX_ENDPOINT_ADDRESS: u8 = LIBUSB_ENDPOINT_OUT | 2;
pub const MAX_TRANSMISSION_UNIT: usize = 128 * 1024;
pub const MHZ: u64 = 1_000_000;
pub const TRANSFER_SIZE: usize = 262_144;
pub const MAX_N: usize = 32;