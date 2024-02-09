fn is_palindrome(s: &str) -> bool {
    let reversed_string: String = s.chars().rev().collect();
    s == reversed_string
}

fn main() {
    let test_string1 = "racecar";
    let test_string2 = "hello";
    
    println!("Is '{}' a palindrome? {}", test_string1, is_palindrome(test_string1));
    println!("Is '{}' a palindrome? {}", test_string2, is_palindrome(test_string2));
}
