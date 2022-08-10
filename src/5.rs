fn is_divisible(number: i32) -> bool {
    for i in 1..21 {
        if number % i != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut smallest: i32 = 0;

    loop {
        smallest += 1;
        if is_divisible(smallest) {
            break;
        }
    }

    println!("smallest = {}", smallest);
}
