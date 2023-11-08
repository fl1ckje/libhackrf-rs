#[repr(u8)]
pub enum TransceiverMode {
    Off = 0,
    Receive = 1,
    Transmit = 2,
    Ss = 3,
    CpldUpdate = 4,
    RxSweep = 5,
}

impl From<TransceiverMode> for u8 {
    fn from(tm: TransceiverMode) -> Self {
        tm as u8
    }
}

impl From<TransceiverMode> for u16 {
    fn from(tm: TransceiverMode) -> Self {
        tm as u16
    }
}
