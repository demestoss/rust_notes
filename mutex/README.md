# Mutex std struct

Discovering Rust interior mutability with UnsafeCell primitive, diving deep into atomics Ordering and implementing Mutex in a naive way using spinlock, to understand why this is bad. Taking a look at `loom` and why it's important and understand that the best way to use atomics is not using them.
