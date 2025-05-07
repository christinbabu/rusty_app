fn main() {
    let x = five();
    let y = plus_five(x);
    let sentence = "The_typed numbers are";
    let z= make_a_sentence(x, y, sentence);
    println!("The value of x is {},{},{}", x, y, z);

    let condition = false;

    let number = if condition {5} else {10};
    println!("The number is {}", number)
}

fn five() -> i32 {
    5
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn make_a_sentence(a: i32, b: i32, sen: &str) -> String {
    format!("{},{},{}", sen, a, b)
}