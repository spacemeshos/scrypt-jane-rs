# scrypt-jane

This crate provides Rust bindings to the C library [floodberry/scrypt-jane](https://github.com/floodyberry/scrypt-jane).

The original library supports several mix and hashing functions, but they are selected at compile-time with -D compiler flags. This crate selects:

- [ChaCha20/8](http://cr.yp.to/chacha.html) mix function,
- [Keccak512](http://keccak.noekeon.org/) hash function (SHA-3).
