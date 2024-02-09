fn longest_common_prefix(strings: Vec<&str>) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_string = strings[0];

    'outer: for (i, c) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(ch) = string.chars().nth(i) {
                if ch != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings1 = vec!["flower", "flow", "flight"];
    let strings2 = vec!["dog", "racecar", "car"];

    println!("Longest common prefix of strings1: {}", longest_common_prefix(strings1));
    println!("Longest common prefix of strings2: {}", longest_common_prefix(strings2));
}
