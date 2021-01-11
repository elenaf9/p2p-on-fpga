// source https://craigjb.com/2020/01/22/ecp5/
#![feature(asm)]
#![no_std]
#![no_main]

extern crate panic_halt;
extern crate riscv_rt;

use riscv_rt::entry;

fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop")
        }
    }
}

fn set_leds(mask: u32) {
    unsafe {
        *(0xF1030013 as *mut u32) = mask;
    }
}

#[entry]
fn main() -> ! {
    let mut mask = 0x40;
    loop {
        set_leds(mask);
        mask >>= 1;
        if mask == 0 {
            mask = 0x40;
        }
        delay(300000);
    }
}

