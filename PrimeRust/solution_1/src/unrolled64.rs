use crate::{
    flag_storage::FlagStorage,
    patterns::{index_pattern, MASK_PATTERNS_U64},
};

pub struct FlagStorageUnrolledBits64 {
    words: Vec<u64>,
    length_bits: usize,
}

impl FlagStorageUnrolledBits64 {
    const BITS: usize = u64::BITS as usize;

    // TODO: consider inlining
    #[inline(never)]
    fn reset_flags_sparse<const EQUIVALENT_SKIP: usize>(&mut self, skip: usize) {
        let mask_set_index = ((EQUIVALENT_SKIP / 2) - 1) % Self::BITS;
        let mask_set = MASK_PATTERNS_U64[mask_set_index];

        let rel_indices = index_pattern::<64>(skip);

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

impl FlagStorage for FlagStorageUnrolledBits64 {
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
            3 => ResetterDenseU64::<3>::reset_dense(&mut self.words),
            5 => ResetterDenseU64::<5>::reset_dense(&mut self.words),
            7 => ResetterDenseU64::<7>::reset_dense(&mut self.words),
            9 => ResetterDenseU64::<9>::reset_dense(&mut self.words),
            11 => ResetterDenseU64::<11>::reset_dense(&mut self.words),
            13 => ResetterDenseU64::<13>::reset_dense(&mut self.words),
            15 => ResetterDenseU64::<15>::reset_dense(&mut self.words),
            17 => ResetterDenseU64::<17>::reset_dense(&mut self.words),
            19 => ResetterDenseU64::<19>::reset_dense(&mut self.words),
            21 => ResetterDenseU64::<21>::reset_dense(&mut self.words),
            23 => ResetterDenseU64::<23>::reset_dense(&mut self.words),
            25 => ResetterDenseU64::<25>::reset_dense(&mut self.words),
            27 => ResetterDenseU64::<27>::reset_dense(&mut self.words),
            29 => ResetterDenseU64::<29>::reset_dense(&mut self.words),
            31 => ResetterDenseU64::<31>::reset_dense(&mut self.words),
            33 => ResetterDenseU64::<33>::reset_dense(&mut self.words),
            35 => ResetterDenseU64::<35>::reset_dense(&mut self.words),
            37 => ResetterDenseU64::<37>::reset_dense(&mut self.words),
            39 => ResetterDenseU64::<39>::reset_dense(&mut self.words),
            41 => ResetterDenseU64::<41>::reset_dense(&mut self.words),
            43 => ResetterDenseU64::<43>::reset_dense(&mut self.words),
            45 => ResetterDenseU64::<45>::reset_dense(&mut self.words),
            47 => ResetterDenseU64::<47>::reset_dense(&mut self.words),
            49 => ResetterDenseU64::<49>::reset_dense(&mut self.words),
            51 => ResetterDenseU64::<51>::reset_dense(&mut self.words),
            53 => ResetterDenseU64::<53>::reset_dense(&mut self.words),
            55 => ResetterDenseU64::<55>::reset_dense(&mut self.words),
            57 => ResetterDenseU64::<57>::reset_dense(&mut self.words),
            59 => ResetterDenseU64::<59>::reset_dense(&mut self.words),
            61 => ResetterDenseU64::<61>::reset_dense(&mut self.words),
            63 => ResetterDenseU64::<63>::reset_dense(&mut self.words),
            65 => ResetterDenseU64::<65>::reset_dense(&mut self.words),
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
                32 => self.reset_flags_sparse::<{ (32 + 1) * 2 + 1 }>(skip),
                33 => self.reset_flags_sparse::<{ (33 + 1) * 2 + 1 }>(skip),
                34 => self.reset_flags_sparse::<{ (34 + 1) * 2 + 1 }>(skip),
                35 => self.reset_flags_sparse::<{ (35 + 1) * 2 + 1 }>(skip),
                36 => self.reset_flags_sparse::<{ (36 + 1) * 2 + 1 }>(skip),
                37 => self.reset_flags_sparse::<{ (37 + 1) * 2 + 1 }>(skip),
                38 => self.reset_flags_sparse::<{ (38 + 1) * 2 + 1 }>(skip),
                39 => self.reset_flags_sparse::<{ (39 + 1) * 2 + 1 }>(skip),
                40 => self.reset_flags_sparse::<{ (40 + 1) * 2 + 1 }>(skip),
                41 => self.reset_flags_sparse::<{ (41 + 1) * 2 + 1 }>(skip),
                42 => self.reset_flags_sparse::<{ (42 + 1) * 2 + 1 }>(skip),
                43 => self.reset_flags_sparse::<{ (43 + 1) * 2 + 1 }>(skip),
                44 => self.reset_flags_sparse::<{ (44 + 1) * 2 + 1 }>(skip),
                45 => self.reset_flags_sparse::<{ (45 + 1) * 2 + 1 }>(skip),
                46 => self.reset_flags_sparse::<{ (46 + 1) * 2 + 1 }>(skip),
                47 => self.reset_flags_sparse::<{ (47 + 1) * 2 + 1 }>(skip),
                48 => self.reset_flags_sparse::<{ (48 + 1) * 2 + 1 }>(skip),
                49 => self.reset_flags_sparse::<{ (49 + 1) * 2 + 1 }>(skip),
                50 => self.reset_flags_sparse::<{ (50 + 1) * 2 + 1 }>(skip),
                51 => self.reset_flags_sparse::<{ (51 + 1) * 2 + 1 }>(skip),
                52 => self.reset_flags_sparse::<{ (52 + 1) * 2 + 1 }>(skip),
                53 => self.reset_flags_sparse::<{ (53 + 1) * 2 + 1 }>(skip),
                54 => self.reset_flags_sparse::<{ (54 + 1) * 2 + 1 }>(skip),
                55 => self.reset_flags_sparse::<{ (55 + 1) * 2 + 1 }>(skip),
                56 => self.reset_flags_sparse::<{ (56 + 1) * 2 + 1 }>(skip),
                57 => self.reset_flags_sparse::<{ (57 + 1) * 2 + 1 }>(skip),
                58 => self.reset_flags_sparse::<{ (58 + 1) * 2 + 1 }>(skip),
                59 => self.reset_flags_sparse::<{ (59 + 1) * 2 + 1 }>(skip),
                60 => self.reset_flags_sparse::<{ (60 + 1) * 2 + 1 }>(skip),
                61 => self.reset_flags_sparse::<{ (61 + 1) * 2 + 1 }>(skip),
                62 => self.reset_flags_sparse::<{ (62 + 1) * 2 + 1 }>(skip),
                63 => self.reset_flags_sparse::<{ (63 + 1) * 2 + 1 }>(skip),
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

struct ResetterDenseU64<const SKIP: usize>();
impl<const SKIP: usize> ResetterDenseU64<SKIP> {
    const BITS: usize = 64;
    const MASK_SET: [u64; 64] = crate::patterns::mask_pattern_set_u64(SKIP);
    const REL_INDICES: [usize; 64] = crate::patterns::index_pattern(SKIP);

    #[inline(always)]
    pub fn reset_dense(words: &mut [u64]) {
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
