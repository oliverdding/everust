use std::cmp::Ordering;

pub fn search<T: Ord>(arr: &[T], target: &T) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        match target.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => {
                left = match mid.checked_add(1) {
                    Some(x) => x,
                    None => break,
                }
            }
            Ordering::Greater => left = mid.checked_add(1).ok_or(usize::MAX)?,
        };
    }
    if left != 0 && target.eq(&arr[left - 1]) {
        Ok(left - 1)
    } else {
        Err(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let res = search(&[], &"a");
        assert_eq!(res, Err(0));
    }

    #[test]
    fn one() {
        let res = search(&["a"], &"a");
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn string_array() {
        let res = search(&["a", "b", "c", "d", "e", "f"], &"d");
        assert_eq!(res, Ok(3));
    }

    #[test]
    fn one_repeat() {
        let res = search(&[1, 1, 1], &1);
        assert_eq!(res, Ok(2));
    }

    #[test]
    fn repeat() {
        let res = search(&[1, 1, 1, 2, 3, 4, 4, 4, 6, 6], &1);
        assert_eq!(res, Ok(2));
        let res = search(&[1, 1, 1, 2, 3, 4, 4, 4, 6, 6], &4);
        assert_eq!(res, Ok(7));
        let res = search(&[1, 1, 1, 2, 3, 4, 4, 4, 6, 6], &6);
        assert_eq!(res, Ok(9));
    }

    #[test]
    fn int_array() {
        let res = search(&[1, 2, 3, 4], &4);
        assert_eq!(res, Ok(3));

        let res = search(&[1, 2, 3, 4], &3);
        assert_eq!(res, Ok(2));

        let res = search(&[1, 2, 3, 4], &2);
        assert_eq!(res, Ok(1));

        let res = search(&[1, 2, 3, 4], &1);
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn not_found() {
        let res = search(&[1, 2, 3, 4], &5);
        assert_eq!(res, Err(4));

        let res = search(&[1, 2, 3, 4], &0);
        assert_eq!(res, Err(0));
    }
}
