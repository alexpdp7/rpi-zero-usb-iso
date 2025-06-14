# rpi-zero-usb-iso

Tasks such as installing Linux frequently require creating USB bootable drives.
This requires wiping out a USB drive for each bootable drive you want to create.
Alternatives such as [Ventoy](https://www.ventoy.net/) tackle this problem.

rpi-zero-usb-iso provides an alternative: use the Linux Mass Storage Gadget feature on a Raspberry Pi to expose ISO files in a directory as USB bootable drives.

## Requirements

A Raspberry Pi that can run connected to a USB port of a computer.

I have tested this with:

* Raspberry Pi Zero 2 W
  * Raspberry Pi OS Lite Bookworm
  * Installed with [Raspberry Pi Imager](https://www.raspberrypi.com/documentation/computers/getting-started.html#raspberry-pi-imager) (with network configuration to access via SSH)
  * An [EP-0097](https://wiki.52pi.com/index.php/EP-0097) so the Pi can be powered safely from the USB port

See [this post on the Raspberry Pi forums for details on other systems that might work and caveats](https://forums.raspberrypi.com/viewtopic.php?t=388185#p2316711).

## Setup

```
mkdir -p ~/.local/bin
wget https://github.com/alexpdp7/rpi-zero-usb-iso/releases/latest/download/rpi-zero-usb-iso -O ~/.local/bin/rpi-zero-usb-iso
chmod +x ~/.local/bin/rpi-zero-usb-iso
rpi-zero-usb-iso setup
```

Copy at least one ISO to the ISOs directory, then reboot the Raspberry if prompted by `rpi-zero-usb-iso setup`.

## Usage

Copy ISOs to the ISOs directory.

`rpi-zero-usb-iso` automatically uses the last modified ISO on boot.
Use `touch` to "select" an ISO.

### Windows

Windows installer ISOs only support their use as optical media.
Use <https://github.com/alexpdp7/windows-usb/> to convert ISOs downloaded from Microsoft to USB images that rpi-zero-usb-iso can use.

### Tested media

* Completed a full install
  * Windows 11 24H2 (using windows-usb to convert optical media)
  * ChromeOS Flex `16002.51.0_reven_recovery_stable-channel_mp-v6`
  * AlmaLinux 10
  * Pop!_OS 22.04 LTS
* Only smoke tested installer start
  * Proxmox VE 8.4
  * Debian 12.11.0
  * Ubuntu 22.04.5
  * Ubuntu 24.04.2

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

### Contributing

Patches are welcome, but I can only test on my hardware.
Happy if others add support for more hardware, but I can only recommend using the hardware that has been tested.
