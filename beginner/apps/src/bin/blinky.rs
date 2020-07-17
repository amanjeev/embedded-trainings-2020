#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
use panic_log as _; // panic handler

#[entry]
fn main() -> ! {
    // uncomment to enable more verbose logs
    // log::set_max_level(log::LevelFilter::Trace);

    let board = dk::init().unwrap();

    let mut led1 = board.leds._1;
    let mut led2 = board.leds._2;
    let mut led3 = board.leds._3;
    let mut led4 = board.leds._4;
    
    let mut timer = board.timer;

    led1.on();
    led2.off();
    led3.on();
    led4.off();
    for _ in 0..10 {
        led1.toggle();
        led2.toggle();
        led3.toggle();
        led4.toggle();
        timer.wait(Duration::from_secs(1));
        log::info!("LED toggled at {:?}", dk::uptime());
    }

    dk::exit()
}
