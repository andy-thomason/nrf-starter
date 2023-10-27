# Course outline - Rust for embedded systems.

## Reading material:

General introduction to Rust

https://doc.rust-lang.org/rust-by-example/

Unsafe Rust

https://doc.rust-lang.org/nomicon/

Cargo and config

https://doc.rust-lang.org/cargo/

Embedded Rust

https://docs.rust-embedded.org/book/intro/no-std.html


## Session 1:

* Rust types - primitives, structs, enums, arrays, str, #[derive(Debug)]
* References - immutable/mutable, slices, rules for references.
* Smart pointers - Box, Rc, Arc.
* Collections Vec, HashMap, BTreeMap.
* Basic control flow - if, match, loop, for, while.
* Implementing methods.
* Result and Option.
* Exercise: Write FizzBuzz in Rust. (https://doc.rust-lang.org/rust-by-example/flow_control/match.html)

## Session 2:

* Functions and return values
* Move semantics
* Closures
* Generic functions and Structures.
* Traits and bounds.
* Result and Option.
* Dynamic traits.
* Exercise: Make a generic Animal trait and apply it to cats and dogs.

## Session 3

* Destructuring assignments.
* Lifetimes.
* Iterators.
* If-let, if-else, while-let.
* Drop trait.
* Deref trait.
* Exercise: Parse a command.

## Session 4

* `#[no-std]` coding for embedded.
* Unsafe rust and pointers.
* Accessing hardware memory locations.
* Calling and being called by C code.
* Bindgen - generating interfaces to C code from headers.
* Logging and tracing. (https://docs.rs/log/latest/log/)
* Exercise: Write a static Rust library and call it from C.

## Session 5

* Atomic, Cell, RefCell and Mutex.
* Interrupt handling.
* Async & Future.
* Concurrency.
* Synchonisation.
* Exercise: Run the qemu example from embedded rust book: https://docs.rust-embedded.org/book/start/qemu.html
