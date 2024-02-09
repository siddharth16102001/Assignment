fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num <= 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let test_numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    
    for num in test_numbers {
        if is_prime(num) {
            println!("{} is prime.", num);
        } else {
            println!("{} is not prime.", num);
        }
    }
}
