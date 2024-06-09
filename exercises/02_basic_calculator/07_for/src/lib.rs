// Rewrite the factorial function using a `for` loop.
pub fn factorial(n: u32) -> u32 {
    let mut f: u32 = n;
    let mut c: u32 = n;

    if f == 1 {
        return f;
    } else if f == 0 {
        return f + 1;
    } else {
        for i in 1..n {
            f = f * (c - 1);
            c = c - 1;
        }
        return f;
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
