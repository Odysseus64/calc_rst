use std::io;

fn main(){

    println!("Enter first number: ");
    let mut first: String = String::new();
    io::stdin().read_line(&mut first)
        .expect("First выдает ошибку");

    let _get_first: f64 = match first.trim().parse(){
        Ok(num1) => {num1},
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };
    
    
    println!("Enter second number: ");
    let mut second: String = String::new();
    io::stdin().read_line(&mut second)
        .expect("Second выдает ошибку");

    let _get_second: f64 = match second.trim().parse(){
        Ok(num2) => {num2},
        Err(e) => {
            println!("Error {}", e);
            return;
        }
    };

    
    println!("Choose element [+ - * /]");
    let mut element: String = String::new();
    io::stdin().read_line(&mut element)
        .expect("Error with element") ;

    let element = element.trim();
    let symbol = match element {
        "+" => Some(_get_first + _get_second),
        "-" => Some(_get_first - _get_second),
        "*" => Some(_get_first * _get_second),
        "/" => {Some(_get_first / _get_second)}
        _ => {
            println!("Error: ");
            None
        }
    };
    if let Some (symbol) = symbol{
        println!("Result: {}", symbol);
    }
}