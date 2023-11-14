use libhackrf::HackRF;
use std::{
    sync::mpsc::{channel, TryRecvError},
    thread,
    time::Duration,
};

struct Args {
    fs: u32,
    fc: u64,
    filter_bw: u32,
    lna_gain: u16,
    vga_gain: u16,
    amp: bool,
    bias_tee: u8,
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

    const RECORD_BUFFER_SIZE: usize = 1024 * 1024;
    let mut record_buffer: Vec<[u8; 2]> = Vec::with_capacity(RECORD_BUFFER_SIZE);

    let (samples_sender, samples_receiver) = channel();
    let (exit_tx, exit_rx) = channel();

    let sample_thread: thread::JoinHandle<Result<(), libhackrf::Error>> =
        thread::spawn(move || -> Result<(), libhackrf::Error> {
            println!("Sample thread has been spawned");

            loop {
                let samples: Vec<u8> = sdr.rx()?;
                samples_sender
                    .send(samples)
                    .expect("Failed to send buffer data from sample thread");

                match exit_rx.try_recv() {
                    Ok(_) => {
                        sdr.stop_rx()?;
                        return Ok(());
                    }
                    Err(TryRecvError::Disconnected) => {
                        println!("Main thread disconnected");
                        return Ok(());
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        });

    let mut i: i32 = 0;
    loop {
        match samples_receiver.try_recv() {
            Ok(buffer) => {
                buffer.chunks_exact(2).for_each(|iq: &[u8]| {
                    record_buffer.push([iq[0], iq[1]]);
                });
                println!("{}", buffer.len());
                thread::sleep(Duration::from_secs(1));
                i += 1;
                println!("RX time: {} s.", i);
                if i == 5 {
                    break;
                }
            }
            Err(TryRecvError::Disconnected) => {
                println!("Sample thread disconnected");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
        // you can do samples processing here
        // or wait for the buffer to fill and do processing outside loop after rx sample thread is closed:
        // if record_buffer.len() >= RECORD_BUFFER_SIZE {
        //     break;
        // }
    }

    println!("Shutting down sample thread");

    match exit_tx.send(()) {
        Ok(()) => (),
        Err(e) => println!("Failed to send exit event (exit_rx disconnected): {}", e),
    }
    sample_thread
        .join()
        .expect("Failed to join sample thread")
        .expect("Sample thread returned an error");

    println!("Done");
}
