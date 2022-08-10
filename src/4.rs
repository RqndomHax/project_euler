fn is_palindrome(number: i32) -> bool {
    let tmp: String = number.to_string().chars().rev().collect();
    number == tmp.parse::<i32>().unwrap()
}

fn main() {
    let mut largest: i32 = 0;

    for x in 100..1000 {
        for y in 100..1000 {
            let tmp = x*y;
            if is_palindrome(tmp) {
                if largest < tmp {
                    largest = tmp;
                }
            }
        }
    }
    println!("largest palindrome = {}", largest);
}
