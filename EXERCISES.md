# Exercises

## FizzBuzz

Fizzbuzz is a game where each player calls out either:

A number
"Fizz" if the number is divisible by three.
"Buzz" if the number is divisible by five.
"FizzBuzz" if the number is divisible by three and five.

For example:

1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz

The objective of this task is to write a function like this:

```
fn fizzbuzz(i: i32) -> String {

}
```

You will probably want to use a match statement, but also try
with if-then-else.

Hint: You can match on two expressions in a tuple:

```
match (i % 3, i % 5) {

}
```

To make the loop to call the function, use for-range.
The loop below iterates over all numbers between 1 and 100 *inclusive*.

```
   for i in 1..=100 {
      ...
   }
```

## Animal

This is a trait building exercise:

Say we want to define an animal with properties:

```
trait Animal {
    fn can_fly(&self) -> bool;

    fn legs(&self) -> i32;

    fn bark(&mut self);

    fn color(&self) -> &'static str;
}
```

We can apply this to some structs:

```
struct Cat {
    color: String,
}

impl Animal for Cat {
    ...
}

struct Parrot {

}

impl Animal for Parrot {
    ...
}

```

Write some tests to test you impls. Remember to
use `#[derive(Debug, PartialEq)]` on your structs.

```
#[test]
fn test_cat() {
    let felix = Cat { color: "tabby".to_string() };

    assert_eq!(felix.color(), "tabby");
}
```

## Parse a command

Say our command is:

```
enum Command {
    FirePhotonTorpedoes { at: String },
    FightAlienMonster { with_shirt_on: bool },
    NumberOfTribbles(u128),
}
```

We can split a string like:

"fire photon torpedoes at kilingons"

Into tokens using `split_ascii_whitespace`

https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace

We would assemble these into a tuple:

```
   let mut iter = command.split_ascii_whitespace();

   match (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next()) {

   }
```

As next() returns an Option, what should we put in the match statement to
detect the string above?

Complete the match for all the three cases in `Command`.

Write a test to cover all the cases (and failures!)

Advanced: Replace String with &str.

## Write a static Rust library and call it from C.

We can instruct Rust to create a C static library.
Make a new project:

```
cargo new --lib myproject
cd myproject
code .
```

Edit lib.rs

```
#![crate_type = "staticlib"]

#[no_mangle]
fn my_c_func() -> i32 {
    1
}
```

Compile the rust code:

```
cargo build
```

Create a C main:

```
#include<stdio.h>

extern int my_c_func();

int main() {
    printf("my_c_func returns %d", my_c_func());
}
```

This calls the C code using an unmagled function name.

Compile with GCC

```
gcc src/main.c -o mymain target/debug/libccall.a 
```

(On some projects you may need to add -lpthread -ldl)

Run

```
./mymain
```

Try making a function that consumes a C string.

Use `CStr` to wrap the C string.

https://doc.rust-lang.org/stable/core/ffi/struct.CStr.html

Try calling a function in the C code.

You can use `CString` to convert a rust string to a C string.
