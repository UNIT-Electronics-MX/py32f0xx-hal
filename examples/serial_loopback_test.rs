#![no_main]
#![no_std]

use core::fmt::Write;

use panic_halt as _;

use py32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    system_init::SystemInit,
};

use cortex_m_rt::entry;
use embedded_hal_02::serial::{Read, Write as OtherWrite};

#[entry]
fn main() -> ! {
    // Inicialización del sistema usando nuestro módulo común
    let system = SystemInit::init_24mhz(); // 24MHz para comunicación serial estable
    
    // Obtener periféricos
    let p = pac::Peripherals::take().unwrap();

    // Configuración de pines para USART2 (CONFIGURACIÓN SOLICITADA)
    // PA0 como TX con AF9 para USART2
    // PA1 como RX con AF9 para USART2
    let tx = system.gpioa.pa0.into_alternate_af9();
    let rx = system.gpioa.pa1.into_alternate_af9();

    // FORZAR configuración AFR por si la HAL no mapea bien AF9:
    let gpioa = &p.GPIOA;
    gpioa.moder.modify(|_, w| w.mode0().alternate().mode1().alternate());
    gpioa.afrl.modify(|_, w| { 
        w.afsel0().bits(9)  // PA0 -> AF9 (USART2_TX)
         .afsel1().bits(9)  // PA1 -> AF9 (USART2_RX)
    });
    gpioa.pupdr.modify(|_, w| w.pupd1().pull_up()); // RX con pull-up

    // Forzar habilitación USART2 por si la HAL no lo hace bien:
    p.RCC.apbenr1.modify(|_, w| w.usart2en().set_bit());
    p.RCC.apbrstr1.modify(|_, w| w.usart2rst().set_bit());
    p.RCC.apbrstr1.modify(|_, w| w.usart2rst().clear_bit());

    // PB5 como pin de debug (salida push-pull) - disponible en DFN8
    let mut debug_pin = system.gpiob.pb5.into_push_pull_output();
    
    // Inicializar debug pin en LOW
    debug_pin.set_low();

    let mut serial = p.USART2.serial((tx, rx), 115_200.bps(), &system.rcc.clocks);
    
    // PARPADEO INICIAL DE CONFIRMACIÓN
    for i in 0..5 {
        debug_pin.set_high();
        cortex_m::asm::delay(1_200_000); // ~50ms a 24MHz
        debug_pin.set_low();
        cortex_m::asm::delay(1_200_000); // ~50ms a 24MHz
    }

    // Test básico de funcionamiento
    let test_pattern = b"TEST\r\n";
    let mut buffer = [0u8; 6];
    let mut success_count = 0;
    let mut fail_count = 0;

    // Realizar 10 pruebas de loopback (solo si hay conexión física PA0->PA1)
    for test_num in 0..10 {
        // Enviar patrón de test
        for &byte in test_pattern {
            nb::block!(serial.write(byte)).ok();
        }

        // Intentar leer el patrón devuelto (timeout simple)
        let mut read_success = true;
        for i in 0..6 {
            let mut timeout = 10000;
            loop {
                match serial.read() {
                    Ok(byte) => {
                        buffer[i] = byte;
                        break;
                    },
                    Err(nb::Error::WouldBlock) => {
                        timeout -= 1;
                        if timeout == 0 {
                            read_success = false;
                            break;
                        }
                        cortex_m::asm::delay(240); // ~10µs a 24MHz
                    },
                    Err(_) => {
                        read_success = false;
                        break;
                    }
                }
            }
            if !read_success {
                break;
            }
        }

        // Verificar si el patrón coincide
        if read_success && buffer == *test_pattern {
            success_count += 1;
            // LED rápido para éxito
            debug_pin.set_high();
            cortex_m::asm::delay(240_000); // ~10ms
            debug_pin.set_low();
        } else {
            fail_count += 1;
            // LED lento para fallo
            debug_pin.set_high();
            cortex_m::asm::delay(2_400_000); // ~100ms
            debug_pin.set_low();
        }
        
        // Pausa entre tests
        cortex_m::asm::delay(4_800_000); // ~200ms
    }

    // Mostrar resultados con parpadeos
    // Parpadear éxitos (LED rápido)
    for _ in 0..success_count {
        debug_pin.set_high();
        cortex_m::asm::delay(600_000); // ~25ms
        debug_pin.set_low();
        cortex_m::asm::delay(600_000); // ~25ms
    }
    
    cortex_m::asm::delay(4_800_000); // Pausa larga
    
    // Parpadear fallos (LED lento)
    for _ in 0..fail_count {
        debug_pin.set_high();
        cortex_m::asm::delay(2_400_000); // ~100ms
        debug_pin.set_low();
        cortex_m::asm::delay(2_400_000); // ~100ms
    }

    // Loop principal: Si hay éxitos, parpadeo continuo rápido
    // Si solo hay fallos, parpadeo continuo lento
    loop {
        if success_count > 0 {
            // Parpadeo rápido = USART2 funciona (con conexión PA0->PA1)
            debug_pin.set_high();
            cortex_m::asm::delay(1_200_000); // ~50ms
            debug_pin.set_low();
            cortex_m::asm::delay(1_200_000); // ~50ms
        } else {
            // Parpadeo lento = USART2 configurado pero sin loopback físico
            debug_pin.set_high();
            cortex_m::asm::delay(4_800_000); // ~200ms
            debug_pin.set_low();
            cortex_m::asm::delay(4_800_000); // ~200ms
        }
    }
}
