//fn add_two(a: i32) -> i32 { a + 2 }
fn add_two(a: i32) -> i32 { a + 3 }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_add_two() { assert_eq!(4, add_two(2)); }
    #[test]
    fn is_add_two_ne() { assert_ne!(4, add_two(2)); }
    #[test]
    fn is_add_two_msg() { assert_eq!(4, add_two(2), "テストに失敗しましたわ。: {}", 4); }
}
