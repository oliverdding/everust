pub fn sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort(arr, 0, arr.len() - 1);
    }
}

fn quick_sort<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let pivot = partition(arr, lo, hi);
        if lo < pivot {
            quick_sort(arr, lo, pivot - 1);
        }
        if pivot < hi {
            quick_sort(arr, pivot + 1, hi);
        }
    }
}

fn partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = lo; // index of smaller element (not included)

    for j in lo..hi {
        if arr[j] < arr[hi] {
            // everything element less than arr[hi] would be swap behind `i`
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, hi);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_single_value() {
        let mut ve1 = vec![12];

        assert_eq!(partition(&mut ve1, 0, 0), 0);

        assert_eq!(ve1, vec![12]);
    }

    #[test]
    fn test_partition_single_normal() {
        let mut ve1 = vec![4, 3, 2, 5, 1];

        assert_eq!(partition(&mut ve1, 0, 4), 0);
    }

    #[test]
    fn empty() {
        let mut ve1 = Vec::<u8>::new();

        sort(&mut ve1);

        assert_eq!(ve1, vec![]);
    }

    #[test]
    fn one() {
        let mut ve1 = vec![1];

        sort(&mut ve1);

        assert_eq!(ve1, vec![1]);
    }

    #[test]
    fn huge() {
        let mut ve1 = (0..1024).collect::<Vec<u32>>();
        ve1.reverse();
        let mut ve2 = ve1.clone();

        sort(&mut ve1);
        ve2.sort();

        assert_eq!(ve1, ve2);
    }

    #[test]
    fn normal() {
        //pre-sorted
        let mut ve1 = vec![16, 14, 77, 11, 4, 7, 3, 8, 1];
        let mut ve2 = ve1.clone();

        sort(&mut ve1);
        ve2.sort();

        assert_eq!(ve1, ve2);
    }

    #[test]
    fn repeat() {
        //pre-sorted
        let mut ve1 = vec![33, 55, 33, 55, 33, 55, 33, 55, 33, 55, 33, 55];
        let mut ve2 = ve1.clone();

        sort(&mut ve1);
        ve2.sort();

        assert_eq!(ve1, ve2);
    }

    #[test]
    fn alpha() {
        //pre-sorted
        let mut ve1 = vec!['a', 'v', 'e', 'g'];
        let mut ve2 = ve1.clone();

        sort(&mut ve1);
        ve2.sort();

        assert_eq!(ve1, ve2);
    }
}
