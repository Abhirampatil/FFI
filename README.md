# proxie-ffi

Rust wrapper around a small C++ string utilities library, done for the Proxie Studio
backend evaluation (Round 1 - Rust FFI + bindgen).

## What's implemented

Went with Option A (String Utilities). Four functions, all wrapped in safe Rust:

- `str_length(&str) -> Result<usize, Error>`
- `str_reverse(&str) -> Result<String, Error>`
- `count_vowels(&str) -> Result<i32, Error>`
- `to_uppercase(&str) -> Result<String, Error>`

```rust
use proxie_ffi::{str_length, str_reverse, count_vowels, to_uppercase};

str_length("hello")?;     // 5
str_reverse("hello")?;    // "olleh"
count_vowels("hello")?;   // 2
to_uppercase("hello")?;   // "HELLO"
```

Everything returns a `Result` instead of panicking, so callers get to decide what to
do when something goes wrong instead of the program just crashing.

## Requirements to build

- Rust/Cargo (built and tested on 1.75)
- g++ or another C++ compiler
- libclang, since bindgen needs it to parse the header. On Ubuntu:
  `sudo apt-get install libclang-dev clang`

## Build & test

```bash
cargo build
cargo test
```

That's it, no extra setup scripts. build.rs handles compiling the C++ file and generating the bindings automatically.

The test suite includes repeated FFI allocation/free cycles to exercise
ownership and cleanup behavior.

## Folder layout

```
proxie-ffi/
├── build.rs                # compiles cpp/lib.cpp + runs bindgen
├── Cargo.toml
├── cpp/
│   ├── wrapper.h            # C-compatible header, extern "C"
│   └── lib.cpp              # actual C++ implementation
├── src/
│   └── lib.rs                # safe Rust API + private ffi module
└── tests/
    └── integration_test.rs   # 17 tests - correctness, edge cases, stress
```

## Memory management

`str_reverse` and `to_uppercase` allocate their result on the C++ side with
`malloc`. The Rust side takes that pointer, copies the bytes into a `String` it
owns, then immediately calls `proxie_free_string` (just a wrapper around `free`)
to release the C buffer. So the C-allocated memory never outlives the single
function call that produced it - nothing gets held onto across calls.

`str_length` and `count_vowels` only read the input, they don't allocate
anything so there's nothing to free there.

All the `unsafe` in the project lives in `src/lib.rs`. It is limited to
calling the generated FFI functions and handling the raw C string pointer
returned by the allocating functions. Every unsafe block has a comment
above it explaining the assumption it relies on.

None of the public functions expose `unsafe` in their signature - if you're
calling `str_reverse("hello")` from outside this crate you never touch a
raw pointer.

## A few design decisions

The main thing I had to think about was that C strings are just bytes, but
Rust strings have to be valid UTF-8. `str_reverse` reverses the raw bytes of
the string, which works fine for plain ASCII text but breaks if the string has
multi-byte characters in it (like "café" - the é takes 2 bytes, and reversing
byte-by-byte splits it apart). I didn't want to just force those bytes into a
`String` anyway (that would either panic or silently produce garbage), so I
used `String::from_utf8`, which is the checked version, and it returns an
error if the bytes aren't valid UTF-8. So reversing "café" gives you a clean
error instead of a broken string. There's a test for this
(`test_unicode_str_reverse_returns_error_instead_of_corrupt_string`).

`to_uppercase` doesn't have this problem because it only modifies bytes below
128 (plain ASCII), so multi-byte characters just pass through untouched and
the result stays valid UTF-8.

For errors I used the `thiserror` crate instead of writing a manual `impl
Display` - it's a pretty standard tool for exactly this and saved having to
hand-roll error formatting.

Build-wise, `cc` compiles the C++ file directly from `build.rs`, no separate
Makefile needed, and I added `cargo:rerun-if-changed` on the C++ files so
edits there actually trigger a rebuild.

## Known limitation

The string operations work at the byte level, not per Unicode character. For
plain English/ASCII input this doesn't matter at all. For non-ASCII input,
`str_reverse` will error out (see above) rather than return something wrong,
and `count_vowels`/`str_length` only count/measure the ASCII portion as
documented. Given the 4-day scope I didn't try to build full Unicode-aware
string handling - it would need a lot more code for something the assignment
didn't really ask for, and I'd rather have honest error behavior than a
half-working feature.
