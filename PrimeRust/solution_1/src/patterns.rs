pub const fn index_pattern<const BITS: usize>(skip: usize) -> [usize; BITS] {
    let start = skip / 2;
    let mut pattern = [0; BITS];
    let mut i = 0;
    while i < BITS {
        let relative_index = start + i * skip;
        pattern[i] = relative_index / BITS;
        i += 1;
    }
    pattern
}

pub const fn modulo_pattern<const BITS: usize>(skip: usize) -> [usize; BITS] {
    let start = skip / 2;
    let mut pattern = [0; BITS];
    let mut i = 0;
    while i < BITS {
        let relative_index = start + i * skip;
        pattern[i] = relative_index % BITS;
        i += 1;
    }
    pattern
}

pub const fn modulo_pattern_sets<const BITS: usize>() -> [[usize; BITS]; BITS] {
    let first = 3;
    let mut pattern_sets = [[0; BITS]; BITS];
    let mut i = 0;
    while i < BITS {
        let skip = first + i * 2;
        pattern_sets[i] = modulo_pattern(skip);
        i += 1;
    }
    pattern_sets
}

// TODO: generic over type, possibly, to generalise to u32
pub const fn mask_pattern_sets_u8() -> [[u8; 8]; 8] {
    let mut mask_sets = [[0; 8]; 8];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 8 {
            mask_sets[i][j] = 1 << BIT_PATTERNS_U8[i][j];
            j += 1;
        }
        i += 1;
    }
    mask_sets
}

// TODO: generic over type (as above)
pub const fn mask_pattern_sets_u32() -> [[u32; 32]; 32] {
    let mut mask_sets = [[0; 32]; 32];
    let mut i = 0;
    while i < 32 {
        let mut j = 0;
        while j < 32 {
            mask_sets[i][j] = 1 << BIT_PATTERNS_U32[i][j];
            j += 1;
        }
        i += 1;
    }
    mask_sets
}

pub const BIT_PATTERNS_U8: [[usize; 8]; 8] = modulo_pattern_sets::<8>();
pub const MASK_PATTERNS_U8: [[u8; 8]; 8] = mask_pattern_sets_u8();

pub const BIT_PATTERNS_U32: [[usize; 32]; 32] = modulo_pattern_sets::<32>();
pub const MASK_PATTERNS_U32: [[u32; 32]; 32] = mask_pattern_sets_u32();

#[cfg(test)]
mod tests {
    use crate::patterns::{index_pattern, BIT_PATTERNS_U32, BIT_PATTERNS_U8, MASK_PATTERNS_U8};

    #[test]
    fn modulo_pattern_set_u8_correct() {
        assert_eq!(BIT_PATTERNS_U8[0], [1, 4, 7, 2, 5, 0, 3, 6]);
        assert_eq!(BIT_PATTERNS_U8[1], [2, 7, 4, 1, 6, 3, 0, 5]);
        assert_eq!(BIT_PATTERNS_U8[7], [0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn mask_pattern_set_u8_correct() {
        let expected: Vec<u8> = [1, 4, 7, 2, 5, 0, 3, 6].iter().map(|b| 1 << b).collect();
        assert_eq!(MASK_PATTERNS_U8[0][..], expected[..]);
    }

    #[test]
    fn modulo_pattern_set_u32_correct() {
        assert_eq!(
            BIT_PATTERNS_U32[0],
            [
                1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 0, 3,
                6, 9, 12, 15, 18, 21, 24, 27, 30
            ]
        );
        assert_eq!(
            BIT_PATTERNS_U32[1],
            [
                2, 7, 12, 17, 22, 27, 0, 5, 10, 15, 20, 25, 30, 3, 8, 13, 18, 23, 28, 1, 6, 11, 16,
                21, 26, 31, 4, 9, 14, 19, 24, 29
            ]
        );
        assert_eq!(
            BIT_PATTERNS_U32[31],
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31
            ]
        );
    }

    #[test]
    fn index_pattern_set_u8_correct() {
        assert_eq!(index_pattern::<8>(3), [0, 0, 0, 1, 1, 2, 2, 2]);
        assert_eq!(index_pattern::<8>(5), [0, 0, 1, 2, 2, 3, 4, 4]);
        assert_eq!(index_pattern::<8>(17), [1, 3, 5, 7, 9, 11, 13, 15]);
        assert_eq!(index_pattern::<8>(51), [3, 9, 15, 22, 28, 35, 41, 47,]);
    }

    #[test]
    fn index_pattern_set_u32_correct() {
        assert_eq!(
            index_pattern::<32>(3),
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2,
                2, 2, 2, 2,
            ]
        );
        assert_eq!(
            index_pattern::<32>(5),
            [
                0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4,
                4, 4, 4, 4,
            ]
        );
        assert_eq!(
            index_pattern::<32>(51),
            [
                0, 2, 3, 5, 7, 8, 10, 11, 13, 15, 16, 18, 19, 21, 23, 24, 26, 27, 29, 31, 32, 34,
                35, 37, 39, 40, 42, 43, 45, 47, 48, 50,
            ]
        )
    }
}
