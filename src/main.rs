#![no_main]
#![no_std]

use core::{fmt::Write, ffi::c_int};
use cortex_m_rt::entry;
use microbit::{
    hal::uart::{Uart, Baudrate, Parity},
    Board,
};

extern crate panic_halt;
extern crate microbit;
extern "C" {
     fn c_add_one(x: c_int) -> c_int;
}

pub fn add_one(x: i32) -> i32 {
    unsafe {
	c_add_one(x)
    }
}

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut serial = Uart::new(
	board.UART0,
	board.uart.into(),
	Parity::EXCLUDED,
	Baudrate::BAUD115200,
    );

    let ten_plus_one = add_one(10);
    writeln!(serial, "10 + 1 = {}\r", ten_plus_one).unwrap();

    loop {}
}
