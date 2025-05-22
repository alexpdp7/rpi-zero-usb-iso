# rpi-zero-usb-iso

## Hacking

### Setup

(Tested on Debian 12)

`~/.cargo/config.toml`:

```
[target.aarch64-unknown-linux-gnu]
linker = "/usr/bin/aarch64-linux-gnu-gcc-12"
```

```
$ sudo apt install gcc-12-aarch64-linux-gnu
$ rustup target add aarch64-unknown-linux-gnu
```

### Building and running

```
$ cargo build --target=aarch64-unknown-linux-gnu && scp target/aarch64-unknown-linux-gnu/debug/rpi-zero-usb-iso store: && ssh store ./rpi-zero-usb-iso ...
```
