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

pub fn get_2d_vector_input() -> Result<(f64, f64, f64, f64), std::num::ParseFloatError> {

    let mut input = String::new();

    println!("Enter float values for the two vectors, separated by spaces (Ex: 2.8 4.3 2.0 3.8): ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 4];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 3 {
            break;
        }
    }

    Ok((nums[0], nums[1], nums[2], nums[3]))
}

pub fn get_2d_scalar_input() -> Result<(f64, f64, f64), std::num::ParseFloatError> {
    let mut input = String::new();

    println!("Enter float values and a third scalar value, separated by spaces (Ex: 2.8 4.3 2.0): ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 3];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 2 {
            break;
        }
    }

    Ok((nums[0], nums[1], nums[2]))
}

pub fn get_3d_vector_input() -> Result<(f64, f64, f64, f64, f64, f64), std::num::ParseFloatError> {

    let mut input = String::new();

    println!("Enter float values for the two 3 Dimensional vectors (Ex: a1 a2 a3 b1 b2 b3, where →a=⟨a1,a2,a3⟩ and →b=⟨b1,b2,b3⟩) ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 6];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 5 {
            break;
        }
    }

    Ok((nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]))
}

pub fn get_3d_scalar_input() -> Result<(f64, f64, f64, f64), std::num::ParseFloatError> {
    let mut input = String::new();

    println!("Enter float values for a 3D vector and a fourth scalar value separated by spaces (Ex: 2.8 4.3 2.0 3.5): ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 4];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 3 {
            break;
        }
    }

    Ok((nums[0], nums[1], nums[2], nums[3]))
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
