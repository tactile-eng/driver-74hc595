#![no_std]
#![warn(missing_docs)]

//! An embedded async driver for 74hc595 (compatible) shift registers.
//!
//! For efficient data transfer, an SPI bus peripheral is used. The SPI bus should be configured in SPI Mode 0 (clock
//! idle state low, data sampled on rising edge). Because the 74hc595 does not have a CS line, the SPI bus cannot be
//! shared with other devices (at least not without additional circuitry).

use embedded_hal::digital::OutputPin;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiBus;

/// A 74hc595 (compatible) shift register.
pub struct ShiftRegister<SPI, OE, CLR, LATCH, D> {
    spi: SPI,
    not_oe: OE,
    not_clr: CLR,
    latch: LATCH,
    delay: D,
}

impl<SPI, OE, CLR, LATCH, D> ShiftRegister<SPI, OE, CLR, LATCH, D>
where
    SPI: SpiBus,
    OE: OutputPin,
    CLR: OutputPin,
    LATCH: OutputPin,
    D: DelayNs,
{
    /// Creates a new `ShiftRegister`.
    pub fn new(spi: SPI, not_oe: OE, not_clr: CLR, latch: LATCH, delay: D) -> Self {
        ShiftRegister {
            spi,
            not_oe,
            not_clr,
            latch,
            delay,
        }
    }

    /// Enables or disables the shift register output drivers.
    pub fn output_enable(&mut self, enable: bool) -> Result<(), OE::Error> {
        self.not_oe.set_state((!enable).into())
    }

    /// Clears the shift register.
    ///
    /// This does not change the value of the output latches. Use [`Self::latch()`]
    /// after clearing if you want the outputs to be reset.
    pub async fn clear(&mut self) -> Result<(), CLR::Error> {
        self.not_clr.set_low()?;
        self.delay.delay_us(1).await;
        self.not_clr.set_high()
    }

    /// Latches the current values in the shift register into the output latches.
    pub async fn latch(&mut self) -> Result<(), LATCH::Error> {
        self.latch.set_high()?;
        self.delay.delay_us(1).await;
        self.latch.set_low()
    }

    /// Loads `data` into the shift register.
    pub async fn load(&mut self, data: &[u8]) -> Result<(), SPI::Error> {
        self.spi.write(data).await
    }

    /// Loads `write` into the shift register and writes the data shifted out of the shift register
    /// into `read`.
    pub async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), SPI::Error> {
        self.spi.transfer(read, write).await
    }
}
