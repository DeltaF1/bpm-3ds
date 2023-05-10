use std::time::{Duration, Instant};

mod circular;

use circular::CircularBuffer;
use ctru::prelude::*;

fn main() {
    let apt = Apt::new().expect("Couldn't get APT");

    let mut hid = Hid::new().expect("Couldn't get HID system");

    let gfx = Gfx::new().expect("Couldn't get GFX system");
    let _console = Console::new(gfx.bottom_screen.borrow_mut());

    let mut now = Instant::now();

    let mut samples: CircularBuffer<std::time::Duration, 8> = CircularBuffer::new();
    while apt.main_loop() {
        hid.scan_input();
        gfx.wait_for_vblank();

        if hid
            .keys_down()
            .intersects(KeyPad::A | KeyPad::B | KeyPad::X | KeyPad::Y | KeyPad::TOUCH)
        {
            let new_now = Instant::now();
            let diff = new_now - now;
            samples.add(diff);
            let average = samples.into_iter().copied().sum::<Duration>() / (samples.len() as u32);
            let freq: f32 = average.as_secs_f32().recip();
            if freq.is_normal() {
                let bpm = freq * 60f32;
                println!("{diff:?}; {bpm}");
            } else {
                println!("{diff:?}; Not enough data")
            }

            now = new_now;
        } else if hid.keys_down().contains(KeyPad::START) {
            break;
        }
    }
}
