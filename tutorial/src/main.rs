fn main() {
    let a = 10;
    let b = 20;
    addition(a,b);
    multiplication(a, b);
}

fn addition(num1: i32, num2: i32) {
    let sum:i128 = (num1 + num2) as i128;
    println!("The sum of Two numbers:{sum}");
}

fn multiplication(num1: i32, num2: i32) {
    let product:i128 = (num1*num2) as i128;
    print!("The product of two numbers:{product}")
}