use std::io;
fn main(){
    println!("请输入你的年龄：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let age:i32=input.trim().parse().expect("请输入一串数字：");
    let msg= if age < 18 {
        "你还未成年！"
    } else {
        "你已经成年了！"
    };
    println!("{}", msg);
    println!("你的年龄是：{}", age);
}