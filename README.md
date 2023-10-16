# Getting started with the Nordic nRF52840 board in rust.

## Linux Instalation instructions.

Install Rust (rustc, cargo, rustup)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install `probe-run`

```
cargo install probe-run
```

```
rustup target add thumbv7em-none-eabihf
```

Add UDEV rules (to a new file in `/etc/udev/rules.d/`)

```
ATTRS{idVendor}=="1366", ENV{ID_MM_DEVICE_IGNORE}="1", TAG+="uaccess"
```

Plug in the board and switch it on.

```
cargo run
```

## Getting the boards

I've been using the NRF52840-DK board (about 40-50 euros)

https://www.digikey.co.uk/en/products/detail/nordic-semiconductor-asa/NRF52840-DK/8593726

But there is a cheaper version here (about 10 euros)

https://www.digikey.co.uk/en/products/detail/nordic-semiconductor-asa/NRF52840-DONGLE/9491124

Other vendors are available!

## Course outline

[Course Outline](COURSE-OUTLINE.md)

## Documentation essential reading.

[Course Outline](COURSE-OUTLINE.md)
