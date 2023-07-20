use crate::equation::Equation;
use crate::matrix::Matrix;

mod equation;
mod matrix;

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_parse_from_string() {
        let equation = Equation::parse_from_string("2x + 3y - 4z = 5".to_string());
        println!("{:?}", equation);
    }

    #[test]
    fn test_matrix_add_equation() {
        let mut matrix = Matrix::new();
        matrix.add_equation(Equation::parse_from_string("2x + 3y - 4z = 5".to_string()));
        matrix.add_equation(Equation::parse_from_string("3x + 4y - 5z = 6".to_string()));
        println!("{}", matrix.to_string());
    }
}

fn main() {
    println!("Enter the amount of equations you want to solve: ");
    let mut amount_of_equations = 0;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    amount_of_equations = input.trim().parse().unwrap();

    let mut equations: Vec<Equation> = Vec::new();
    for _ in 0..amount_of_equations {
        println!("Enter a valid system of equations: ");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        equations.push(Equation::parse_from_string(input.trim().to_string()));
    }

    println!("Initial matrix: ");
    println!("{}", "-".repeat(100));
    let mut system = Matrix::new();
    for equation in &equations {
        system.add_equation(equation.clone());
    }
    println!("{}", system.to_string());

    let (transformations, mut solved) = system.gaussian_elimination();
    println!("Transformations: ");
    for transformation in transformations {
        println!("{}", transformation.to_string());
    }
    println!("{}", solved.to_string());
}
