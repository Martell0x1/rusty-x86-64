use core::fmt::write;
use crate::vga_buffer::{WRITER, Color , _print};

/*
===========================
    VGA BUFFER MACROS
===========================
 */
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::LightGray, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::LightBlue, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! panic_print {
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::Red, format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! exception_print{
    ($($arg:tt)*) => {
        $crate::vga_buffer::_print($crate::vga_buffer::Color::Pink, format_args!($($arg)*));
    };
}

/*
===========================
 SERIAL MACROS
===========================
 */

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    }
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules!  serial_warn {
    ($($arg:tt)*) => {
        let serial_color = $crate::serial::SerialColor::Yellow;
        let background_color = $crate::serial::SerialColor::Black;
        $crate::serial::set_serial_color(serial_color, background_color);
        $crate::serial::_print(format_args!($($arg)*));
        $crate::serial::reset_serial_color();
    };
}
#[macro_export]
macro_rules! serial_panic {
    ($($arg:tt)*) => {
        let serial_color = $crate::serial::SerialColor::Red;
        let background_color = $crate::serial::SerialColor::Black;
        $crate::serial::set_serial_color(serial_color, background_color);
        $crate::serial::_print(format_args!($($arg)*));
        $crate::serial::reset_serial_color();
    };
}
