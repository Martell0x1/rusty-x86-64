use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

pub enum SerialColor{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
}
impl SerialColor {
    pub fn fg_code(self) -> u8 {
        self as u8 + 30
    }
    pub fn bg_code(self) -> u8 {
        self as u8 + 40
    }
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn set_serial_color(foreground: SerialColor, background: SerialColor) {
    use core::fmt::Write;
    let mut serial = SERIAL1.lock();
    write!(
        serial,
        "\x1B[{};{}m",
        foreground.fg_code(),
        background.bg_code()
    )
    .unwrap();
}
pub fn reset_serial_color() {
    use core::fmt::Write;
    let mut serial = SERIAL1.lock();
    write!(serial, "\x1B[0m").unwrap();
}
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments){
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
         SERIAL1.lock().write_fmt(args).expect("Printing to serial failed"); 
    });

}