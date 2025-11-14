# I2C General Call Driver
A simple `#[no_std]` platform-agnostic driver for issuing I2C general calls according to spec.

## Usage
```rust,ignore
let mut driver = i2c_general_call::GeneralCall::new(i2c_bus_instance);

// Issue a latch address general call command, which instructs devices to latch their address based on current hardware state.
driver.latch_addr().expect("No devices on the bus support the latch address general call command.");

// Issue a reset general call command, which instructs devices to reset registers to power-on state.
// This also instructs devices to latch their address based on current hardware state.
driver.reset().expect("No devices on the bus support the reset general call command.");
```

Async I2C transactions are also supported by enabling the `async` feature and adding `.await` to
calls in the above usage example.

## License
Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).