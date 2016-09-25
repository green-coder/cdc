extern crate cdc;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use cdc::*;

fn chunk_file<S: Into<String>>(path: S) -> io::Result<()> {
    let f = try!(File::open(path.into()));
    let reader: BufReader<File> = BufReader::new(f);
    let byte_iter = reader.bytes().map(|b| b.unwrap());

    let mut nb_separator: usize = 0;
    for separator in SeparatorIter::new(byte_iter) {
        println!("Index: {}, hash: {:016x}", separator.index, separator.hash);
        nb_separator += 1;
    }
    println!("We found {} separators.", nb_separator);

	Ok(())
}

fn main() {
	chunk_file("myLargeFile.bin").unwrap();
}
