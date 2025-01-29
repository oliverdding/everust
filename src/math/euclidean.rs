pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a%b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_zero() {
        let res = gcd(5, 0);
        assert_eq!(res, 5);

        let res = gcd(0, 100);
        assert_eq!(res, 100);
    }

    #[test]
    fn two_zero() {
        let res = gcd(0, 0);
        assert_eq!(res, 0);
    }

    #[test]
    fn simple() {
        let res = gcd(10, 5);
        assert_eq!(res, 5);

        let res = gcd(9, 4);
        assert_eq!(res, 1);

        let res = gcd(20, 15);
        assert_eq!(res, 5);
    }
}