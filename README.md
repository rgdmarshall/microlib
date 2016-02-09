# microlib

[![Build Status](https://travis-ci.org/rgdmarshall/microlib.svg?branch=master)](https://travis-ci.org/rgdmarshall/microlib)

Microlib is a do-nothing library which serves as a proving ground
for GH/Travis integration.

See the [documentation](https://rgdmarshall.github.io/microlib) for API reference.

## Getting Started

Add the following to `Cargo.toml`:

```toml
[dependencies]
microlib = "0.1"
```

Also, import the crate in your crate root:

```rust
extern crate microlib;
```

## Example

```rust
extern crate microlib;

use microlib;

if microlib::is_awesome() {
    println!("Awesome!");
}
```
