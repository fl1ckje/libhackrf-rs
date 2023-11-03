#[repr(u8)]
pub enum Request {
    SetTransceiverMode = 1,
    SampleRateSet = 6,
    BasebandFilterBandwidthSet = 7,
    BoardIdRead = 14,
    VersionStringRead = 15,
    SetFreq = 16,
    AmpEnable = 17,
    SetLnaGain = 19,
    SetVgaGain = 20,
    SetTxvgaGain = 21,
    AntennaEnable = 23,
    Reset = 30,
    ClkoutEnable = 32,
}

impl From<Request> for u8 {
    fn from(r: Request) -> Self {
        r as u8
    }
}
