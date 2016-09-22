### Description

This Rust crate contains useful classes for performing a *Content-Defined Chunking* (CDC).

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

Example of usage:

```rust
extern crate cdc_rs;

use std::u64;
use std::cmp::{min, max};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use cdc_rs::*;

fn find_chunks_in_file<S: Into<String>>(path: S) -> io::Result<()> {
    let f = try!(File::open(path.into()));
    let stream_length = f.metadata().unwrap().len();
    let reader: BufReader<File> = BufReader::new(f);
    let byte_iter = reader.bytes().map(|b| b.unwrap());
    let separator_iter = SeparatorIter::new(byte_iter);
    let chunk_iter = ChunkIter::new(separator_iter, stream_length);

    let mut nb_chunk = 0;
    let mut total_size = 0;
    let mut smallest_size = u64::MAX;
    let mut largest_size = 0;
    let expected_size = 1 << 13;
    let mut size_variance = 0;
    for chunk in chunk_iter {
        println!("Index: {}, size: {:6}, separator_hash: {:016x}", chunk.index, chunk.size, chunk.separator_hash);
        nb_chunk += 1;
        total_size += chunk.size;
        smallest_size = min(smallest_size, chunk.size);
        largest_size = max(largest_size, chunk.size);
        size_variance += (chunk.size - expected_size).pow(2);
    }

    println!("{} chunks with an average size of {} bytes.", nb_chunk, total_size / nb_chunk);
    println!("Expected chunk size: {} bytes", expected_size);
    println!("Smallest chunk: {} bytes.", smallest_size);
    println!("Largest chunk: {} bytes.", largest_size);
    println!("Standard size deviation: {} bytes.",
        (size_variance as f64 / nb_chunk as f64).sqrt() as u64);

	Ok(())
}

fn main() {
	find_chunks_in_file("my_large_file.bin").unwrap();
}
```

### License

This piece of software is licensed under the [MIT license](LICENSE.txt).
