use crate::{
    flag_storage::FlagStorage,
    patterns::{index_pattern, MASK_PATTERNS_U8},
};

pub struct FlagStorageUnrolledBits8 {
    words: Vec<u8>,
    length_bits: usize,
}

impl FlagStorageUnrolledBits8 {
    const BITS: usize = u8::BITS as usize;

    // inline, since it's always the same
    #[inline(always)]
    fn reset_flags_sparse(&mut self, skip: usize) {
        let mask_set_index = ((skip / 2) - 1) % Self::BITS;
        let mask_set = MASK_PATTERNS_U8[mask_set_index];

        let rel_indices = index_pattern::<8>(skip);

        self.words.chunks_exact_mut(skip).for_each(|chunk| {
            for i in 0..Self::BITS {
                let word_idx = rel_indices[i];
                // TODO: safety note
                unsafe {
                    *chunk.get_unchecked_mut(word_idx) |= mask_set[i];
                }
            }
        });

        let remainder = self.words.chunks_exact_mut(skip).into_remainder();
        for i in 0..Self::BITS {
            let word_idx = rel_indices[i];
            if word_idx < remainder.len() {
                // TODO: safety note
                unsafe {
                    *remainder.get_unchecked_mut(word_idx) |= mask_set[i];
                }
            } else {
                break;
            }
        }

        // restore original factor bit -- we have clobbered it, and it is the prime
        let factor_index = skip / 2;
        let factor_word = factor_index / Self::BITS;
        let factor_bit = factor_index % Self::BITS;
        if let Some(w) = self.words.get_mut(factor_word) {
            *w &= !(1 << factor_bit);
        }
    }

    // rather do a function call; we'll have more registers available, and it's
    // not called very often
    #[inline(never)]
    fn reset_flags_dense<const SKIP: usize>(&mut self) {
        let mask_set_index = ((SKIP / 2) - 1) % Self::BITS;
        let mask_set = MASK_PATTERNS_U8[mask_set_index];

        let rel_indices = index_pattern::<8>(SKIP);

        self.words.chunks_exact_mut(SKIP).for_each(|chunk| {
            for i in 0..Self::BITS {
                let word_idx = rel_indices[i];
                // TODO: safety note
                unsafe {
                    *chunk.get_unchecked_mut(word_idx) |= mask_set[i];
                }
            }
        });

        let remainder = self.words.chunks_exact_mut(SKIP).into_remainder();
        for i in 0..Self::BITS {
            let word_idx = rel_indices[i];
            if word_idx < remainder.len() {
                // TODO: safety note
                unsafe {
                    *remainder.get_unchecked_mut(word_idx) |= mask_set[i];
                }
            } else {
                break;
            }
        }

        // restore original factor bit -- we have clobbered it, and it is the prime
        let factor_index = SKIP / 2;
        let factor_word = factor_index / Self::BITS;
        let factor_bit = factor_index % Self::BITS;
        if let Some(w) = self.words.get_mut(factor_word) {
            *w &= !(1 << factor_bit);
        }
    }
}

impl FlagStorage for FlagStorageUnrolledBits8 {
    fn create_true(size: usize) -> Self {
        let num_words = size / Self::BITS + (size % Self::BITS).min(1);
        Self {
            words: vec![0; num_words],
            length_bits: size,
        }
    }

    #[inline(always)]
    fn reset_flags(&mut self, skip: usize) {
        // call into dispatcher
        // TODO: autogenerate match_reset_dispatch!(self, skip, 19, reset_flags_dense, reset_flags_sparse);
        match skip {
            3 => self.reset_flags_dense::<3>(),
            5 => self.reset_flags_dense::<5>(),
            7 => self.reset_flags_dense::<7>(),
            9 => self.reset_flags_dense::<9>(),
            11 => self.reset_flags_dense::<11>(),
            // 13 => self.reset_flags_dense::<13>(),
            // 15 => self.reset_flags_dense::<15>(),
            // 17 => self.reset_flags_dense::<17>(),
            // 19 => self.reset_flags_dense::<19>(),
            _ => self.reset_flags_sparse(skip),
        };
    }

    #[inline(always)]
    fn get(&self, index: usize) -> bool {
        if index >= self.length_bits {
            return false;
        }
        let word = self.words.get(index / Self::BITS).unwrap();
        *word & (1 << (index % Self::BITS)) == 0
    }
}
