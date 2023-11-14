use libhackrf::HackRF;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{
    env,
    fs::File,
    io::Write,
    path::PathBuf,
    sync::mpsc::{channel, TryRecvError},
    thread,
    time::{Duration, SystemTime},
};

struct Args {
    fs: u32,
    fc: u64,
    filter_bw: u32,
    lna_gain: u16,
    vga_gain: u16,
    amp: bool,
    bias_tee: u8,
    file_name: String,
}

fn main() {
    receive(Args {
        fs: 10_000_000,
        fc: 87_600_000,
        filter_bw: 4_000_000,
        lna_gain: 20,
        vga_gain: 32,
        amp: false,
        bias_tee: 0,
        file_name: "samples.dat".to_owned(),
    });
}

fn receive(args: Args) {
    let mut sdr: HackRF = HackRF::new().expect("Failed to open HackRF One");

    sdr.set_sample_rate_auto(args.fs)
        .expect("Failed to set sample rate");

    sdr.set_freq(args.fc)
        .expect("Failed to set carrier frequency");

    sdr.set_baseband_filter_bandwidth(args.filter_bw)
        .expect("Failed to set baseband filter bandwidth");

    sdr.set_amp_enable(args.amp)
        .expect("Failed to disable amplifier");

    sdr.set_antenna_enable(args.bias_tee)
        .expect("Failed to disable antenna power");

    sdr.set_lna_gain(args.lna_gain)
        .expect("Failed to set LNA gain");

    sdr.set_vga_gain(args.vga_gain)
        .expect("Failed to set VGA gain");

    sdr.enter_rx_mode().expect("Failed to enter RX mode");

    let mut write_dir: PathBuf = env::current_dir().unwrap();
    write_dir.push(args.file_name);
    let write_path: PathBuf = write_dir.into();
    let mut file: File = File::create(write_path).expect("Failed to create file");
    let start_time: SystemTime = SystemTime::now();

    let (status_tx, status_rx) = channel();
    let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_clone = exit_flag.clone();
    let sample_thread: thread::JoinHandle<Result<(), libhackrf::Error>> =
        thread::spawn(move || -> Result<(), libhackrf::Error> {
            println!("Sample thread has been spawned");
            loop {
                let samples: Vec<u8> = sdr.rx()?;
                file.write_all(&samples).expect("Failed to write to file");
                status_tx
                    .send(())
                    .expect("Failed to send status from sample thread");
                if exit_flag_clone.load(Ordering::Relaxed) {
                    sdr.stop_rx().unwrap();
                    return Ok(());
                }
            }
        });

    let exit_flag_clone = exit_flag.clone();
    ctrlc::set_handler(move || {
        exit_flag_clone.store(true, Ordering::Relaxed);
    })
    .expect("Failed to set Ctrl-C handler");

    loop {
        match status_rx.try_recv() {
            Ok(_) => {
                let elapsed_time: Duration = start_time.elapsed().unwrap();
                let total_seconds: u64 = elapsed_time.as_secs();
                let hours: u64 = total_seconds / 3600;
                let minutes: u64 = (total_seconds % 3600) / 60;
                let seconds: u64 = total_seconds % 60;

                if hours >= 1 {
                    println!("RX time: {} hr {} min {} s", hours, minutes, seconds);
                } else if minutes >= 1 {
                    println!("RX time: {} min {} s", minutes, seconds);
                } else {
                    println!("RX time: {} s", seconds);
                }
                thread::sleep(Duration::from_secs(1));
            }
            Err(TryRecvError::Disconnected) => {
                println!("Sample thread disconnected");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
        if exit_flag.load(Ordering::Relaxed) {
            println!("User keystroke detected");
            break;
        }
    }

    println!("Shutting down sample thread");
    sample_thread
        .join()
        .expect("Failed to join sample thread")
        .expect("Sample thread returned an error");
}
