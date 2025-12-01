pub fn swap<T: Copy>(a: T, b: T) -> [T; 2] {
    let (a, b) = (b, a); // built-in swap
    [a, b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_test_integers() {
        assert_eq!(swap(5, 9), [9, 5]);
    }

    #[test]
    fn swap_test_floats() {
        assert_eq!(swap(1.2, 3.4), [3.4, 1.2]);
    }

    #[test]
    fn swap_test_chars() {
        assert_eq!(swap('a', 'z'), ['z', 'a']);
    }
    #[test]
    fn swap_test_strings() {
        assert_eq!(swap("hello", "world"), ["world", "hello"]);
    }
}


