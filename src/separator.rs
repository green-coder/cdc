use super::{RollingHash64, Rabin64};

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

impl<I> SeparatorIter<I, fn(u64) -> bool> where I: Iterator<Item=u8> {
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

impl<I, F> SeparatorIter<I, F> where I: Iterator<Item=u8>, F: Fn(u64) -> bool {
    pub fn custom_new(mut iter: I,
        separator_size_nb_bits: u32,
        predicate: F) -> SeparatorIter<I, F> {
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

impl<I, F> Iterator for SeparatorIter<I, F> where I: Iterator<Item=u8>, F: Fn(u64) -> bool {
    type Item = Separator;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            self.rabin.slide(&v);
            self.index += 1;
            if (self.predicate)(self.rabin.hash) {
                let separator = Some(Separator {index: self.index, hash: self.rabin.hash});

                // Note: We skip subsequent separators which may overlap the current one.
                self.index += self.rabin.reset_and_prefill_window(&mut self.iter) as u64;

                return separator;
            }
        }

        None
    }

}
