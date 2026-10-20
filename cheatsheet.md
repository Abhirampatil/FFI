PROXIE STUDIO CHEAT SHEET 

 FUNDAMENTALS


Q. what is the project ?

A -->  it is a Rust wrapper around  C++ library which demonstrate  interoperability between 2 languages eg C++ and rust in our case.
   --> Wraps a small C++ string library so Rust code can use it through a 100%-safe API — no raw pointers or manual memory management exposed.

Q. what is FFI

A --> (Foreign Function Interface)  A programming tool that lets code written in one language call functions or use services written in another language
    --> FFI is just the mechanism that lets code written in one language call code written in another. It works because, deep down, compiled code is just machine instructions with a calling convention if two languages agree on that convention, they can call each other.
    --> This assignment is a miniature, self-contained version of that exact real-world problem
    -->  Rust can use the  wrapped C++ code so Rust programmers never have to touch unsafe C pointers directly.

Q. Extern C 

A --> names get "mangled" (renamed internally, to support things like function overloading) so Rust can't predict. 
    --> extern "C"  is used — it tells the C++ compiler to use plain C-style names," so Rust can find and call them.
    --> used to enable Foreign Function Interface (FFI) interoperability, allowing Rust code to communicate directly with C code

Q. memory management

A  --> in C/C++ you allocate memory using malloc and must be freed manually by user by callng Free.            forget it and memory will leak. calling it twice will result in a crash 
     --> Rust's string & vector free themselves automatically when they go out of scope unlike C/C++ (this is called RAII resource aquisition in initialization)
     -->  Rust copies the bytes into an rust String, then immediately call free on the original C++ buffer via proxie_free_string. (Avoiding memory leak). (C++ memory is gone, only exists a copy in Rust)

Q. Bindgen  

A  --> Bindgen is a tool that reads a C++ header file and automatically generates the matching (unsafe) Rust function declarations.   (Pointed in Wrapper.h). 
     Produces a file fill of raw extern "C"
     --> Reads the C header (wrapper.h) and auto-generates the raw Rust FFI function declarations, so you don't hand-write them.



C++ side contains (string functions)
• str_length() — Calculate string length
• str_reverse() — Reverse a string in-place or return new
• count_vowels() — Count vowels in a string
• to_uppercase() — Convert string to uppercase



Q&A

Q. Walk me through your project.

1 --> There's a small C++ library with four string functions
2 --> exposed  through a C-compatible header using extern "C".
3 --> A build.rs script compiles that C++ code and runs bindgen against the header to generate raw Rust bindings automatically.
4 --> Then lib.rs  wraps those raw bindings in safe functions — each one converts the Rust string into a C string, calls into C++ inside a small unsafe block, and converts the result back into a safe Rust type, freeing any C++-allocated memory along the way.

Q. Why is none of your public API marked unsafe?

1 --> Because the whole point of a wrapper is that callers shouldn't have to think about raw pointers or manual memory — that's what "safe Rust API" means.
2 --> The unsafe parts are real, but they're contained to a couple of small blocks inside lib.rs, each with a comment explaining exactly what assumption makes it safe. That way there's one place to audit instead of unsafety scattered through the codebase. 

Q. How do you prevent memory leaks across the FFI boundary?

1 --> The two functions that allocate — str_reverse and to_uppercase — get a malloc'd buffer back from C++.
2 --> copy those bytes into an owned Rust String immediately, then call free on the C buffer right after, inside the same function.
3 --> though havent tried the valgrind test

Q. What would you do differently with more time?

1 --> Make the operations Unicode-aware at the character level instead of the byte level, 
2 --> add more property-based/fuzz testing on the FFI boundary, 
3 --> probably benchmark against a pure-Rust implementation to see what the FFI overhead actually costs.


Q. why this project ?

A --> A company might have a huge, battle-tested C++ library doing some heavy lifting and want to build new services around it in a language with better safety guarantees, like Rust. Nobody rewrites the C++ from scratch Instead, you build a bridge: a thin layer that lets Rust call into the existing C++ code safely. that bridge if FFI


Q. Known limitation

The string operations work at the byte level, not per Unicode character. For
plain English/ASCII input this doesn't matter at all. For non-ASCII input,
`str_reverse` will error out (see above) rather than return something wrong,
and `count_vowels`/`str_length` only count/measure the ASCII portion as
documented. Given the 4-day scope I didn't try to build full Unicode-aware
string handling - it would need a lot more code for something the assignment
didn't really ask for, and I'd rather have honest error behavior than a
half-working feature.

Overall Architecure of the project                     YOUR RUST CODE
                         │
                         ▼
              ┌─────────────────────┐
              │   src/lib.rs        │
              │                     │
              │ Safe Rust API       │
              │                     │
              │ str_reverse()       │
              │ str_length()        │
              │ count_vowels()      │
              │ to_uppercase()      │
              └──────────┬──────────┘
                         │
                         │ unsafe FFI call
                         ▼
              ┌─────────────────────┐
              │ generated bindings  │
              │      bindgen        │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │     wrapper.h       │
              │                     │
              │ extern "C"          │
              │ C-compatible API    │
              └──────────┬──────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │      lib.cpp        │
              │                     │
              │ Actual C++ code     │
              │ malloc/free/etc.    │
              └─────────────────────┘

cargo build


Cargo
  │
  ▼
build.rs
  ├──────────────► cc ─────────► lib.cpp
  │
  └──────────────► bindgen ────► wrapper.h
                                  │
                                  ▼
                              bindings.rs






