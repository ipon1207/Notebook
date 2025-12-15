use std::io;

fn main() {
    println!("温度を入力してください!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let celsius: f64 = input.trim().parse().expect("有効な数値を入力してください");
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

    println!("摂氏 {} 度は華氏 {} 度です", celsius, fahrenheit);
}
