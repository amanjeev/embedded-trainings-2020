#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// in the exercise we comment this line out 
// and write our own panic handler below
//use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    dk::init().unwrap();

    foo();

    loop {
        asm::bkpt();
    }
}

#[inline(never)]
fn foo() {
    asm::nop();
    bar();
}

#[inline(never)]
fn bar() {
    let i = index();
    let array = [0, 1, 2];
    let x = array[i]; // out of bounds access

    log::info!("{}", x);
}

fn index() -> usize {
    3
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("LOL ;;;;; {}", info);
    loop {
        asm::bkpt()
    }
}