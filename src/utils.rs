use std::io::stdin;

use ansi_term::Colour;

pub fn get_input() -> Result<(f64, f64), std::num::ParseFloatError> {
    let mut num1 = String::new();
    println!("Enter the first operand");
    stdin()
        .read_line(&mut num1)
        .expect("Failed to read the line");
    let num1: f64 = num1.trim().parse()?;

    let mut num2 = String::new();
    println!("Enter the second operand");
    stdin()
        .read_line(&mut num2)
        .expect("Failed to read the line");
    let num2: f64 = num2.trim().parse()?;

    Ok((num1, num2))
}

pub fn get_vector_input() -> Result<(f64, f64, f64, f64), std::num::ParseFloatError> {
    let mut num1 = String::new();
    println!("Enter the first component for the first 2D Vector");
    stdin()
        .read_line(&mut num1)
        .expect("Failed to read the line");
    let num1: f64 = num1.trim().parse()?;

    let mut num2 = String::new();
    println!("Enter the second component for the first 2D Vector");
    stdin()
        .read_line(&mut num2)
        .expect("Failed to read the line");
    let num2: f64 = num2.trim().parse()?;
    
    let mut num3 = String::new();
    println!("Enter the first component for the second 2D Vector");
    stdin()
        .read_line(&mut num3)
        .expect("Failed to read the line");
    let num3: f64 = num3.trim().parse()?;

    let mut num4 = String::new();
    println!("Enter the second component for the second 2D Vector");
    stdin()
        .read_line(&mut num4)
        .expect("Failed to read the line");
    let num4: f64 = num4.trim().parse()?;

    Ok((num1, num2, num3, num4))
}

pub fn get_scale_mul_input() -> Result<(f64, f64, f64), std::num::ParseFloatError> {
    let mut num1 = String::new();
    println!("Enter the first component for the 2D Vector");
    stdin()
        .read_line(&mut num1)
        .expect("Failed to read the line");
    let num1: f64 = num1.trim().parse()?;

    let mut num2 = String::new();
    println!("Enter the second component for the 2D Vector");
    stdin()
        .read_line(&mut num2)
        .expect("Failed to read the line");
    let num2: f64 = num2.trim().parse()?;
    
    let mut num3 = String::new();
    println!("Enter the number to multiply each component of the vector by");
    stdin()
        .read_line(&mut num3)
        .expect("Failed to read the line");
    let num3: f64 = num3.trim().parse()?;

    Ok((num1, num2, num3))
}

pub fn answer() -> String {
    let res = Colour::Green.paint("\nThe answer is");

    res.to_string()
}

pub fn greet() -> String {
    let art = Colour::Red.paint(
        r#"
 ________  ________                 ________  ________  ___       ________     
|\   __  \|\   ____\               |\   ____\|\   __  \|\  \     |\   ____\    
\ \  \|\  \ \  \___|_  ____________\ \  \___|\ \  \|\  \ \  \    \ \  \___|    
 \ \   _  _\ \_____  \|\____________\ \  \    \ \   __  \ \  \    \ \  \       
  \ \  \\  \\|____|\  \|____________|\ \  \____\ \  \ \  \ \  \____\ \  \____  
   \ \__\\ _\ ____\_\  \              \ \_______\ \__\ \__\ \_______\ \_______\
    \|__|\|__|\_________\              \|_______|\|__|\|__|\|_______|\|_______|
             \|_________|                                                      

    "#
    );

    art.to_string()
}

pub fn exit() {
    println!("{}", Colour::Purple.paint("\nGoodbye!\n"));
}
