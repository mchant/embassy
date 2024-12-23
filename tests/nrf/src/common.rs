#![macro_use]

use {defmt_rtt as _, panic_probe as _};

#[cfg(feature = "nrf52832")]
teleprobe_meta::target!(b"nrf52832-dk");
#[cfg(feature = "nrf52840")]
teleprobe_meta::target!(b"nrf52840-dk");
#[cfg(feature = "nrf52833")]
teleprobe_meta::target!(b"nrf52833-dk");
#[cfg(feature = "nrf5340")]
teleprobe_meta::target!(b"nrf5340-dk");
#[cfg(feature = "nrf9160")]
teleprobe_meta::target!(b"nrf9160-dk");
#[cfg(feature = "nrf51422")]
teleprobe_meta::target!(b"nrf51-dk");

macro_rules! define_peris {
    ($($name:ident = $peri:ident,)* $(@irq $irq_name:ident = $irq_code:tt,)*) => {
        #[allow(unused_macros)]
        macro_rules! peri {
            $(
                ($p:expr, $name) => {
                    $p.$peri
                };
            )*
        }
        #[allow(unused_macros)]
        macro_rules! irqs {
            $(
                ($irq_name) => {{
                    embassy_nrf::bind_interrupts!(struct Irqs $irq_code);
                    Irqs
                }};
            )*
            ( @ dummy ) => {};
        }

        #[allow(unused)]
        #[allow(non_camel_case_types)]
        pub mod peris {
            $(
                pub type $name = embassy_nrf::peripherals::$peri;
            )*
        }
    };
}

#[cfg(feature = "nrf51422")]
define_peris!(PIN_A = P0_13, PIN_B = P0_14,);

#[cfg(feature = "nrf52832")]
define_peris!(
    PIN_A = P0_11, PIN_B = P0_12,
    PIN_X = P0_13,
    UART0 = UARTE0,
    SPIM0 = TWISPI0,
    @irq UART0 = {UARTE0 => uarte::InterruptHandler<peripherals::UARTE0>;},
    @irq UART0_BUFFERED = {UARTE0 => buffered_uarte::InterruptHandler<peripherals::UARTE0>;},
    @irq SPIM0 = {TWISPI0 => spim::InterruptHandler<peripherals::TWISPI0>;},
);

#[cfg(feature = "nrf52833")]
define_peris!(
    PIN_A = P1_01, PIN_B = P1_02,
    PIN_X = P1_03,
    UART0 = UARTE0,
    UART1 = UARTE1,
    SPIM0 = TWISPI0,
    @irq UART0 = {UARTE0 => uarte::InterruptHandler<peripherals::UARTE0>;},
    @irq UART1 = {UARTE1 => uarte::InterruptHandler<peripherals::UARTE1>;},
    @irq UART0_BUFFERED = {UARTE0 => buffered_uarte::InterruptHandler<peripherals::UARTE0>;},
    @irq UART1_BUFFERED = {UARTE1 => buffered_uarte::InterruptHandler<peripherals::UARTE1>;},
    @irq SPIM0 = {TWISPI0 => spim::InterruptHandler<peripherals::TWISPI0>;},
);

#[cfg(feature = "nrf52840")]
define_peris!(
    PIN_A = P1_02, PIN_B = P1_03,
    PIN_X = P1_04,
    UART0 = UARTE0,
    UART1 = UARTE1,
    SPIM0 = TWISPI0,
    @irq UART0 = {UARTE0 => uarte::InterruptHandler<peripherals::UARTE0>;},
    @irq UART1 = {UARTE1 => uarte::InterruptHandler<peripherals::UARTE1>;},
    @irq UART0_BUFFERED = {UARTE0 => buffered_uarte::InterruptHandler<peripherals::UARTE0>;},
    @irq UART1_BUFFERED = {UARTE1 => buffered_uarte::InterruptHandler<peripherals::UARTE1>;},
    @irq SPIM0 = {TWISPI0 => spim::InterruptHandler<peripherals::TWISPI0>;},
);

#[cfg(feature = "nrf5340")]
define_peris!(
    PIN_A = P1_08, PIN_B = P1_09,
    PIN_X = P1_10,
    UART0 = SERIAL0,
    UART1 = SERIAL1,
    SPIM0 = SERIAL0,
    @irq UART0 = {SERIAL0 => uarte::InterruptHandler<peripherals::SERIAL0>;},
    @irq UART1 = {SERIAL1 => uarte::InterruptHandler<peripherals::SERIAL1>;},
    @irq UART0_BUFFERED = {SERIAL0 => buffered_uarte::InterruptHandler<peripherals::SERIAL0>;},
    @irq UART1_BUFFERED = {SERIAL1 => buffered_uarte::InterruptHandler<peripherals::SERIAL1>;},
    @irq SPIM0 = {SERIAL0 => spim::InterruptHandler<peripherals::SERIAL0>;},
);

#[cfg(feature = "nrf9160")]
define_peris!(
    PIN_A = P0_00, PIN_B = P0_01,
    PIN_X = P0_02,
    UART0 = SERIAL0,
    UART1 = SERIAL1,
    SPIM0 = SERIAL0,
    @irq UART0 = {SERIAL0 => uarte::InterruptHandler<peripherals::SERIAL0>;},
    @irq UART1 = {SERIAL1 => uarte::InterruptHandler<peripherals::SERIAL1>;},
    @irq UART0_BUFFERED = {SERIAL0 => buffered_uarte::InterruptHandler<peripherals::SERIAL0>;},
    @irq UART1_BUFFERED = {SERIAL1 => buffered_uarte::InterruptHandler<peripherals::SERIAL1>;},
    @irq SPIM0 = {SERIAL0 => spim::InterruptHandler<peripherals::SERIAL0>;},
);
