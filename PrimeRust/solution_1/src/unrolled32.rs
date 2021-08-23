use crate::{
    flag_storage::FlagStorage,
    patterns::{index_pattern, MASK_PATTERNS_U32},
};

pub struct FlagStorageUnrolledBits32 {
    words: Vec<u32>,
    length_bits: usize,
}

impl FlagStorageUnrolledBits32 {
    const BITS: usize = u32::BITS as usize;

    // TODO: consider inlining
    #[inline(never)]
    fn reset_flags_sparse<const EQUIVALENT_SKIP: usize>(&mut self, skip: usize) {
        let mask_set_index = ((EQUIVALENT_SKIP / 2) - 1) % Self::BITS;
        let mask_set = MASK_PATTERNS_U32[mask_set_index];

        let rel_indices = index_pattern::<32>(skip);

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
}

impl FlagStorage for FlagStorageUnrolledBits32 {
    fn create_true(size: usize) -> Self {
        let num_words = size / Self::BITS + (size % Self::BITS).min(1);
        Self {
            words: vec![0; num_words],
            length_bits: size,
        }
    }

    #[inline(never)]
    fn reset_flags(&mut self, skip: usize) {
        // call into dispatcher
        // TODO: autogenerate match_reset_dispatch!(self, skip, 19, reset_flags_dense, reset_flags_sparse);
        match skip {
            3 => ResetterDenseU32::<3>::reset_dense(&mut self.words),
            5 => ResetterDenseU32::<5>::reset_dense(&mut self.words),
            7 => ResetterDenseU32::<7>::reset_dense(&mut self.words),
            9 => ResetterDenseU32::<9>::reset_dense(&mut self.words),
            11 => ResetterDenseU32::<11>::reset_dense(&mut self.words),
            13 => ResetterDenseU32::<13>::reset_dense(&mut self.words),
            15 => ResetterDenseU32::<15>::reset_dense(&mut self.words),
            17 => ResetterDenseU32::<17>::reset_dense(&mut self.words),
            19 => ResetterDenseU32::<19>::reset_dense(&mut self.words),
            21 => ResetterDenseU32::<21>::reset_dense(&mut self.words),
            23 => ResetterDenseU32::<23>::reset_dense(&mut self.words),
            25 => ResetterDenseU32::<25>::reset_dense(&mut self.words),
            27 => ResetterDenseU32::<27>::reset_dense(&mut self.words),
            29 => ResetterDenseU32::<29>::reset_dense(&mut self.words),
            31 => ResetterDenseU32::<31>::reset_dense(&mut self.words),
            33 => ResetterDenseU32::<33>::reset_dense(&mut self.words),
            35 => ResetterDenseU32::<35>::reset_dense(&mut self.words),
            37 => ResetterDenseU32::<37>::reset_dense(&mut self.words),
            39 => ResetterDenseU32::<39>::reset_dense(&mut self.words),
            41 => ResetterDenseU32::<41>::reset_dense(&mut self.words),
            43 => ResetterDenseU32::<43>::reset_dense(&mut self.words),
            45 => ResetterDenseU32::<45>::reset_dense(&mut self.words),
            47 => ResetterDenseU32::<47>::reset_dense(&mut self.words),
            49 => ResetterDenseU32::<49>::reset_dense(&mut self.words),
            51 => ResetterDenseU32::<51>::reset_dense(&mut self.words),
            53 => ResetterDenseU32::<53>::reset_dense(&mut self.words),
            55 => ResetterDenseU32::<55>::reset_dense(&mut self.words),
            57 => ResetterDenseU32::<57>::reset_dense(&mut self.words),
            59 => ResetterDenseU32::<59>::reset_dense(&mut self.words),
            61 => ResetterDenseU32::<61>::reset_dense(&mut self.words),
            63 => ResetterDenseU32::<63>::reset_dense(&mut self.words),
            65 => ResetterDenseU32::<65>::reset_dense(&mut self.words),
            skip_sparse => match ((skip_sparse / 2) - 1) % Self::BITS {
                // TODO: this needs a clean up; we're doing unnecessary conversions
                0 => self.reset_flags_sparse::<{ (0 + 1) * 2 + 1 }>(skip),
                1 => self.reset_flags_sparse::<{ (1 + 1) * 2 + 1 }>(skip),
                2 => self.reset_flags_sparse::<{ (2 + 1) * 2 + 1 }>(skip),
                3 => self.reset_flags_sparse::<{ (3 + 1) * 2 + 1 }>(skip),
                4 => self.reset_flags_sparse::<{ (4 + 1) * 2 + 1 }>(skip),
                5 => self.reset_flags_sparse::<{ (5 + 1) * 2 + 1 }>(skip),
                6 => self.reset_flags_sparse::<{ (6 + 1) * 2 + 1 }>(skip),
                7 => self.reset_flags_sparse::<{ (7 + 1) * 2 + 1 }>(skip),
                8 => self.reset_flags_sparse::<{ (8 + 1) * 2 + 1 }>(skip),
                9 => self.reset_flags_sparse::<{ (9 + 1) * 2 + 1 }>(skip),
                10 => self.reset_flags_sparse::<{ (10 + 1) * 2 + 1 }>(skip),
                11 => self.reset_flags_sparse::<{ (11 + 1) * 2 + 1 }>(skip),
                12 => self.reset_flags_sparse::<{ (12 + 1) * 2 + 1 }>(skip),
                13 => self.reset_flags_sparse::<{ (13 + 1) * 2 + 1 }>(skip),
                14 => self.reset_flags_sparse::<{ (14 + 1) * 2 + 1 }>(skip),
                15 => self.reset_flags_sparse::<{ (15 + 1) * 2 + 1 }>(skip),
                16 => self.reset_flags_sparse::<{ (16 + 1) * 2 + 1 }>(skip),
                17 => self.reset_flags_sparse::<{ (17 + 1) * 2 + 1 }>(skip),
                18 => self.reset_flags_sparse::<{ (18 + 1) * 2 + 1 }>(skip),
                19 => self.reset_flags_sparse::<{ (19 + 1) * 2 + 1 }>(skip),
                20 => self.reset_flags_sparse::<{ (20 + 1) * 2 + 1 }>(skip),
                21 => self.reset_flags_sparse::<{ (21 + 1) * 2 + 1 }>(skip),
                22 => self.reset_flags_sparse::<{ (22 + 1) * 2 + 1 }>(skip),
                23 => self.reset_flags_sparse::<{ (23 + 1) * 2 + 1 }>(skip),
                24 => self.reset_flags_sparse::<{ (24 + 1) * 2 + 1 }>(skip),
                25 => self.reset_flags_sparse::<{ (25 + 1) * 2 + 1 }>(skip),
                26 => self.reset_flags_sparse::<{ (26 + 1) * 2 + 1 }>(skip),
                27 => self.reset_flags_sparse::<{ (27 + 1) * 2 + 1 }>(skip),
                28 => self.reset_flags_sparse::<{ (28 + 1) * 2 + 1 }>(skip),
                29 => self.reset_flags_sparse::<{ (29 + 1) * 2 + 1 }>(skip),
                30 => self.reset_flags_sparse::<{ (30 + 1) * 2 + 1 }>(skip),
                31 => self.reset_flags_sparse::<{ (31 + 1) * 2 + 1 }>(skip),
                _ => debug_assert!(false, "this case should not occur"),
            },
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

struct ResetterDenseU32<const SKIP: usize>();
impl<const SKIP: usize> ResetterDenseU32<SKIP> {
    const BITS: usize = 32;

    const fn mask_pattern_set() -> [u32; 32] {
        let start = SKIP / 2;
        let mut pattern = [0; 32];
        let mut i = 0;
        while i < 32 {
            let relative_index = start + i * SKIP;
            let shift = relative_index % 32;
            let mask = 1u32 << shift;
            pattern[i] = mask;
            i += 1;
        }
        pattern
    }

    const fn index_pattern() -> [usize; 32] {
        let start = SKIP / 2;
        let mut pattern = [0; 32];
        let mut i = 0;
        while i < 32 {
            let relative_index = start + i * SKIP;
            pattern[i] = relative_index / 32;
            i += 1;
        }
        pattern
    }

    const MASK_SET: [u32; 32] = Self::mask_pattern_set();
    const REL_INDICES: [usize; 32] = Self::index_pattern();

    #[inline(never)]
    pub fn reset_dense(words: &mut [u32]) {
        words.chunks_exact_mut(SKIP).for_each(|chunk| {
            for i in 0..Self::BITS {
                let word_idx = Self::REL_INDICES[i];
                // TODO: safety note
                unsafe {
                    *chunk.get_unchecked_mut(word_idx) |= Self::MASK_SET[i];
                }
            }
        });

        let remainder = words.chunks_exact_mut(SKIP).into_remainder();
        for i in 0..Self::BITS {
            let word_idx = Self::REL_INDICES[i];
            if word_idx < remainder.len() {
                // TODO: safety note
                unsafe {
                    *remainder.get_unchecked_mut(word_idx) |= Self::MASK_SET[i];
                }
            } else {
                break;
            }
        }

        // restore original factor bit -- we have clobbered it, and it is the prime
        let factor_index = SKIP / 2;
        let factor_word = factor_index / Self::BITS;
        let factor_bit = factor_index % Self::BITS;
        if let Some(w) = words.get_mut(factor_word) {
            *w &= !(1 << factor_bit);
        }
    }
}
