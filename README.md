# Getting started with the Nordic nRF52840 board in rust.

# Instalation instructions.

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

