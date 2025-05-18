# box_swap

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
