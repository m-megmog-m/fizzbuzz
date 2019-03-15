use std::string::String;

/// メイン関数
fn main() {
    for num in 1..100 {
        println!("{}", shout(num));
    }
}

/// FizzBuzzメインロジック
fn shout(num: i32) -> String {
    if num % 15 == 0 {
        "FizzBuzz!".to_string()
    } else if num % 5 == 0 {
        "Buzz!".to_string()
    } else if num % 3 == 0 {
        "Fizz!".to_string()
    } else {
        num.to_string()
    }
}

#[cfg(test)]
mod tests {
    //! テストモジュール

    use super::*;

    /// FizzBuzzメインロジックのテスト
    #[test]
    fn it_works() {
        assert_eq!(shout(1), "1");
        assert_eq!(shout(2), "2");
        assert_eq!(shout(3), "Fizz!");
        assert_eq!(shout(4), "4");
        assert_eq!(shout(5), "Buzz!");
    }
}