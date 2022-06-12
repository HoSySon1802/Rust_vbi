use std::fs;
use std::io;

fn main() {
    let contents = fs::read_to_string("textBai2.txt").expect("Something went wrong");

    println!("Nhap Tu");

    let mut input_str = String::new();
            io::stdin().read_line(&mut input_str).unwrap();

    println!("Tu Can Tim Kiem : {}", &input_str);

    let number = contents.matches(&input_str.trim()).count();
    println!("So Lan Xuat Hien : {} ", number);

}