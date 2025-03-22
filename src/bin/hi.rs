// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Hi, I am new here!                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::gpio::{Input, Pull};
use embassy_time::{Timer, Duration}; 
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let p = embassy_rp::init(Default::default());


    defmt::info!("Echipa \"I/O CTL\" va saluta!\n");

    let mut sw4 = Input::new(p.PIN_2, Pull::Up);  // GP2
    let mut sw5 = Input::new(p.PIN_3, Pull::Up);  // GP3
    let mut sw6 = Input::new(p.PIN_4, Pull::Up);  // GP4
    let mut sw7 = Input::new(p.PIN_5, Pull::Up);  // GP5



    loop {
        sw4.wait_for_low().await;
        defmt::info!("Butonul SW4 a fost apasat");
        while sw4.is_low() {
            Timer::after(Duration::from_millis(1)).await;
        }
    }
}
