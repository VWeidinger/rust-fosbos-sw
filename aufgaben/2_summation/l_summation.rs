fn main() {
    let num = 6;
    println!("Zahl {} Summe: {}", num, summation(num));
}

//solide Lösung
fn summation(number: i32) -> i32 {
    let mut result = 0;
    for iter in 1..=number {
        result += iter;
    }
    result
}

//schicke Lösung --> müsste ihr nicht können, einfach das man es mal gesehen hat :)
fn summation_pro(number: i32) -> i32 {
    (0..=number).sum()
}