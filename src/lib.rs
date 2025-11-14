//! I2C General Call Driver
//! TODO
#![no_std]

use embedded_hal::i2c;
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

// General call (aka broadcast) address
const GENERAL_CALL_ADDR: u8 = 0x00;

// Only two specified commands
enum Command {
    Reset,
    LatchAddr,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Reset => 0x06,
            Command::LatchAddr => 0x04,
        }
    }
}

/// I2C general call error.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// No device on the bus acknowledged the general call.
    NoAckCall,
    /// At least one device on the bus acknowledged the general call, but not the specific command.
    NoAckCmd,
    /// Other I2C error was encountered.
    I2C(E),
}

/// I2C general call driver.
#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "GeneralCall",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
pub struct GeneralCall<I2C: AsyncI2c> {
    i2c: I2C,
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "GeneralCall",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]
impl<E: i2c::Error, I2C: AsyncI2c<Error = E>> GeneralCall<I2C> {
    fn res_map(res: Result<(), E>) -> Result<(), Error<E>> {
        res.map_err(|e| match e.kind() {
            i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Address) => Error::NoAckCall,
            i2c::ErrorKind::NoAcknowledge(i2c::NoAcknowledgeSource::Data) => Error::NoAckCmd,
            _ => Error::I2C(e),
        })
    }

    /// Create a new instance of an I2C general call driver.
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Destroy this driver instance and return the underlying I2C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Issue a reset command general call, which instructs devices on the bus to latch their addresses
    /// and reset their registers to the default state.
    ///
    /// If successful, then at least one device on the bus ACKed the reset command.
    /// However, there is no guarantee that it actually performed a reset. One should verify
    /// the device actually honors reset general calls with the datasheet.
    ///
    /// # Errors
    ///
    /// If no device accepts general calls, [`Error::NoAckCall`] will be returned.
    ///
    /// If at least one device accepts general calls but not a reset command, [`Error::NoAckCmd`] will be returned.
    ///
    /// If any other I2C error occurs, the underlying error will be returned.
    pub async fn reset(&mut self) -> Result<(), Error<E>> {
        let res = self
            .i2c
            .write(GENERAL_CALL_ADDR, &[Command::Reset.into()])
            .await;
        Self::res_map(res)
    }

    /// Issue a address latch command general call, which instructs devices on the bus to latch their addresses
    /// but NOT perform a full reset.
    ///
    /// If successful, then at least one device on the bus ACKed the address latch command.
    /// However, there is no guarantee that it actually performed a latch. One should verify
    /// the device actually honors address latch general calls with the datasheet.
    ///
    /// # Errors
    ///
    /// If no device accepts general calls, [`Error::NoAckCall`] will be returned.
    ///
    /// If at least one device accepts general calls but not an address latch command,
    /// [`Error::NoAckCmd`] will be returned.
    ///
    /// If any other I2C error occurs, the underlying error will be returned.
    pub async fn latch_addr(&mut self) -> Result<(), Error<E>> {
        let res = self
            .i2c
            .write(GENERAL_CALL_ADDR, &[Command::LatchAddr.into()])
            .await;
        Self::res_map(res)
    }

    /// Issue an arbitrary command general call. The command code must be nonzero as that is
    /// forbidden by spec.
    ///
    /// If successful, then at least one device on the bus ACKed the command.
    /// However, there is no guarantee that it actually performed the command. One should verify
    /// the device actually honors the command with the datasheet.
    ///
    /// # Errors
    ///
    /// If no device accepts general calls, [`Error::NoAckCall`] will be returned.
    ///
    /// If at least one device accepts general calls but not the command,
    /// [`Error::NoAckCmd`] will be returned.
    ///
    /// If any other I2C error occurs, the underlying error will be returned.
    pub async fn call(&mut self, cmd: core::num::NonZeroU8) -> Result<(), Error<E>> {
        let res = self.i2c.write(GENERAL_CALL_ADDR, &[cmd.get()]).await;
        Self::res_map(res)
    }
}
