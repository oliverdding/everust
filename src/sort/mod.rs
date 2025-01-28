mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;

pub use bubble_sort::sort as bubble_sort;
pub use insertion_sort::sort as insertion_sort;
pub use merge_sort::sort as merge_sort;
pub use quick_sort::sort as quick_sort;
pub use selection_sort::sort as selection_sort;

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    extern crate rand;

    use rand::thread_rng;
    use rand::seq::SliceRandom;
    use test::Bencher;

    const START: u32 = 0;
    const END: u32 = 1024;

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        let mut arr = (START..=END).collect::<Vec<u32>>();
        arr.shuffle(&mut thread_rng());
        b.iter(|| bubble_sort(&mut arr.clone()));
    }

    #[bench]
    fn bench_insertion_sort(b: &mut Bencher) {
        let mut arr = (START..=END).collect::<Vec<u32>>();
        arr.shuffle(&mut thread_rng());
        b.iter(|| insertion_sort(&mut arr.clone()));
    }

    #[bench]
    fn bench_quick_sort(b: &mut Bencher) {
        let mut arr = (START..=END).collect::<Vec<u32>>();
        arr.shuffle(&mut thread_rng());
        b.iter(|| quick_sort(&mut arr.clone()));
    }

    #[bench]
    fn bench_vec_quick_sort(b: &mut Bencher) {
        let mut arr = (START..=END).collect::<Vec<u32>>();
        arr.shuffle(&mut thread_rng());
        b.iter(|| arr.clone().sort());
    }

    #[bench]
    fn bench_selection_sort(b: &mut Bencher) {
        let mut arr = (START..=END).collect::<Vec<u32>>();
        arr.shuffle(&mut thread_rng());
        b.iter(|| selection_sort(&mut arr.clone()));
    }
}
