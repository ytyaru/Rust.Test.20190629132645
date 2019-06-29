pub struct Guess {
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 { panic!("0〜100までの整数でなければならない。: {}", value); }
        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "0〜100までの整数でなければならない。: 199")]
//    #[should_panic(expected = "0〜100までの整数でなければならない。: 200")]
    fn greater_than_100() { Guess::new(200); }
}
