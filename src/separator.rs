use super::{Rabin64, RollingHash64};

pub struct Separator {
    pub index: u64,
    pub hash: u64,
}

pub struct SeparatorIter<I, F> {
    iter: I,
    predicate: F,
    rabin: Rabin64,
    index: u64,
}

impl<I> SeparatorIter<I, fn(u64) -> bool>
where
    I: Iterator<Item = u8>,
{
    pub fn new(iter: I) -> SeparatorIter<I, fn(u64) -> bool> {
        // window_size: 1 << 6 == 64 bytes
        let separator_size_nb_bits = 6;

        #[inline]
        fn default_predicate(x: u64) -> bool {
            const BITMASK: u64 = (1u64 << 13) - 1;
            x & BITMASK == BITMASK
        }

        Self::custom_new(iter, separator_size_nb_bits, default_predicate)
    }
}

impl<I, F> SeparatorIter<I, F>
where
    I: Iterator<Item = u8>,
    F: Fn(u64) -> bool,
{
    pub fn custom_new(
        mut iter: I,
        separator_size_nb_bits: u32,
        predicate: F,
    ) -> SeparatorIter<I, F> {
        let mut rabin = Rabin64::new(separator_size_nb_bits);
        let index = rabin.reset_and_prefill_window(&mut iter) as u64;

        SeparatorIter {
            iter: iter,
            predicate: predicate,
            rabin: rabin,
            index: index,
        }
    }
}

impl<I, F> Iterator for SeparatorIter<I, F>
where
    I: Iterator<Item = u8>,
    F: Fn(u64) -> bool,
{
    type Item = Separator;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(byte) = self.iter.next() {
            self.rabin.slide(&byte);
            self.index += 1;
            if (self.predicate)(self.rabin.hash) {
                let separator = Separator {
                    index: self.index,
                    hash: self.rabin.hash,
                };

                // Note: We skip subsequent separators which may overlap the current one.
                self.index += self.rabin.reset_and_prefill_window(&mut self.iter) as u64;

                return Some(separator);
            }
        }

        None
    }
}

// Converts a separator's hash to a level.
pub struct HashToLevel {
    lvl0_nb_bits: u32,
    lvlup_nb_bits: u32,
    lvlup_bitmask: u64,
}

impl HashToLevel {
    pub fn new() -> HashToLevel {
        Self::custom_new(13, 3)
    }

    pub fn custom_new(lvl0_nb_bits: u32, lvlup_nb_bits: u32) -> HashToLevel {
        HashToLevel {
            lvl0_nb_bits: lvl0_nb_bits,
            lvlup_nb_bits: lvlup_nb_bits,
            lvlup_bitmask: (1u64 << lvlup_nb_bits) - 1,
        }
    }

    pub fn to_level(&self, hash: u64) -> usize {
        let mut level = 0usize;
        let mut h = hash >> self.lvl0_nb_bits;
        while h & self.lvlup_bitmask == self.lvlup_bitmask {
            level += 1;
            h >>= self.lvlup_nb_bits;
        }

        level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_to_level() {
        let converter = HashToLevel::custom_new(4, 2);

        for n in 0..4 {
            assert_eq!(converter.to_level((9u64 << n) - 1), 0);
        }
        for n in 4..6 {
            assert_eq!(converter.to_level((9u64 << n) - 1), 0);
        }
        for n in 6..8 {
            assert_eq!(converter.to_level((9u64 << n) - 1), 1);
        }
        for n in 8..10 {
            assert_eq!(converter.to_level((9u64 << n) - 1), 2);
        }
        for n in 10..12 {
            assert_eq!(converter.to_level((9u64 << n) - 1), 3);
        }
        for n in 12..14 {
            assert_eq!(converter.to_level((9u64 << n) - 1), 4);
        }
    }
}
