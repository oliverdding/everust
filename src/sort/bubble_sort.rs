pub fn sort<T: Ord>(arr: &mut [T]) {
    (0..arr.len()).for_each(|i|
        (0..arr.len() - 1 - i).for_each(|j|
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        )
    );
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
