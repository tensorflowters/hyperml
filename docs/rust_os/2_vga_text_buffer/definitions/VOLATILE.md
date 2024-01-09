# Volatile

We just saw that our message was printed correctly. However, it might not work with future Rust compilers that optimize more aggressively.

The problem is that we only write to the Buffer and never read from it again.

The compiler doesn’t know that we really access VGA buffer memory (instead of normal RAM) and knows nothing about the side effect that some characters appear on the screen.

So it might decide that these writes are unnecessary and can be omitted.

To avoid this erroneous optimization, we need to specify these writes as volatile. This tells the compiler that the write has side effects and should not be optimized away.

In order to use volatile writes for the VGA buffer, we use the volatile library.

This crate (this is how packages are called in the Rust world) provides a Volatile wrapper type with read and write methods.

These methods internally use the read_volatile and write_volatile functions of the core library and thus guarantee that the reads/writes are not optimized away.

We can add a dependency on the volatile crate by adding it to the dependencies section of our Cargo.toml

Make sure to specify volatile version 0.2.6.

Newer versions of the crate are not compatible with this post. 0.2.6 is the semantic version number.

For more information, see the Specifying Dependencies guide of the cargo documentation.
