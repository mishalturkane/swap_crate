pub fn swap(a: i32, b: i32) -> [i32; 2] {
    let (a, b) = (b, a); // built-in swap
    [a, b]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_test() {
        assert_eq!(swap(5, 9), [9, 5]);
    }
}
