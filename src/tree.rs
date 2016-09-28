/// Example of type to use with the generic structures below.
//pub type Hash256 = [u8; 256/8];

#[derive(Debug)]
pub struct HashedChunk<H> {
    pub hash: H,
    pub level: usize,
}

#[derive(Debug)]
pub struct Node<H> {
    pub hash: H,  // The hash acting as the ID of this node.
    pub level: usize,
    pub children: Vec<H>,
}

pub struct NodeIter<I, F, H> {
    // Configuration
    chunks: I,
    new_node: F,
    max_children: usize,

    // Internal state
    level_hashes: Vec<Vec<H>>, // level_hashes[level] -> Vec<H>
    out_buffer: Vec<Node<H>>, // Fifo
}

impl<I, F, H> NodeIter<I, F, H> where
        I: Iterator<Item=HashedChunk<H>>,
        F: Fn(usize, &Vec<H>) -> Node<H>,
        H: Copy {
    pub fn new(iter: I, new_node: F, max_node_children: usize) -> NodeIter<I, F, H> {
        NodeIter {
            chunks: iter,
            new_node: new_node,
            max_children: max_node_children,
            level_hashes: Vec::with_capacity(16),
            out_buffer: Vec::with_capacity(16),
        }
    }

    fn add_at_level(&mut self, level: usize, hash: H) {
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

impl<I, F, H> Iterator for NodeIter<I, F, H> where
        I: Iterator<Item=HashedChunk<H>>,
        F: Fn(usize, &Vec<H>) -> Node<H>,
        H: Copy {
    type Item = Node<H>;

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
