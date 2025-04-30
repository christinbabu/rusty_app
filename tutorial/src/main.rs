fn main () {
    let x = five();
    let y = plus_five(x);
    println!("The value of x is {x},{y}");
}

fn five() -> i32 {
    5
}

fn plus_five(x:i32) ->i32 {
    x + 5
}