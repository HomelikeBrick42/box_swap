# box_swap

[![Latest Version](https://img.shields.io/crates/v/box_swap.svg)](https://crates.io/crates/box_swap)
[![Rust Documentation](https://docs.rs/box_swap/badge.svg)](https://docs.rs/box_swap)
![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)

An atomic verison of `Option<Box<T>>`.

One use case for this is being able to have one thread send many updates to another, but the other thread only cares about looking at the latest value.

```rust, no_run
use box_swap::BoxSwap;

struct Data {}

let value: BoxSwap<Data> = BoxSwap::empty();
std::thread::scope(|s| {
    s.spawn(|| {
        loop {
            if let Some(new_value) = value.take() {
                // update the value, maybe if this was a renderer you would upload the value to the gpu
                // if multiple values were "sent" before this happens, it doesnt matter, `new_value` is the most up-to-date value
            }

            // do whatever
        }
    });

    loop {
        // do whatever computation, maybe this could be a game loop

        value.store(Box::new(Data {})); // send the data to the other thread
    }
});
```
