# rpi-zero-usb-iso

## Setup

* Use [Imager](https://www.raspberrypi.com/documentation/computers/getting-started.html#raspberry-pi-imager) to burn Rasperry Pi OS LITE, with SSH access, to a MicroSD card.
* Start your Raspberry with the MicroSD card.
* Connect to the Raspberry with `ssh`.
* `mkdir -p .local/bin`
* Copy `rpi-zero-usb-iso` to `.local/bin`.
* `rpi-zero-usb-iso setup`
* Copy at least one ISO to the ISOs directory.
* Reboot if prompted.

## Usage

Copy ISOs to the ISOs directory.

`rpi-zero-usb-iso` automatically uses the last modified ISO on boot.

## Hacking

### Setup

(Tested on Debian 12)

`~/.cargo/config.toml`:

```
[target.aarch64-unknown-linux-musl]
linker = "/usr/bin/aarch64-linux-gnu-gcc-12"
```

```
$ sudo apt install gcc-12-aarch64-linux-gnu
$ rustup target add aarch64-unknown-linux-musl
```

### Building and running

```
$ cargo build --target=aarch64-unknown-linux-musl && scp target/aarch64-unknown-linux-musl/debug/rpi-zero-usb-iso x: && ssh x ./rpi-zero-usb-iso ...
```
