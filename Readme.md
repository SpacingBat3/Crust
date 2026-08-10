<div align=center>

<img src="https://github.com/tsoding/Crust/raw/main/crust.png" height="256px"/>

# "Crust"

My spin on *Rust, but C code* idea.

</div>

## Context

This is directly inspired by [`tscoding/Crust`](https://github.com/tsoding/Crust), but either
breaking some concepts or adding a few ones. It is also being shown by implementation, with
actual C code comparison, so you might compare and see by yourself.

To make it even less standard approach, I'm building C counterpart via Cargo, which might not
be made for a job, is experimental and, at least with my current implementation that is a subject
to change, is definitely not the way you should build C with, but nonetheless it works I guess.

## Some of the rules

> [!NOTE]
> This is still work in progress and the rules list is there non-exhaustive.
> Many of these are also concepts I might need to implement for the codebase.

### 1. `#[unsafe(no_mangle)]` for result symbols

C symbols are straightfoward and pretty, let's make Rust ones
the same!

### 2. C ABI as much as possible

If overusing `unsafe` was to less of a challenge for you, `extern "C"`,
`#repr(C)` and others are also now your best friends!

### 3. C header file, if symbols are `pub`.

If it is trully public, `include/` is always a must-have
with no excuses!

### 4. `#![no_std]`, `#![no_main]`, only `libc`

`std` is not standard library, if it isn't standard for
your operating system. Let's use true and only standard
library, `libc`! Who needs safe abstractions when you
can have undefined behaviors? 🙃️

### 5. Have fun hacking

Fork this, add your own rules that you think will make Rust
*more C* and be more than welcome to experiment with our fun
coding concept! Make your priority for the projects to be
first and foremost made for fun.

## Benefits of Crust???

(Un)surprisingly, if you remove `std` from Rust, you gain
the benefit of depending on nothing, and that by itself
plus `libc` can give you insane disk space savings for
the price of C `unsafe` nature of `libc`. At the same time,
I've noticed that Rust can somehow produce binaries that
take less space than respective C binary. You might also
notice other disadvantages or benefits, like Rust quirks
that makes it more safe or better defines operations, or
that there is significant difference in C ops vs Rust ones.
