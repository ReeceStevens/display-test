# Display Test Application

This is a minimal test case for the RA8875 display driver.

This example is implemented on an STM32F4 discovery board.

## Getting Started

In one terminal, kick off openocd:

```
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
```

In another, build and run the application with `cargo run`.
