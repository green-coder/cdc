extern crate cdc_rs;

use cdc_rs::*;

static mut hash_id: usize = 0;
fn get_new_hash_id() -> usize {
    unsafe {
        let id = hash_id;
        hash_id += 1;
        id
    }
}

fn my_new_node(level: usize, children: &Vec<Hash256>) -> HashNode {
    HashNode {
        hash: get_new_hash_id(),
        level: level,
        children: children.clone()
    }
}

fn main() {
    let levels = [0usize, 0, 1, 0, 1, 1, 2, 1, 0, 1, 0, 1];
    //let levels = [1usize, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1];
    let hashed_chunk_it = levels.iter().enumerate().map(|(index, level)| {
        HashedChunk {hash: index, level: *level}
    });

    unsafe {
        hash_id = levels.len();
    }

    for node in NodeIter::new(hashed_chunk_it, my_new_node, 0) {
        println!("{:?}", node);
    }
}
