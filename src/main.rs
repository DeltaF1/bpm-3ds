use std::time::{Duration, Instant};

mod circular;

use circular::CircularBuffer;
use ctru::prelude::*;

fn main() {
    let apt = Apt::new().expect("Couldn't get APT");

    let mut hid = Hid::new().expect("Couldn't get HID system");

    let gfx = Gfx::new().expect("Couldn't get GFX system");
    {
        let _console = Console::new(gfx.top_screen.borrow_mut());
        println!("Tap any face buttons or the screen in time to a beat");
        println!("Start to exit");
        println!("Select to clear");
    }
    let _console = Console::new(gfx.bottom_screen.borrow_mut());

    let mut now = None;

    let mut samples: CircularBuffer<std::time::Duration, 8> = CircularBuffer::new();
    let mut bpm = 0.;
    while apt.main_loop() {
        hid.scan_input();
        gfx.wait_for_vblank();

        if hid
            .keys_down()
            .intersects(KeyPad::A | KeyPad::B | KeyPad::X | KeyPad::Y | KeyPad::TOUCH)
        {
            let new_now = Instant::now();
            if let Some(old_now) = now {
                let diff = new_now - old_now;
                samples.add(diff);
                let average = samples.into_iter().copied().sum::<Duration>() / (samples.len() as u32);
                let freq: f32 = average.as_secs_f32().recip();
                if freq.is_normal() {
                    bpm = freq * 60f32;
                    println!("{diff:?}; {bpm} BPM");
                } else {
                    println!("{diff:?}; Not enough data")
                }
            }

            now = Some(new_now);
        } else if hid.keys_down().contains(KeyPad::SELECT) {
            bpm = 0.;
            now = None;
            samples = CircularBuffer::new();
        } else if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        // TODO: Make a beep in time with the bpm
        // Gotta make it line up with the taps too though
    }
}
