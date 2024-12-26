### Development

This project has been fully configured for VSCode, but it does require a
Raspberry Pi Debug Probe.

Simply choose "Run > Start Debugging". It will build the source, clear the
flash, push the ut2 binary to the pico, and boot it. Output will appear in
the debug console, including any panic messages that might halt your program.

### Building for production

First boot your pico in usb drive mode by plugging it in while holding the
bootsel button.

```
cargo run --release
cp target/thumbv6m-none-eabi/release/embassy-rp-blinky.uf2 /Volumes/RPI-RP2
```
