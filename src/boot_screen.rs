use crate::vga_buffer::{Color, WRITER};
use crate::{print, println};
use x86_64::instructions::hlt;

/// Simple delay loop
fn delay(count: u64) {
    for _ in 0..count {
        hlt();
    }
}

pub fn show() {
    // WRITER.lock().clear_screen(); // clear screen first
    print_logo();

    // Animate each progress bar
    animate_progress("Initializing Kernel", Color::LightCyan, 30);
    animate_progress("Setting up memory", Color::LightGreen, 50);
    animate_progress("Loading interrupts", Color::LightRed, 80);
    animate_progress("Boot complete", Color::LightBlue, 100);
}
/// Print the ASCII Rust OS logo using macros
fn print_logo() {
     let logo = [
         "                                                          ",
         " ____            _                ___   __      __   _  _   ",
         "|  _ \\ _   _ ___| |_ _   _  __  _( _ ) / /_    / /_ | || |  ",
         "| |_) | | | / __| __| | | | \\ \\/ / _ \\| '_ \\  | '_ \\| || |_ ",
         "|  _ <| |_| \\__ \\ |_| |_| |  >  < (_) | (_) | | (_) |__   _|",
         "|_| \\_\\__,_|___/\\__|\\__, | /_/\\_\\___/ \\___/___\\___/   |_|  ",
         "                     |___/                |_____|           "
    ];

    for line in logo.iter() {
        println!("{}", line); // use your macro
    }
    println!("Starting Up....");
}

fn animate_progress(task: &str, color: Color, target_percent: u8) {
    let mut writer = WRITER.lock();
    writer.set_color(color, Color::Black);

    // Write task first
    for &b in task.as_bytes() {
        writer.write_byte(b);
    }
    writer.write_byte(b' ');

    for percent in 0..=target_percent {
        // Move cursor back to start of bar
        let start_pos = task.len() + 1;

        // Draw bar
        writer.set_column(start_pos);
        writer.write_byte(b'[');
        let filled = (percent / 10) as usize;
        for i in 0..10 {
            if i < filled {
                writer.write_byte(b'#');
            } else {
                writer.write_byte(b' ');
            }
        }
        writer.write_byte(b']');

        // Write percent after bar
        writer.write_byte(b' ');
        if percent >= 10 {
            writer.write_byte(b'0' + (percent / 10));
        } else {
            writer.write_byte(b' ');
        }
        writer.write_byte(b'0' + (percent % 10));
        writer.write_byte(b'%');

        delay(50_000);
    }

    writer.write_byte(b'\n'); // finish line
}


/// Print a single progress bar using stack array
fn print_progress(task: &str, percent: u8, color: Color) {
    let mut bar = [b' '; 12]; // [ '[', 10 chars, ']' ]
    bar[0] = b'[';
    let filled = (percent / 10) as usize;
    for i in 0..filled {
        bar[i + 1] = b'#';
    }
    for i in filled + 1..11 {
        bar[i] = b' ';
    }
    bar[11] = b']';

    let bar_str = core::str::from_utf8(&bar).unwrap();

    crate::vga_buffer::_print(color, format_args!("{} {}% {}\n", task, percent, bar_str));
}