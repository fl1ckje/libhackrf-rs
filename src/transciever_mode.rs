#[repr(u8)]
pub enum TranscieverMode {
    Off = 0,
    Receive = 1,
    Transmit = 2,
    Ss = 3,
    CpldUpdate = 4,
    RxSweep = 5,
}

impl From<TranscieverMode> for u8 {
    fn from(tm: TranscieverMode) -> Self {
        tm as u8
    }
}

impl From<TranscieverMode> for u16 {
    fn from(tm: TranscieverMode) -> Self {
        tm as u16
    }
}
