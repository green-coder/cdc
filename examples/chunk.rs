extern crate cdc_rs;

use std::u64;
use std::cmp::{min, max};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use cdc_rs::*;

#[inline]
fn my_default_predicate(x: u64) -> bool {
    (x & 0b1111111111111) == 0b1111111111111
}

fn chunk_file<S: Into<String>>(path: S) -> io::Result<()> {
    let f = try!(File::open(path.into()));
    let stream_length = f.metadata().unwrap().len();
    let reader: BufReader<File> = BufReader::new(f);
    let byte_iter = reader.bytes().map(|b| b.unwrap());

    //let separator_iter = SeparatorIter::new(byte_iter);
    //let separator_iter = SeparatorIter::custom_new(byte_iter, 6, |x| (x & 0b1111111111111) == 0b1111111111111);
    let separator_iter = SeparatorIter::custom_new(byte_iter, 6, my_default_predicate);
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
        size_variance += (chunk.size as i64 - expected_size as i64).pow(2);
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
	chunk_file("myLargeFile.bin").unwrap();
}
