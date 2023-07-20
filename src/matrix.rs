use std::ptr::eq;
use crate::equation::Equation;

// Gaussian Elimination Transformation Types
#[derive(Debug, Clone)]
pub enum Transformation {
    Multiply(MatrixRow, f64),
    Swap(MatrixRow, MatrixRow),
    Add(MatrixRow, MatrixRow),
    Compounded(Vec<Transformation>, MatrixRow)
}

impl ToString for Transformation {
    fn to_string(&self) -> String {
        match self {
            Transformation::Multiply(row, constant) => {
                let mut output = String::new();
                output += "L";
                output += format!("{}", row.occupying_row).as_str();
                output += " * ";
                output += format!("{}", constant).as_str();
                output += " -> L";
                output += format!("{}", row.occupying_row).as_str();
                output
            },
            Transformation::Swap(row1, row2) => {
                let mut output = String::new();
                output += "L";
                output += format!("{}", row1.occupying_row).as_str();
                output += " <-> L";
                output += format!("{}", row2.occupying_row).as_str();
                output
            },
            Transformation::Add(row1, row2) => {
                let mut output = String::new();
                output += "L";
                output += format!("{}", row1.occupying_row).as_str();
                output += " + L";
                output += format!("{}", row2.occupying_row).as_str();
                output += " -> L";
                output += format!("{}", row1.occupying_row).as_str();
                output
            },
            Transformation::Compounded(transformations, row) => {
                let mut output = String::new();
                for transformation in transformations {
                    output += transformation.to_string().as_str();
                    output += "\n";
                }
                output += "L";
                output += format!("{}", row.occupying_row).as_str();
                output += " -> L";
                output += format!("{}", row.occupying_row).as_str();
                output
            }
            _ => {
                String::new()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatrixRow {
    pub coefficients: Vec<f64>,
    pub constant: f64,
    pub occupying_row: usize
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub variables: Vec<String>,
    pub rows: Vec<MatrixRow>
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            variables: Vec::new(),
            rows: Vec::new()
        }
    }

    pub fn gaussian_elimination(&mut self) -> (Vec<Transformation>, Matrix) {
        let mut matrix_to_solve = self.clone();
        let mut transformations: Vec<Transformation> = Vec::new();

        for k in 0..matrix_to_solve.rows.len() {
            let mut i_max = k;
            let mut v_max = matrix_to_solve.rows[k].coefficients[k];

            for i in k+1..matrix_to_solve.rows.len() {
                if matrix_to_solve.rows[i].coefficients[k].abs() > v_max.abs() {
                    v_max = matrix_to_solve.rows[i].coefficients[k];
                    i_max = i;
                }
            }

            if v_max == 0.0 {
                panic!("Matrix is singular");
            }

            if i_max != k {
                transformations.push(Transformation::Swap(matrix_to_solve.rows[k].clone(), matrix_to_solve.rows[i_max].clone()));
                matrix_to_solve.rows.swap(k, i_max);
            }

            for i in k+1..matrix_to_solve.rows.len() {
                let f = matrix_to_solve.rows[i].coefficients[k] / matrix_to_solve.rows[k].coefficients[k];
                transformations.push(Transformation::Compounded(
                    vec![
                        Transformation::Multiply(matrix_to_solve.rows[k].clone(), f),
                        Transformation::Add(matrix_to_solve.rows[i].clone(), matrix_to_solve.rows[k].clone())
                    ],
                    matrix_to_solve.rows[i].clone()
                ));
                matrix_to_solve.rows[i].coefficients[k] = 0.0;
                for j in k+1..matrix_to_solve.rows.len() {
                    matrix_to_solve.rows[i].coefficients[j] -= matrix_to_solve.rows[k].coefficients[j] * f;
                }
                matrix_to_solve.rows[i].constant -= matrix_to_solve.rows[k].constant * f;
            }
        }

        // back substitution
        for i in (0..matrix_to_solve.rows.len()).rev() {
            let mut f = matrix_to_solve.rows[i].constant;
            for j in i+1..matrix_to_solve.rows.len() {
                f -= matrix_to_solve.rows[i].coefficients[j] * matrix_to_solve.rows[j].constant;
            }
            f /= matrix_to_solve.rows[i].coefficients[i];
            transformations.push(Transformation::Multiply(matrix_to_solve.rows[i].clone(), 1.0 / matrix_to_solve.rows[i].coefficients[i]));
            matrix_to_solve.rows[i].constant = f;
            matrix_to_solve.rows[i].coefficients[i] = 1.0;
        }

        (transformations, matrix_to_solve)
    }

    pub fn adjust_rows(&mut self) {
        for row in &mut self.rows {
            if row.coefficients.len() < self.variables.len() {
                for _ in row.coefficients.len()..self.variables.len() {
                    row.coefficients.push(0.0);
                }
            }
        }
    }

    pub fn add_equation(&mut self, equation: Equation) {
        let mut row = MatrixRow {
            coefficients: Vec::new(),
            constant: equation.constant,
            occupying_row: self.rows.len()
        };

        for variable in self.variables.clone() {
            if equation.variables.contains(&variable) {
                row.coefficients.push(equation.coefficients[equation.variables.iter().position(|x| x == &variable).unwrap()]);
            } else {
                row.coefficients.push(0.0);
            }
        }

        for variable in equation.variables.clone() {
            if !self.variables.contains(&variable) {
                self.variables.push(variable.clone());
                row.coefficients.push(equation.coefficients[equation.variables.iter().position(|x| x == &variable).unwrap()]);
            }
        }

        row.constant = equation.constant;
        self.adjust_rows();
        self.rows.push(row);
    }
}

impl ToString for Matrix {
    fn to_string(&self) -> String {
        let mut output = String::new();

        for variable in &self.variables {
            output += format!("{:<10}", variable).as_str();
        }
        output += format!("{:<10}", "K").as_str();
        output += "\n";

        for row in &self.rows {
            for coefficient in &row.coefficients {
                let coefficient = (coefficient * 1000.0).round() / 1000.0;
                output += format!("{:<10}", coefficient).as_str();
            }
            let constant = (row.constant * 1000.0).round() / 1000.0;
            output += format!("{:<10}", constant).as_str();
            output += "\n";
        }
        output
    }
}