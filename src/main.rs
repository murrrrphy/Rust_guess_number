use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("请输入数字");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("读取失败");
        println!("输入的数字为: {}", number);
        let number: u32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("请输入正确的数字");
                continue;
            }
        };
        match number.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("对了");
                break;
            }
        }
    }
}
