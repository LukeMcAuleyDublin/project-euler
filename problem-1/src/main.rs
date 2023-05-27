fn main() {
    let target: i32 = generate_numbers();
    println!("Answer to problem 1: {}", target);
}

fn generate_numbers() -> i32 {
    let mut sum: i32 = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
