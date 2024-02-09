fn is_prime(num: u32) -> bool {
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
    let test_numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 40];
    
    for num in test_numbers {
        if is_prime(num) {
            println!("{} is prime.", num);
        } else {
            println!("{} is not prime.", num);
        }
    }
}
