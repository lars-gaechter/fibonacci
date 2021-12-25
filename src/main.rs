fn main() {
    let input = 5;
    let mut n = 0;
    let mut number1 = 0;
    let mut number2 = 1;
    let mut number3 = 0;
    let fibonacci = loop {
        number3 = number1 + number2;
        number1 = number2;
        number2 = number3;
        println!("{}", number2);
        if n == input - 1 {
            break number2;
        }
        n = n+1;
    };
    println!("{}th fibonacci number = {}", fibonacci, input)
}
