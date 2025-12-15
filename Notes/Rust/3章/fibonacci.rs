use std::io;

fn main() {
    println!("フィボナッチ数列のn番目の値を計算します");
    println!("nを入力してください:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let n: u32 = input.trim().parse().expect("有効な整数を入力してください");

    let result = fibonacci(n);

    println!("フィボナッチ数列の{}番目の値は{}です", n, result);
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
