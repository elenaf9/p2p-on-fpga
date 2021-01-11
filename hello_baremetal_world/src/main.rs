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
        *(0xf1030000 as *mut u32) = mask << 19; // set bit 19 
    }
}

#[entry]
fn main() -> ! {
    let mut mask = 1;
    unsafe {
        *(0xf1030008 as *mut u32) = 0; // no alternate functions
        *(0xf103000c as *mut u32) = 0; // set all bits to out
        *(0xf1030004 as *mut u32) = !(1<<19); // set bit 19 to out
    }
    loop {
        set_leds(mask);
         mask = if mask == 0 {
            1
        } else { 0 };
        delay(200000);
    }
}
