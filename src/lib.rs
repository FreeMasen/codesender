
use rppal::gpio::{Gpio, Level, Error, OutputPin};
use std::time::Duration;


const BIG: u64 = 3;
const SYNC: u64 = 31;

pub fn send(code: usize, pin: u8, length: u64) -> Result<(), Error> {
    let bin = format!("{:024b}", code);
    let high_dur = Duration::from_micros(BIG * length);
    let low_dur = Duration::from_micros(length);
    let sync_dur = Duration::from_micros(SYNC * length);
    let io = Gpio::new()?;
    let mut pin = io.get(pin)?.into_output();

    for c in bin.chars() {
        if c == '1' {
            send_(&mut pin, high_dur, low_dur);
        } else {
            send_(&mut pin, low_dur, high_dur);
        }
    }
    send_(&mut pin, low_dur, sync_dur);
    Ok(())
}

fn send_(pin: &mut OutputPin, high_dur: Duration, low_dur: Duration) {
    pin.write(Level::High);
    wait(high_dur);
    pin.write(Level::Low);
    wait(low_dur);
}

pub fn wait(target: Duration) {
    let now = std::time::SystemTime::now();
    while now.elapsed().unwrap() < target {

    }
}
