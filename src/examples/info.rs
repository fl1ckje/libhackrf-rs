use libhackrf::{HackRF, Off};

fn main() {
    let hackrf: HackRF<Off> = HackRF::new().expect("Failed to open HackRF One");
    get_info(&hackrf);
}

fn get_info(hackrf: &HackRF<Off>) {
    println!("Board id: {:?}", hackrf.board_id().unwrap());
    println!("Firmware version: {:?}", hackrf.version().unwrap());
    println!("API version: {:?}", hackrf.device_version().to_string());
}
