fn is_prime(divider: u64) -> bool {
    if divider > 3 && divider % 2 != 0 && divider % 3 != 0 {
        return false;
    }
    true
}

fn get_next_prime(divider: u64) -> u64 {
    if !is_prime(divider) {
        get_next_prime(divider + 1);
    }
    divider
}

fn main() {
    let mut remainder: u64 = 600851475143;
    let mut divider: u64 = 2;
    
    loop {
        if remainder % divider != 0 {
            divider = get_next_prime(divider + 1);
            continue;
        }
        remainder = remainder / divider;
        println!("prime number = {}", divider);
        divider = 2;
        if remainder == 1 {
            break;
        }
    }
}
