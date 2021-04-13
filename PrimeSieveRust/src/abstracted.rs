use std::time::{Duration, Instant};

use primes::{
    FlagStorage, FlagStorageBitVector, FlagStorageByteVector, PrimeSieve, PrimeSieveAsync,
};
use tokio::task;

pub mod primes {
    use std::{cell::UnsafeCell, collections::HashMap, time::Duration, usize};

    use tokio::task;

    /// Validator to compare against known primes.
    /// Pulled this out into a separate struct, as it's defined
    /// `const` in C++. There are various ways to do this in Rust, including
    /// lazy_static, etc. Should be able to do the const initialisation in the future.
    pub struct PrimeValidator(HashMap<usize, usize>);
    impl Default for PrimeValidator {
        fn default() -> Self {
            let map = [
                (10, 4),   // Historical data for validating our results - the number of primes
                (100, 25), // to be found under some limit, such as 168 primes under 1000
                (1000, 168),
                (10000, 1229),
                (100000, 9592),
                (1000000, 78498),
                (10000000, 664579),
                (100000000, 5761455),
            ]
            .iter()
            .copied()
            .collect();
            PrimeValidator(map)
        }
    }
    impl PrimeValidator {
        pub fn is_valid(&self, sieve_size: usize, result: usize) -> bool {
            if let Some(&expected) = self.0.get(&sieve_size) {
                result == expected
            } else {
                false
            }
        }

        #[allow(dead_code)]
        pub fn known_results(&self) -> &HashMap<usize, usize> {
            &self.0
        }
    }

    /// Trait defining the interface to different kinds of storage, e.g.
    /// bits within bytes, a vector of bytes, etc.
    pub trait FlagStorage: Send {
        /// create new storage for given number of flags pre-initialised to all true
        fn create_true(size: usize) -> Self;

        /// reset all flags at indices starting at `start` with a stride of `stride`
        fn reset_flags(&mut self, start: usize, skip: usize);

        /// get a specific flag
        fn get(&self, index: usize) -> bool;
    }

    /// Storage using a simple vector of bytes.
    /// Doing the same with bools is equivalent, as bools are currently
    /// represented as bytes in Rust. However, this is not guaranteed to
    /// remain so for all time. To ensure consistent memory use in the future,
    /// we're explicitly using bytes (u8) here.
    pub struct FlagStorageByteVector(Vec<u8>);
    impl FlagStorage for FlagStorageByteVector {
        fn create_true(size: usize) -> Self {
            FlagStorageByteVector(vec![1; size])
        }

        // bounds checks are elided since we're runing up to .len()
        fn reset_flags(&mut self, start: usize, skip: usize) {
            let mut i = start;
            while i < self.0.len() {
                self.0[i] = 0;
                i += skip;
            }
        }

        fn get(&self, index: usize) -> bool {
            if let Some(val) = self.0.get(index) {
                *val == 1
            } else {
                false
            }
        }
    }

    struct UnsafeMutPtr {
        ptr: *mut u8,
    }
    impl UnsafeMutPtr {
        fn new(ptr: *mut u8) -> Self {
            UnsafeMutPtr { ptr }
        }
    }
    unsafe impl Send for UnsafeMutPtr {}

    fn ceiling(numerator: isize, denominator: isize) -> isize {
        (numerator + denominator - 1) / denominator
    }

    /// Storage using a vector of bytes, but addressing individual bits within each
    pub struct FlagStorageBitVector {
        words: Vec<u8>,
        length_bits: usize,
    }

    // darn it -- can't use async in traits yet.
    impl FlagStorageBitVector {
        #[inline(always)]
        #[allow(dead_code)]
        async fn reset_flags_async_old(&mut self, start: usize, skip: usize) {
            let mut i = start;
            while i < self.words.len() * U8_BITS {
                let word_idx = i / U8_BITS;
                let bit_idx = i % U8_BITS;
                // unsafe get_mut_unchecked is superfluous here -- the compiler
                // seems to know that we're within bounds, so it yields no performance
                // benefit.
                unsafe {
                    *self.words.get_unchecked_mut(word_idx) &= !(1 << bit_idx);
                }
                i += skip;
            }
        }
        #[inline(always)]
        async fn reset_flags_async(&mut self, start: usize, skip: usize) {
            let first_word = start / U8_BITS;
            let words_remaining = self.words.len() - first_word;
            let words_chunks = words_remaining / 4;
            let word_starts = [
                first_word,
                first_word + words_chunks,
                first_word + words_chunks * 2,
                first_word + words_chunks * 3,
            ];

            let ptr0 = UnsafeMutPtr::new(self.words.as_mut_ptr());
            let ptr1 = UnsafeMutPtr::new(self.words.as_mut_ptr());
            let ptr2 = UnsafeMutPtr::new(self.words.as_mut_ptr());
            let ptr3 = UnsafeMutPtr::new(self.words.as_mut_ptr());

            // spin up other tasks to handle chunks 1,2,3
            // let handle1 = task::spawn_blocking(move || {
            //     Self::reset_flags_internal(ptr1, word_starts[1], words_chunks, start, skip)
            // });
            // let handle2 = task::spawn_blocking(move || {
            //     Self::reset_flags_internal(ptr2, word_starts[2], words_chunks,  start,skip)
            // });
            // let handle3 = task::spawn_blocking(move || {
            //     Self::reset_flags_internal(ptr3, word_starts[3], words_chunks,  start,skip)
            // });

            // and process chunk 0 right here
            Self::reset_flags_internal(ptr0, word_starts[0], words_chunks,  start, skip).await;
            Self::reset_flags_internal(ptr1, word_starts[1], words_chunks,  start, skip).await;
            Self::reset_flags_internal(ptr2, word_starts[2], words_chunks,  start, skip).await;
            Self::reset_flags_internal(ptr3, word_starts[3], words_chunks,  start, skip).await;

            // let _ = handle1.await.unwrap();
            // let _ = handle2.await.unwrap();
            // let _ = handle3.await.unwrap();
        }

        #[inline(always)]
        async fn reset_flags_internal(array: UnsafeMutPtr, start_word: usize, word_chunks: usize, start: usize, skip: usize) {
            // find first index, then run until we're out of words
            let word_idx = start_word * U8_BITS;
            let rel_start = word_idx as isize - start as isize;
            let num_skips_ceil = ceiling(rel_start, skip as isize);
            let actual_start = num_skips_ceil * skip as isize + start as isize;
            
            let end_index = (start_word + word_chunks) * 8; // check this
            let mut i = actual_start;
            let ptr = array.ptr;
            while i < end_index as isize {
                let word_idx = i / U8_BITS as isize;
                let bit_idx = i % U8_BITS as isize;
                unsafe {
                    *ptr.offset(word_idx) &= !(1 << bit_idx);
                }
                i += skip as isize;
            }
        }
    }

    const U8_BITS: usize = 8;
    impl FlagStorage for FlagStorageBitVector {
        fn create_true(size: usize) -> Self {
            let num_words = size / U8_BITS + (size % U8_BITS).min(1);
            FlagStorageBitVector {
                words: vec![0xff; num_words],
                length_bits: size,
            }
        }

        fn reset_flags(&mut self, start: usize, skip: usize) {
            let mut i = start;
            while i < self.words.len() * U8_BITS {
                let word_idx = i / U8_BITS;
                let bit_idx = i % U8_BITS;
                // unsafe get_mut_unchecked is superfluous here -- the compiler
                // seems to know that we're within bounds, so it yields no performance
                // benefit.
                *self.words.get_mut(word_idx).unwrap() &= !(1 << bit_idx);
                i += skip;
            }
        }

        fn get(&self, index: usize) -> bool {
            if index >= self.length_bits {
                return false;
            }
            let word = self.words.get(index / U8_BITS).unwrap();
            *word & (1 << (index % U8_BITS)) != 0
        }
    }

    pub struct PrimeSieve<T: FlagStorage> {
        sieve_size: usize,
        flags: T,
    }

    impl<T> PrimeSieve<T>
    where
        T: FlagStorage,
    {
        pub fn new(sieve_size: usize) -> Self {
            let num_flags = sieve_size / 2 + 1;
            PrimeSieve {
                sieve_size,
                flags: T::create_true(num_flags),
            }
        }

        fn is_num_flagged(&self, number: usize) -> bool {
            if number % 2 == 0 {
                return false;
            }
            let index = number / 2;
            self.flags.get(index)
        }

        // count number of primes (not optimal, but doesn't need to be)
        pub fn count_primes(&self) -> usize {
            (1..self.sieve_size)
                .filter(|v| self.is_num_flagged(*v))
                .count()
        }

        // calculate the primes up to the specified limit
        #[inline(always)]
        pub fn run_sieve(&mut self) {
            let mut factor = 3;
            let q = (self.sieve_size as f32).sqrt() as usize;

            // note: need to check up to and including q, otherwise we
            // fail to catch cases like sieve_size = 1000
            while factor <= q {
                // find next factor - next still-flagged number
                factor = (factor..self.sieve_size)
                    .find(|n| self.is_num_flagged(*n))
                    .unwrap();

                // reset flags starting at `start`, every `factor`'th flag
                let start = factor * 3 / 2;
                let skip = factor;
                self.flags.reset_flags(start, skip);

                factor += 2;
            }
        }

        pub fn print_results(
            &self,
            show_results: bool,
            duration: Duration,
            passes: usize,
            validator: &PrimeValidator,
        ) {
            if show_results {
                print!("2,");
                for num in (3..self.sieve_size).filter(|n| self.is_num_flagged(*n)) {
                    print!("{},", num);
                }
                print!("\n");
            }

            let count = self.count_primes();

            println!(
                "Passes: {}, Time: {}, Avg: {}, Limit: {}, Count: {}, Valid: {}",
                passes,
                duration.as_secs_f32(),
                duration.as_secs_f32() / passes as f32,
                self.sieve_size,
                count,
                validator.is_valid(self.sieve_size, self.count_primes())
            );
        }
    }

    pub struct PrimeSieveAsync {
        sieve_size: usize,
        flags: FlagStorageBitVector,
    }

    impl PrimeSieveAsync {
        pub fn new(sieve_size: usize) -> Self {
            let num_flags = sieve_size / 2 + 1;
            PrimeSieveAsync {
                sieve_size,
                flags: FlagStorageBitVector::create_true(num_flags),
            }
        }

        fn is_num_flagged(&self, number: usize) -> bool {
            if number % 2 == 0 {
                return false;
            }
            let index = number / 2;
            self.flags.get(index)
        }

        // count number of primes (not optimal, but doesn't need to be)
        pub fn count_primes(&self) -> usize {
            (1..self.sieve_size)
                .filter(|v| self.is_num_flagged(*v))
                .count()
        }

        // calculate the primes up to the specified limit
        #[inline(always)]
        pub async fn run_sieve(&mut self) {
            let mut factor = 3;
            let q = (self.sieve_size as f32).sqrt() as usize;

            // note: need to check up to and including q, otherwise we
            // fail to catch cases like sieve_size = 1000
            while factor <= q {
                // find next factor - next still-flagged number
                factor = (factor..self.sieve_size)
                    .find(|n| self.is_num_flagged(*n))
                    .unwrap();

                // reset flags starting at `start`, every `factor`'th flag
                let start = factor * 3 / 2;
                let skip = factor;
                self.flags.reset_flags_async(start, skip).await;

                factor += 2;
            }
        }

        pub fn print_results(
            &self,
            show_results: bool,
            duration: Duration,
            passes: usize,
            validator: &PrimeValidator,
        ) {
            if show_results {
                print!("2,");
                for num in (3..self.sieve_size).filter(|n| self.is_num_flagged(*n)) {
                    print!("{},", num);
                }
                print!("\n");
            }

            let count = self.count_primes();

            println!(
                "Passes: {}, Time: {}, Avg: {}, Limit: {}, Count: {}, Valid: {}",
                passes,
                duration.as_secs_f32(),
                duration.as_secs_f32() / passes as f32,
                self.sieve_size,
                count,
                validator.is_valid(self.sieve_size, self.count_primes())
            );
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    print!("Bit storage:   ");
    run_implementation::<FlagStorageBitVector>();

    print!("Bit async:     ");
    run_implementation_async().await;
    //run_parallel::<FlagStorageBitVector>();

    print!("Byte storage:  ");
    run_implementation::<FlagStorageByteVector>();
    run_parallel::<FlagStorageByteVector>();
    run_parallel_tokio::<FlagStorageByteVector>().await;
    run_parallel_tokio_local_merge::<FlagStorageByteVector>().await;
    run_parallel_tokio_local_merge_task::<FlagStorageByteVector>().await;

    Ok(())
}

const RUN_DURATION_SECONDS: u64 = 5;
const SIEVE_SIZE: usize = 1000000;

fn run_implementation<T: FlagStorage>() {
    let mut passes = 0;
    let mut prime_sieve = None;

    let start_time = Instant::now();
    let run_duration = Duration::from_secs(RUN_DURATION_SECONDS);
    while (Instant::now() - start_time) < run_duration {
        let mut sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
        sieve.run_sieve();
        prime_sieve.replace(sieve);
        passes += 1;
    }
    let end_time = Instant::now();

    if let Some(sieve) = prime_sieve {
        sieve.print_results(
            false,
            end_time - start_time,
            passes,
            &primes::PrimeValidator::default(),
        );
    }
}

async fn run_implementation_async() {
    let mut passes = 0;
    let mut prime_sieve = None;

    let start_time = Instant::now();
    let run_duration = Duration::from_secs(RUN_DURATION_SECONDS);
    while (Instant::now() - start_time) < run_duration {
        let mut sieve: PrimeSieveAsync = PrimeSieveAsync::new(SIEVE_SIZE);
        sieve.run_sieve().await;
        prime_sieve.replace(sieve);
        passes += 1;
    }
    let end_time = Instant::now();

    if let Some(sieve) = prime_sieve {
        sieve.print_results(
            false,
            end_time - start_time,
            passes,
            &primes::PrimeValidator::default(),
        );
    }
}

fn run_parallel<T: 'static + FlagStorage>() {
    use std::thread;

    let start_time = Instant::now();
    let run_duration = Duration::from_secs(RUN_DURATION_SECONDS);
    let num_threads = num_cpus::get();
    //let num_threads = 1;

    let mut threads = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut last_sieve = None;
            let mut local_passes = 0;
            while (Instant::now() - start_time) < run_duration {
                let mut sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
                sieve.run_sieve();
                last_sieve.replace(sieve);
                local_passes += 1;
            }
            // validate result is correct and return number of passes on this thread
            // let validator = primes::PrimeValidator::default();
            // assert!(validator.is_valid(SIEVE_SIZE, last_sieve.unwrap().count_primes()));
            (local_passes, last_sieve.unwrap())
        });
        threads.push(handle);
    }

    // wait for all threads to terminate, then take the end time
    let results: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
    let end_time = Instant::now();

    // sum total passes, and grab the final sieve from the first thread for reporting
    let total_passes: usize = results.iter().map(|r| r.0).sum();
    let check_sieve = &results.first().unwrap().1;

    check_sieve.print_results(
        false,
        end_time - start_time,
        total_passes,
        &primes::PrimeValidator::default(),
    );
}

async fn run_parallel_tokio<T: 'static + FlagStorage>() {
    let start_time = Instant::now();
    let run_duration = Duration::from_secs(RUN_DURATION_SECONDS);
    let num_tasks = num_cpus::get();
    //let num_threads = 1;

    let mut tasks = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        let handle = tokio::spawn(async move {
            let mut last_sieve = None;
            let mut local_passes = 0;
            while (Instant::now() - start_time) < run_duration {
                let mut sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
                sieve.run_sieve();
                last_sieve.replace(sieve);
                local_passes += 1;
            }
            // validate result is correct and return number of passes on this thread
            // let validator = primes::PrimeValidator::default();
            // assert!(validator.is_valid(SIEVE_SIZE, last_sieve.unwrap().count_primes()));
            (local_passes, last_sieve.unwrap())
        });
        tasks.push(handle);
    }

    // wait for all threads to terminate, then take the end time
    let mut results = Vec::with_capacity(num_tasks);
    for task in tasks {
        results.push(task.await.unwrap());
    }
    let end_time = Instant::now();

    // sum total passes, and grab the final sieve from the first thread for reporting
    let total_passes: usize = results.iter().map(|r| r.0).sum();
    let check_sieve = &results.first().unwrap().1;

    check_sieve.print_results(
        false,
        end_time - start_time,
        total_passes,
        &primes::PrimeValidator::default(),
    );
}

// similar to the C++ _PAR implementation -- local tasks on a thread pool
async fn run_parallel_tokio_local_merge<T: 'static + FlagStorage>() {
    let start_time = Instant::now();
    let run_duration = Duration::from_secs(RUN_DURATION_SECONDS);
    let num_tasks = num_cpus::get();
    //let num_threads = 1;

    let mut total_passes = 0;
    while (Instant::now() - start_time) < run_duration {
        let mut tasks = Vec::with_capacity(num_tasks);
        for _ in 0..num_tasks {
            let handle = tokio::spawn(async move {
                let mut sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
                sieve.run_sieve();
                1 // single pass
            });
            tasks.push(handle);
        }

        for task in tasks {
            let passes = task.await.unwrap();
            total_passes += passes;
        }
    }
    let end_time = Instant::now();

    let mut check_sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
    check_sieve.run_sieve();
    check_sieve.print_results(
        false,
        end_time - start_time,
        total_passes,
        &primes::PrimeValidator::default(),
    );
}

// similar to the C++ _PAR implementation -- local tasks on a thread pool
// using lightweight tasks here instead; using spawn_blocking as the tasks
// are, well, blocking!
async fn run_parallel_tokio_local_merge_task<T: 'static + FlagStorage>() {
    let start_time = Instant::now();
    let run_duration = Duration::from_secs(RUN_DURATION_SECONDS);
    let num_tasks = num_cpus::get();
    //let num_threads = 1;

    let mut total_passes = 0;
    while (Instant::now() - start_time) < run_duration {
        let mut tasks = Vec::with_capacity(num_tasks);
        for _ in 0..num_tasks {
            // note: using tokio::task here
            let handle = task::spawn_blocking(|| {
                let mut sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
                sieve.run_sieve();
                1 // single pass
            });
            tasks.push(handle);
        }

        for task in tasks {
            let passes = task.await.unwrap();
            total_passes += passes;
        }
    }
    let end_time = Instant::now();

    let mut check_sieve: PrimeSieve<T> = primes::PrimeSieve::new(SIEVE_SIZE);
    check_sieve.run_sieve();
    check_sieve.print_results(
        false,
        end_time - start_time,
        total_passes,
        &primes::PrimeValidator::default(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes::{
        FlagStorage, FlagStorageBitVector, FlagStorageByteVector, PrimeSieve, PrimeValidator,
    };

    #[test]
    fn sieve_known_correct_bits_async() {
        let validator = PrimeValidator::default();
        let mut sizes: Vec<_> = validator.known_results().keys().copied().collect();
        sizes.sort();
        for sieve_size in &sizes {
            let mut sieve: PrimeSieveAsync = PrimeSieveAsync::new(*sieve_size);
            tokio_test::block_on(sieve.run_sieve());
            let expected_primes = validator.known_results().get(sieve_size).unwrap();
            assert_eq!(
                *expected_primes,
                sieve.count_primes(),
                "wrong number of primes for sieve = {}",
                sieve_size
            );
        }
    }

    #[test]
    fn sieve_known_correct_bits() {
        sieve_known_correct::<FlagStorageBitVector>();
    }

    #[test]
    fn sieve_known_correct_bytes() {
        sieve_known_correct::<FlagStorageByteVector>();
    }

    fn sieve_known_correct<T: FlagStorage>() {
        let validator = PrimeValidator::default();
        for (sieve_size, expected_primes) in validator.known_results().iter() {
            let mut sieve: PrimeSieve<T> = primes::PrimeSieve::new(*sieve_size);
            sieve.run_sieve();
            assert_eq!(
                *expected_primes,
                sieve.count_primes(),
                "wrong number of primes for sieve = {}",
                sieve_size
            );
        }
    }
}
