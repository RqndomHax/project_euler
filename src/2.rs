fn main() {
    let mut sum: i32 = 0;
    let mut last: i32 = 1;
    let mut previous: i32 = 1;
    loop {
        println!("number = {}", last);
        if previous + last >= 4000000 {
            break;
        }
        let tmp = last;
        last += previous;
        previous = tmp;
        if last % 2 == 0 {
            sum += last;
        }
    }
    println!("sum = {}", sum);
}
