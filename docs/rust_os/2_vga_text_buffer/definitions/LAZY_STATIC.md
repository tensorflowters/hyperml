# Lazy Statics

The one-time initialization of statics with non-const functions is a common problem in Rust. Fortunately, there already exists a good solution in a crate named lazy_static. This crate provides a lazy_static! macro that defines a lazily initialized static. Instead of computing its value at compile time, the static lazily initializes itself when accessed for the first time. Thus, the initialization happens at runtime, so arbitrarily complex initialization code is possible.

Let’s add the lazy_static crate to our project

However, this WRITER is pretty useless since it is immutable. This means that we can’t write anything to it (since all the write methods take &mut self). One possible solution would be to use a mutable static. But then every read and write to it would be unsafe since it could easily introduce data races and other bad things. Using static mut is highly discouraged. There were even proposals to remove it. But what are the alternatives? We could try to use an immutable static with a cell type like RefCell or even UnsafeCell that provides interior mutability. But these types aren’t Sync (with good reason), so we can’t use them in statics.

## Spinlocks

To get synchronized interior mutability, users of the standard library can use Mutex. It provides mutual exclusion by blocking threads when the resource is already locked. But our basic kernel does not have any blocking support or even a concept of threads, so we can’t use it either. However, there is a really basic kind of mutex in computer science that requires no operating system features: the spinlock. Instead of blocking, the threads simply try to lock it again and again in a tight loop, thus burning CPU time until the mutex is free again.

To use a spinning mutex, we can add the spin crate as a dependency

## Safety

Note that we only have a single unsafe block in our code, which is needed to create a Buffer reference pointing to 0xb8000. Afterwards, all operations are safe. Rust uses bounds checking for array accesses by default, so we can’t accidentally write outside the buffer. Thus, we encoded the required conditions in the type system and are able to provide a safe interface to the outside.

## A println Macro

Now that we have a global writer, we can add a println macro that can be used from anywhere in the codebase. Rust’s macro syntax is a bit strange, so we won’t try to write a macro from scratch. Instead, we look at the source of the println! macro in the standard library:

```rust
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
```

Macros are defined through one or more rules, similar to match arms. The println macro has two rules: The first rule is for invocations without arguments, e.g., println!(), which is expanded to print!("\n") and thus just prints a newline. The second rule is for invocations with parameters such as println!("Hello") or println!("Number: {}", 4). It is also expanded to an invocation of the print! macro, passing all arguments and an additional newline \n at the end.

The #[macro_export] attribute makes the macro available to the whole crate (not just the module it is defined in) and external crates. It also places the macro at the crate root, which means we have to import the macro through use std::println instead of std::macros::println.

The print! macro is defined as:

```rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}
```

The macro expands to a call of the _print function in the io module. The $crate variable ensures that the macro also works from outside the std crate by expanding to std when it’s used in other crates.

The format_args macro builds a fmt::Arguments type from the passed arguments, which is passed to `_print`. The `_print` function of libstd calls print_to, which is rather complicated because it supports different Stdout devices. We don’t need that complexity since we just want to print to the VGA buffer.

To print to the VGA buffer, we just copy the println! and print! macros, but modify them to use our own _print function.
