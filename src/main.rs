fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, expected)| {
            assert_eq!(is_palindrome(*n), *expected);
        });
    }
}

fn main() {
    let number = 121;
    println!("Is {} a palindrome? {}", number, is_palindrome(number));
}
