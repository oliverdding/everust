pub fn sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        sort(&mut arr[..mid]);
        sort(&mut arr[mid..]);
        merge(&mut arr[..], mid)
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let arr1 = arr[..mid].to_vec();
    let mut idx1: usize = 0;
    let arr2 = arr[mid..].to_vec();
    let mut idx2: usize = 0;

    let mut i = 0;
    let size = arr.len() - mid;
    while idx1 < mid && idx2 < size {
        // copy smaller element to arr
        arr[i] = if arr1[idx1] <= arr2[idx2] {
            idx1 += 1;
            arr1[idx1 - 1]
        } else {
            idx2 += 1;
            arr2[idx2 - 1]
        };
        i += 1;
    }
    arr[i..].clone_from_slice(if idx1 < mid {
        // copy the left element to current arr
        &arr1[idx1..]
    } else {
        &arr2[idx2..]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
