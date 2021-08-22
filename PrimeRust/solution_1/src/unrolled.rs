use crate::{flag_storage::FlagStorage, patterns::{MASK_PATTERNS_U8, index_pattern}};


pub struct FlagStorageUnrolledBits8 {
    words: Vec<u8>,
    length_bits: usize,
}

impl FlagStorageUnrolledBits8 {
    const BITS: usize = u8::BITS as usize;

    fn reset_flags_sparse(&mut self, skip: usize) {
        let mask_set_index = ((skip / 2) - 1) % Self::BITS;
        let mask_set = MASK_PATTERNS_U8[mask_set_index];

        let rel_indices = index_pattern::<8>(skip);

        self.words.chunks_exact_mut(skip).for_each(|chunk| {
            for i in 0..Self::BITS {
                let word_idx = rel_indices[i];
                chunk[word_idx] |= mask_set[i];
            }
        });

        let remainder = self.words.chunks_exact_mut(skip).into_remainder();
        for i in 0..Self::BITS {
            let word_idx = rel_indices[i];
            if word_idx < remainder.len() {
                remainder[word_idx] |= mask_set[i];
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

    fn reset_flags_dense<const SKIP: usize>(&mut self) {
        let mask_set_index = ((SKIP / 2) - 1) % Self::BITS;
        let mask_set = MASK_PATTERNS_U8[mask_set_index];

        let rel_indices = index_pattern::<8>(SKIP);

        self.words.chunks_exact_mut(SKIP).for_each(|chunk| {
            for i in 0..Self::BITS {
                let word_idx = rel_indices[i];
                chunk[word_idx] |= mask_set[i];
            }
        });

        let remainder = self.words.chunks_exact_mut(SKIP).into_remainder();
        for i in 0..Self::BITS {
            let word_idx = rel_indices[i];
            if word_idx < remainder.len() {
                remainder[word_idx] |= mask_set[i];
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

    fn print(&self, limit: usize) {
        println!("Storage----");
        for (i, w) in self.words.iter().take(limit).enumerate() {
            println!("{:6}: {}", i, format_bits(*w));
        }
        let last_idx = self.words.len() - 1;
        let last_word = self.words[last_idx];
        println!("{:6}: {}", last_idx, format_bits(last_word));
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

    fn reset_flags(&mut self, skip: usize) {
        // call into dispatcher
        // TODO: autogenerate match_reset_dispatch!(self, skip, 19, reset_flags_dense, reset_flags_sparse);
        match skip {
            3 => self.reset_flags_dense::<3>(),
            5 => self.reset_flags_dense::<5>(),
            7 => self.reset_flags_dense::<7>(),
            9 => self.reset_flags_dense::<9>(),
            11 => self.reset_flags_dense::<11>(),
            13 => self.reset_flags_dense::<13>(),
            15 => self.reset_flags_dense::<15>(),
            17 => self.reset_flags_dense::<17>(),
            19 => self.reset_flags_dense::<19>(),
            _ => self.reset_flags_sparse(skip)
        };
    }

    fn get(&self, index: usize) -> bool {
        if index >= self.length_bits {
            return false;
        }
        let word = self.words.get(index / Self::BITS).unwrap();
        *word & (1 << (index % Self::BITS)) == 0
    }
}

fn format_bits(val: u8) -> String {
    let bits = (0..8)
        .map(|b| (val & (1 << b)) >> b)
        .map(|b| format!("{} ", b));
    bits.collect()
}

pub fn self_test() {
    let size = 512;
    self_test_specific(3, size);
    self_test_specific(5, size);
    self_test_specific(7, size);
    self_test_specific(63, size);
    self_test_specific(1001, size);
    self_test_specific(2000, size);
}

fn self_test_specific(skip: usize, size: usize) {
    let mut storage = FlagStorageUnrolledBits8::create_true(size);
    let lim = 16;
    println!("Testing with skip = {}", skip);
    storage.reset_flags(skip);
    storage.print(lim);
}
