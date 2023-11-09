use libhackrf::HackRF;

fn main() {
    let sdr: HackRF = HackRF::new().expect("Failed to open HackRF One");
    println!("Board id: {:}", sdr.board_id().unwrap());
    println!("Firmware version: {:}", sdr.version().unwrap());
    println!("API version: {:}", sdr.device_version().to_string());

    let part_and_serial: ((u32, u32), String) = sdr.part_id_serial_read().unwrap();
    println!(
        "{}",
        format!(
            "Part id number: 0x{:08x?} 0x{:08x?}\nSerial number: {:032x?}",
            part_and_serial.0 .0, part_and_serial.0 .1, part_and_serial.1,
        )
    );
}
