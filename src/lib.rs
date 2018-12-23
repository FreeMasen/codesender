extern crate rppal;

use rppal::gpio::{Gpio, Level, Error};
use std::time::Duration;


const BIG: u64 = 3;
const SYNC: u64 = 31;

pub fn send(code: usize, pin: u8, length: u64) -> Result<(), Error> {
    let bin = format!("{:024b}", code);
    let high_dur = Duration::from_micros(BIG * length);
    let low_dur = Duration::from_micros(length);
    let sync_dur = Duration::from_micros(SYNC * length);
    let io = Gpio::new()?;

    for c in bin.chars() {
        if c == '1' {
            send_(&io, pin, high_dur, low_dur);
        } else {
            send_(&io, pin, low_dur, high_dur);
        }
    }
    send_(&io, pin, low_dur, sync_dur);
    Ok(())
}

fn send_(io: &Gpio, pin: u8, high_dur: Duration, low_dur: Duration) {
    io.write(pin, Level::High);
    wait(high_dur);
    io.write(pin, Level::Low);
    wait(low_dur);
}

pub fn wait(target: Duration) {
    let now = std::time::SystemTime::now();
    while now.elapsed().unwrap() < target {

    }
}
