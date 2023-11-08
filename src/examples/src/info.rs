use libhackrf::HackRF;

fn main() {
    let sdr: HackRF = HackRF::new().expect("Failed to open HackRF One");
    println!("Board id: {:?}", sdr.board_id().unwrap());
    println!("Firmware version: {:?}", sdr.version().unwrap());
    println!("API version: {:?}", sdr.device_version().to_string());
}
