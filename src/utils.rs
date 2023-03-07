use std::io::stdin;

use ansi_term::Colour;

type OpFloat19<T> =
    Result<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), std::num::ParseFloatError>;
type OpFloat9<T> = Result<(T, T, T, T, T, T, T, T, T), std::num::ParseFloatError>;
type OpFloat8<T> = Result<(T, T, T, T, T, T, T, T), std::num::ParseFloatError>;
type OpFloat6<T> = Result<(T, T, T, T, T, T), std::num::ParseFloatError>;
type OpFloat4<T> = Result<(T, T, T, T), std::num::ParseFloatError>;
type OpFloat3<T> = Result<(T, T, T), std::num::ParseFloatError>;

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

pub fn get_2d_vector_input() -> OpFloat4<f64> {
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

pub fn get_single_2d_matrix_input() -> OpFloat4<f64> {
    let mut input = String::new();

    println!("Enter float values for the 2D matrix, separated by spaces (Ex: 2.8 4.3 2.0 3.8): ");

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

pub fn get_single_3d_matrix_input() -> OpFloat9<f64> {
    let mut input = String::new();

    println!("Enter float values for the 3D matrix, separated by spaces (Ex: 2.8 4.3 2.0 3.8 3 4.5 2 3 6.5): ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 9];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 8 {
            break;
        }
    }

    Ok((nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], nums[8]))
}

pub fn get_2d_scalar_input() -> OpFloat3<f64> {
    let mut input = String::new();

    println!(
        "Enter float values and a third scalar value, separated by spaces (Ex: 2.8 4.3 2.0): "
    );

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

pub fn get_3d_vector_input() -> OpFloat6<f64> {
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

pub fn get_2d_matrix_input() -> OpFloat8<f64> {
    let mut input = String::new();

    println!("Enter float values for the 2D Matrices, separated by spaces (Ex: 2.8 4.3 2.0 4.6 2 3 6.5 3): ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 8];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 7 {
            break;
        }
    }

    Ok((
        nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7],
    ))
}

pub fn get_3d_matrix_input() -> OpFloat19<f64> {
    let mut input = String::new();

    println!("Enter 18 float values for two 3D Matrices, separated by spaces (Ex: 2.8 4.3 2.0 4.6 ... 3): ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let values = input.split_whitespace();
    let mut nums = [0.0; 18];

    for (i, value) in values.enumerate() {
        nums[i] = value.parse()?;
        if i == 17 {
            break;
        }
    }

    Ok((
        nums[0], nums[1], nums[2], nums[3], nums[4], nums[5], nums[6], nums[7], nums[8], nums[9],
        nums[10], nums[11], nums[12], nums[13], nums[14], nums[15], nums[16], nums[17],
    ))
}

pub fn get_3d_scalar_input() -> OpFloat4<f64> {
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

pub fn get_3d_translation_input() -> OpFloat6<f64> {
    let mut input = String::new();

    println!("Enter float values for the 3D Vector to be summed with a 3D Point");

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

    "#,
    );

    art.to_string()
}

pub fn exit() {
    println!("{}", Colour::Purple.paint("\nGoodbye!\n"));
}
