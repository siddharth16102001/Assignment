// Method 1: Using iterator and collect
fn reverse_string_method1(s: &str) -> String {
    s.chars().rev().collect()
}

// Method 2: In-place reversal using a mutable character array
fn reverse_string_method2(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        chars.swap(i, len - 1 - i);
    }
    chars.iter().collect()
}

fn main() {
    let input_string = "Hello, world!";
    println!("Original string: {}", input_string);
    
    let reversed_string1 = reverse_string_method1(input_string);
    println!("Reversed string (Method 1): {}", reversed_string1);
    
    let reversed_string2 = reverse_string_method2(input_string);
    println!("Reversed string (Method 2): {}", reversed_string2);
}
