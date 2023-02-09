use color_eyre::eyre::Result;
use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    Select,
};
use nalgebra::{Vector2, Vector3};

pub mod utils;

fn get_render_cfg() -> RenderConfig {
    RenderConfig {
        answer: StyleSheet::new()
            .with_attr(Attributes::ITALIC)
            .with_fg(Color::LightCyan),
        help_message: StyleSheet::new().with_fg(Color::LightCyan),
        ..Default::default()
    }
}

fn menu(items: &[String]) -> String {
    Select::new("What operation would you like to perform?", items.to_vec())
        .with_help_message("Vim mode is disabled")
        .prompt()
        .unwrap_or_else(|e| e.to_string())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("{}", utils::greet());
    println!("By CM-IV <chuck@civdev.xyz>\n");

    inquire::set_global_render_config(get_render_cfg());

    fn handle_input(calculation: impl Fn(f64, f64) -> f64) {
        match utils::get_input() {
            Ok(nums) => println!("{}: {}\n", utils::answer(), calculation(nums.0, nums.1)),
            Err(_) => println!("The input is not a valid number, please try again."),
        }
    }

    fn handle_2d_vector_input(operation: impl Fn(Vector2<f64>, Vector2<f64>) -> Vector2<f64>) {
        match utils::get_2d_vector_input() {
            Ok(components) => {
                println!("{}: {}\n", utils::answer(), operation(Vector2::new(components.0, components.1), Vector2::new(components.2, components.3)));
            },
            Err(_) => println!("The input is not a valid float for the vector component, please try again."),
        }
    }

    fn handle_2d_scalar_input(operation: impl Fn(Vector2<f64>, f64) -> Vector2<f64>) {
        match utils::get_2d_scalar_input() {
            Ok(components) => {
                println!("{}: {}\n", utils::answer(), operation(Vector2::new(components.0, components.1), components.2));
            },
            Err(_) => println!("The input is an invalid float number, please try again.")
        }
    }

    fn handle_3d_vector_input(operation: impl Fn(Vector3<f64>, Vector3<f64>) -> Vector3<f64>) {
        match utils::get_3d_vector_input() {
            Ok(components) => {
                println!("{}: {}\n", utils::answer(), operation(Vector3::new(components.0, components.1, components.2), Vector3::new(components.3, components.4, components.5)));
            },
            Err(_) => println!("The input is not a valid float for the vector component, please try again."),
        }
    }

    fn handle_3d_dot_input(operation: impl Fn(Vector3<f64>, Vector3<f64>) -> f64) {
        match utils::get_3d_vector_input() {
            Ok(components) => {
                println!("{}: {}\n", utils::answer(), operation(Vector3::new(components.0, components.1, components.2), Vector3::new(components.3, components.4, components.5)));
            },
            Err(_) => println!("The input is not a valid float for the vector component, please try again."),
        }
    }

    fn handle_3d_scalar_input(operation: impl Fn(Vector3<f64>, f64) -> Vector3<f64>) {
        match utils::get_3d_scalar_input() {
            Ok(components) => {
                println!("{}: {}\n", utils::answer(), operation(Vector3::new(components.0, components.1, components.2), components.3));
            },
            Err(_) => println!("The input is an invalid float number, please try again.")
        }
    }

    loop {
        match menu(&[
            "Addition".into(),
            "Subtraction".into(),
            "Multiplication".into(),
            "Division".into(),
            "2D Vector Addition".into(),
            "2D Vector Subtraction".into(),
            "2D Vector Scalar Multiplication".into(),
            "3D Vector Addition".into(),
            "3D Vector Subtraction".into(),
            "3D Vector Scalar Multiplication".into(),
            "3D Vector Dot Product".into(),
            "3D Vector Cross Product".into(),
            "Exit".into(),
        ])
        .as_str()
        {
            "Addition" => handle_input(|a, b| a + b),
            "Subtraction" => handle_input(|a, b| a - b),
            "Multiplication" => handle_input(|a, b| a * b),
            "Division" => handle_input(|a, b| a / b),
            "2D Vector Addition" => handle_2d_vector_input(|a, b| a + b),
            "2D Vector Subtraction" => handle_2d_vector_input(|a, b| a - b),
            "2D Vector Scalar Multiplication" => handle_2d_scalar_input(|a, b| a.scale(b)),
            "3D Vector Addition" => handle_3d_vector_input(|a, b| a + b),
            "3D Vector Subtraction" => handle_3d_vector_input(|a, b| a - b),
            "3D Vector Scalar Multiplication" => handle_3d_scalar_input(|a, b| a.scale(b)),
            "3D Vector Dot Product" => handle_3d_dot_input(|a, b| a.dot(&b)),
            "3D Vector Cross Product" => handle_3d_vector_input(|a, b| a.cross(&b)),
            "Exit" => {
                utils::exit();
                break;
            }
            err => println!("{err}"),
        }
    }

    Ok(())
}
