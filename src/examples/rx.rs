use libhackrf::{HackRF, Off, Rx};
use std::{
    sync::mpsc::{channel, TryRecvError},
    thread,
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
    let mut hackrf: HackRF<Off> = HackRF::new().expect("Failed to open HackRF One");
    const DIV: u32 = 1;

    hackrf
        .set_sample_rate(args.fs, DIV)
        .expect("Failed to set sample rate");

    hackrf
        .set_freq(args.fc)
        .expect("Failed to set carrier frequency");

    hackrf
        .set_baseband_filter_bandwidth(args.filter_bw)
        .expect("Failed to set baseband filter bandwidth");

    hackrf
        .set_amp_enable(args.amp)
        .expect("Failed to disable amplifier");

    hackrf
        .set_antenna_enable(args.bias_tee)
        .expect("Failed to disable antenna power");

    hackrf
        .set_lna_gain(args.lna_gain)
        .expect("Failed to set LNA gain");

    hackrf
        .set_vga_gain(args.vga_gain)
        .expect("Failed to set VGA gain");

    let mut hackrf: HackRF<Rx> = hackrf.into_rx_mode().expect("Failed to enter RX mode");

    let (samples_sender, samples_receiver) = channel();
    let (exit_sender, exit_receiver) = channel();

    let sample_thread = thread::Builder::new()
        .name("sample_thread".to_string())
        .spawn(move || -> Result<(), libhackrf::Error> {
            println!("Sample thread has been spawned");

            loop {
                let samples: Vec<u8> = hackrf.rx()?;
                samples_sender
                    .send(samples)
                    .expect("Failed to send buffer data from sample thread");

                match exit_receiver.try_recv() {
                    Ok(_) => {
                        hackrf.stop_rx()?;
                        return Ok(());
                    }
                    Err(TryRecvError::Disconnected) => {
                        println!("Main thread disconnected");
                        return Ok(());
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        })
        .expect("Failed to spawn sample thread");

    const RECORD_BUFFER_SIZE: usize = 1024 * 1024;
    let mut record_buffer: Vec<[u8; 2]> = Vec::with_capacity(RECORD_BUFFER_SIZE);

    loop {
        match samples_receiver.try_recv() {
            Ok(buffer) => buffer.chunks_exact(2).for_each(|iq: &[u8]| {
                record_buffer.push([iq[0], iq[1]]);
            }),
            Err(TryRecvError::Disconnected) => {
                println!("Sample thread disconnected");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
        // signal processing with record buffer in the loop goes right here after
        // match samples_receiver.try_recv() in realtime

        //also you can wait for the buffer to fill and do processing outside the loop
        if record_buffer.len() >= RECORD_BUFFER_SIZE {
            break;
        }
    }

    println!("Shutting down sample thread");

    if let Err(e) = exit_sender.send(()) {
        println!("Failed to send exit event (receiver disconnected): {}", e);
    }

    sample_thread
        .join()
        .expect("Failed to join sample thread")
        .expect("Sample thread returned an error");

    println!("Done");
}
