use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let numbers: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("请输入一串数字：")).collect();
    let num_a = numbers[0];
    let num_b = numbers[1];
    let sum = num_a + num_b;
    println!("两数之和为:{}\n", sum);
}
