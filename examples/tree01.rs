use std::sync::atomic::{AtomicU32, Ordering};
use cdc::*;

type IntHash = u32;

static HASH_ID: AtomicU32 = AtomicU32::new(0);
fn get_new_hash_id() -> IntHash {
    HASH_ID.fetch_add(1, Ordering::Relaxed)
}

fn my_new_node(level: usize, children: &Vec<IntHash>) -> Node<IntHash> {
    Node {
        hash: get_new_hash_id(),
        level,
        children: children.clone(),
    }
}

fn main() {
    let levels = [0usize, 0, 1, 0, 1, 1, 2, 1, 0, 1, 0, 1];
    //let levels = [1usize, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1];
    let hashed_chunk_it = levels.iter().enumerate().map(|(index, level)| HashedChunk {
        hash: index as IntHash,
        level: *level,
    });

    HASH_ID.store(levels.len() as _, Ordering::Relaxed);

    for node in NodeIter::new(hashed_chunk_it, my_new_node, 0) {
        println!("{:?}", node);
    }
}
