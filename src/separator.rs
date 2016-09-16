use super::{RollingHash64, Rabin64};

pub struct Separator {
    pub index: u64,
    pub hash: u64,
}

pub struct SeparatorIter<InputIter> {
    iter: InputIter,
    rabin: Rabin64,
    bitmask: u64,
    masked_value: u64,
    index: u64,
}

impl<InputIter: Iterator<Item=u8>> SeparatorIter<InputIter> {
    pub fn new(iter: InputIter) -> SeparatorIter<InputIter> {
        // window_size: 1 << 6 == 64 bytes
        Self::custom_new(iter, 6, (1u64 << 13) - 1, (1u64 << 13) - 1)
    }

    pub fn custom_new(mut iter: InputIter,
        separator_size_nb_bits: u32,
        bitmask: u64,
        masked_value: u64) -> SeparatorIter<InputIter> {

        let mut rabin = Rabin64::new(separator_size_nb_bits);
        let index = rabin.prefill_window(&mut iter) as u64;

        SeparatorIter {
            iter: iter,
            rabin: rabin,
            bitmask: bitmask,
            masked_value: masked_value,
            index: index,
        }

    }
}

impl<InputIter: Iterator<Item=u8>> Iterator for SeparatorIter<InputIter> {
    type Item = Separator;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            self.rabin.slide(&v);
            self.index += 1;
            if self.rabin.hash & self.bitmask == self.masked_value {
                let separator = Some(Separator {index: self.index, hash: self.rabin.hash});

                // Note: We skip consequent separators which may overlap the current one.
                self.rabin.reset();
                self.index += self.rabin.prefill_window(&mut self.iter) as u64;

                return separator;
            }
        }

        None
    }

}
