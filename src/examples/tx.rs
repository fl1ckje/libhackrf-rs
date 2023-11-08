use libhackrf::{HackRF, Off, Tx, MAX_TRANSMISSION_UNIT};
use rand::{distributions::Uniform, Rng};
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
    txvga_gain: u16,
    amp: bool,
    bias_tee: u8,
}

fn main() {
    transmit(Args {
        fs: 10_000_000,
        fc: 87_600_000,
        filter_bw: 4_000_000,
        lna_gain: 40,
        txvga_gain: 47,
        amp: true,
        bias_tee: 0,
    });
}

fn transmit(args: Args) {
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
        .set_txvga_gain(args.txvga_gain)
        .expect("Failed to set VGA gain");

    let mut hackrf: HackRF<Tx> = hackrf.into_tx_mode().expect("Failed to enter TX mode");
    let (exit_tx, exit_rx) = channel();

    let sample_thread: thread::JoinHandle<Result<(), libhackrf::Error>> =
        thread::spawn(move || -> Result<(), libhackrf::Error> {
            let range: Uniform<u8> = Uniform::from(0..255);

            println!("Spawned sample thread");

            loop {
                hackrf.tx(rand::thread_rng()
                    .sample_iter(&range)
                    .take(MAX_TRANSMISSION_UNIT)
                    .collect())?;

                match exit_rx.try_recv() {
                    Ok(_) => {
                        hackrf.stop_tx()?;
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

    for i in 1..=5 {
        thread::sleep(Duration::from_secs(1));
        println!("TX time: {} s.", i);
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
