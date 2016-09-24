//pub type Hash256 = [u8; 256/8];
pub type Hash256 = usize;

pub struct HashedChunk {
    pub hash: Hash256,
    pub level: usize,
}

#[derive(Debug)]
pub struct HashNode {
    pub hash: Hash256,  // The hash of data from some of the fields below.
    pub level: usize,
    pub children: Vec<Hash256>,
}

pub struct NodeIter<I, F> {
    // Configuration
    chunks: I,
    new_node: F,
    max_children: usize,

    // Internal state
    level_hashes: Vec<Vec<Hash256>>, // level_hashes[level] -> Vec<Hash256>
    out_buffer: Vec<HashNode>, // Fifo
}

impl<I, F> NodeIter<I, F>
    where I: Iterator<Item=HashedChunk>, F: Fn(usize, &Vec<Hash256>) -> HashNode {
    pub fn new(iter: I, new_node: F, max_node_children: usize) -> NodeIter<I, F> {
        NodeIter {
            chunks: iter,
            new_node: new_node,
            max_children: max_node_children,
            level_hashes: Vec::with_capacity(16),
            out_buffer: Vec::with_capacity(16),
        }
    }

    fn add_at_level(&mut self, level: usize, hash: Hash256) {
        // Ensures that the vector is large enough.
        if level >= self.level_hashes.len() {
            self.level_hashes.resize(level + 1, vec![]);
        }

        self.level_hashes[level].push(hash);

        // If max_children was set to non-zero, limit the number of children.
        if self.level_hashes[level].len() == self.max_children {
            self.output_level(level);
        }
    }

    fn output_level(&mut self, level: usize) {
        match self.level_hashes[level].len() {
            0 => return, // Don't output empty nodes.
            1 => {
                // Don't output a node with only 1 hash, move it to the upper level.
                let level_up_hash = self.level_hashes[level][0];
                self.level_hashes[level].clear();
                self.add_at_level(level + 1, level_up_hash);
            },
            _ => {
                let node = (self.new_node)(level, &self.level_hashes[level]);
                let level_up_hash = node.hash;
                self.out_buffer.push(node);
                self.level_hashes[level].clear();
                self.add_at_level(level + 1, level_up_hash);
            }
        }
    }

    fn output_levels(&mut self, below_level: usize) {
        for level in 0..below_level {
            self.output_level(level);
        }
    }

}

impl<I, F> Iterator for NodeIter<I, F>
    where I: Iterator<Item=HashedChunk>, F: Fn(usize, &Vec<Hash256>) -> HashNode {
    type Item = HashNode;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.out_buffer.len() > 0 {
                return self.out_buffer.pop();
            }

            if let Some(chunk) = self.chunks.next() {
                self.add_at_level(0, chunk.hash);
                self.output_levels(chunk.level);
                self.out_buffer.reverse();
            }
            else {
                let len = self.level_hashes.len();
                if len > 0 {
                    // Flush the remaining hashes.
                    self.output_levels(len);
                    self.level_hashes.clear();
                    self.out_buffer.reverse();
                }
                else {
                    return None;
                }
            }
        }
    }
}
