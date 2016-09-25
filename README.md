### Description

This Rust crate is a very useful library for performing a *Content-Defined Chunking* (CDC).

### Content-Defined Chunking

CDC is when you look for some specific pattern inside a data stream, and you use them as separator for cutting your file into small pieces.

It is useful when your data has changed and you want to express the new version based on the previous one: one way of doing it is to refer to the pieces of the previous data which were not modified and to the new pieces which contains the modified bits.

### What's in the crate

From low level to high level:

* A `RollingHash64` trait, for rolling hash with a 64 bits hash value.

* `Rabin64`, an implementation of the Rabin Fingerprint rolling hash with a 64 bits hash value.

* `Separator`, a struct which describes a place in a data stream identified as a separator.

* `SeparatorIter`, an adaptor which takes an `Iterator<Item=u8>` as input and which enumerates all the separators found.

* `Chunk`, a struct which describes a piece of the data stream (index and size).

* `ChunkIter`, an adaptor which takes an `Iterator<Item=Separator>` as input and which enumerates chunks.

### Implementation details

* The library is not cutting any files, it only provides information on how to do it.

* You can change the default window size used by `Rabin64`, and how the `SeparatorIter` is choosing the separator.

* The design of this crate is not final and is probably going to change in the future.

### Performance

There is a **huge** difference between the debug build and the release build in terms of performance. Remember that when you test the lib, use `cargo run --release`.

I may try to improve the performance of the lib at some point, but for now they are good enough for my own usage.

### Getting started

Each module is documented via an example which you can find in the `examples/` folder.

To run them, use a command like:

    cargo run --example separator --release

Note: At the moment, some examples are looking for a file named `myLargeFile.bin` which I didn't uploaded to Github. Please use your own file for testing.

### License

This piece of software is licensed under the [MIT license](LICENSE.txt).
