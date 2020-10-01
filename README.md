<!-- Copyright (c) 2020 Christopher Taylor                                          -->
<!--                                                                                -->
<!--   Distributed under the Boost Software License, Version 1.0. (See accompanying -->
<!--   file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)        -->

# armd4ocl - A Rust Macro DSL for OpenCL

Project details [here](http://www.github.com/ct-clmsn/armd4ocl/).

`armd4ocl` (or "armed-for-opencl") uses the Rust macro system to embed
OpenCL language syntax into Rust. Users no longer have to write OpenCL
programs into a raw string in a Rust program or load a text file containing
an OpenCL program into an application's memory.

Users can express OpenCL programs using an `opencl_str!` macro. The `opencl_str!`
macro syntax checks the user's code and exports well formed OpenCL programs
as a `&std::str`. This macro should be useable with any Rust binding for
OpenCL.

### Limitations

* Read the [Issue Tracker](https://github.com/ct-clmsn/armd4ocl/issues) prior to using this library to learn
it's limitations and edge cases. This macro only provides user's with syntax
checking, it does not provide static code analysis. Syntax checking is limited
to what the author is capable of doing with the Rust macro system and what the
Rust macro system allows.
* The OpenCL strings generated by `opencl_str!` will not have one-to-one formatting
with the user-provided input; the output will not be pretty-printed.
* Users should note that `for`, `while`, `if`, `else if`, `else` are [strict keywords](https://doc.rust-lang.org/reference/keywords.html#strict-keywords) in the Rust language. Strict keywords cannot be overloaded by the macro system. To use this macro in OpenCL programs,
users are required to replace their use of the OpenCL `for`, `while`, `if`, `else if`, `else` keyword by using
`for_`, `while_`, `if_`, `else_if_`, `else_` as an alternative token.
* Support for OpenCL function calls is currently not available.
* Since the macro is a syntax checker, it will allow recursive function calls
in an OpenCL program. Recursion is not permitted in OpenCL programs according
to the language standard.

### License

Boost Software License

### Features

* Users can embed OpenCL programs into a Rust application
* Compile-time syntax checking of OpenCL programs

### Demo

`cargo test

### TODO

* Procedural macro support

### Author

Christopher Taylor

### Dependencies

* [Rust](https://www.rust-lang.org)
* [OpenCL-bindings-1](https://crates.io/keywords/gpgpu)
* [OpenCL-bindings-2](https://crates.io/search?q=OpenCL)
* [OpenCL-bindings-3](https://crates.io/search?page=1&per_page=10&q=ocl)
