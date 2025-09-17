#![no_main]
#![no_std]

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    system_init::SystemInit,
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Inicialización del sistema
    let system = SystemInit::init_24mhz();
    
    // Configurar PB5 como LED de debug
    let mut led = system.gpiob.pb5.into_push_pull_output();
    led.set_low();

    // 3 blinks de inicialización (confirmamos que arranca)
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(4_800_000); // ~200ms a 24MHz
        led.set_low();
        cortex_m::asm::delay(4_800_000);
    }

    // Pausa
    cortex_m::asm::delay(12_000_000); // ~500ms

    // PASO 1: Obtener periféricos (1 blink largo si OK)
    let p = pac::Peripherals::take().unwrap();
    led.set_high();
    cortex_m::asm::delay(9_600_000); // 400ms - indica que obtuvimos periféricos
    led.set_low();
    cortex_m::asm::delay(2_400_000);

    // PASO 2: Configurar GPIO PA1 (2 blinks si OK)
    let rx = system.gpioa.pa1.into_alternate_af9();
    for _ in 0..2 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    cortex_m::asm::delay(2_400_000);

    // PASO 3: Configurar GPIO PA2 (3 blinks si OK)
    let tx = system.gpioa.pa2.into_alternate_af4();
    for _ in 0..3 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low();
        cortex_m::asm::delay(1_200_000);
    }
    cortex_m::asm::delay(2_400_000);

    // PASO 4: Intentar configurar USART2 (4 blinks si OK)
    // Aquí es donde probablemente falla
    let _serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    for _ in 0..4 {
        led.set_high();
        cortex_m::asm::delay(1_200_000); // 50ms
        led.set_low();
        cortex_m::asm::delay(1_200_000);
    }

    // Si llegamos aquí, todo funcionó - parpadeo continuo
    loop {
        led.set_high();
        cortex_m::asm::delay(6_000_000); // 250ms
        led.set_low();
        cortex_m::asm::delay(6_000_000); // 250ms
    }
}
