# This is a list of essential reads for Rust docuementation

Core documentation
This is re-exported by std, so in server applcations, `core::ptr` is `std::ptr`

https://doc.rust-lang.org/core/

Alloc library
This contains collectons and smart pointers that need a global allocator.

https://doc.rust-lang.org/alloc/

Primitives

https://doc.rust-lang.org/core/index.html#primitives

Result

https://doc.rust-lang.org/core/result/enum.Result.html

Option

https://doc.rust-lang.org/core/option/enum.Option.html

Iterator
And adaptable trait for getting the next element in a sequence.

https://doc.rust-lang.org/core/iter/trait.Iterator.html

Future
Coroutines in Rust
https://doc.rust-lang.org/core/future/trait.Future.html

Fn function trait
https://doc.rust-lang.org/core/ops/trait.Fn.html

Drop trait
https://doc.rust-lang.org/core/ops/trait.Drop.html

Deref trait
https://doc.rust-lang.org/core/ops/trait.Deref.html

## Embedded examples:
Interrupt handling in the NRF HAL.
https://github.com/nrf-rs/nrf-hal/blob/master/examples/rtc-demo/src/main.rs

Interrupt handling using low level primitives.
https://github.com/rust-embedded/cortex-m-rt/blob/master/examples/device.rs



