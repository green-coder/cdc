use super::Separator;

pub struct Chunk {
    pub index: u64,
    pub size: u64,
    pub separator_hash: u64,
}

pub struct ChunkIter<Iter> {
    separators: Iter,
    stream_length: u64,
    last_separator_index: u64,
}

impl<Iter: Iterator<Item = Separator>> ChunkIter<Iter> {
    pub fn new(iter: Iter, stream_length: u64) -> ChunkIter<Iter> {
        ChunkIter {
            separators: iter,
            stream_length: stream_length,
            last_separator_index: 0,
        }
    }
}

impl<Iter: Iterator<Item = Separator>> Iterator for ChunkIter<Iter> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        match self.separators.next() {
            Some(separator) => {
                let chunk_size = separator.index - self.last_separator_index;
                self.last_separator_index = separator.index;
                return Some(Chunk {
                    index: self.last_separator_index,
                    size: chunk_size,
                    separator_hash: separator.hash,
                });
            }
            None => {
                let chunk_size = self.stream_length - self.last_separator_index;
                self.last_separator_index = self.stream_length;
                if chunk_size > 0 {
                    return Some(Chunk {
                        index: self.last_separator_index,
                        size: chunk_size,
                        separator_hash: 0, // any value is ok, last chunk of the stream.
                    });
                } else {
                    return None;
                }
            }
        }
    }
}
