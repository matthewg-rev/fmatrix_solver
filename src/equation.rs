// x + 2y + 4 = 0
// 8x + y = 9
// z + 2w + 1 = 12
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    None,
    Add,
    Subtract,
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub raw: String,

    pub variables: Vec<String>,
    pub coefficients: Vec<f64>,
    pub operations: Vec<Operator>,
    pub constant: f64
}

impl Equation {
    pub fn parse_from_string(raw: String) -> Equation {
        let mut equation = Equation {
            raw: raw.clone(),
            variables: Vec::new(),
            coefficients: Vec::new(),
            operations: Vec::new(),
            constant: 0.0
        };

        // all variables are non-numerical single characters
        let mut variables: Vec<String> = Vec::new();
        let mut coefficients: Vec<f64> = Vec::new();
        let mut operations: Vec<Operator> = Vec::new();

        // split by '='
        // check if '=' exists
        let exists = raw.contains('=');
        let mut equation_raw = raw.clone();
        if !exists { equation.constant = 0.0; } else {
            let split: Vec<&str> = raw.split('=').collect();
            // check which side of the equation is the constant
            if let Ok(value) = split[0].trim().parse::<f64>() {
                equation.constant = value;
                equation_raw = split[1].to_string();
            } else if let Ok(value) = split[1].trim().parse::<f64>() {
                equation.constant = value;
                equation_raw = split[0].to_string();
            } else {
                equation.constant = 0.0;
                equation_raw = raw.clone();
            }
        }

        let mut equation_raw = equation_raw.replace(" ", "");
        let split: Vec<&str> = equation_raw.split(|c| c == '+' || c == '-').collect();
        let mut negative = false;
        for section in split.clone() {
            // find f64 in section
            let mut coefficient = 1.0;
            let mut coefficient_str = String::new();
            let mut variable = String::new();

            for (i, c) in section.chars().enumerate() {
                if c.is_numeric() || c == '.' {
                    coefficient_str.push(c);
                } else {
                    if i == 0 {
                        coefficient = 1.0;
                    } else {
                        coefficient = coefficient_str.parse().unwrap();
                    }
                    variable.push(c);
                }
            }
            if let Ok(value) = coefficient_str.parse::<f64>() {
                coefficient = value;
            }

            // find operator in equation_raw
            if section != split[0] {
                // find the operator before the section:
                // make sure '4x + 4' doesn't find '4x' as the occurence of section when
                // we are checking for the operator before '4'

                let mut operator = Operator::None;
                let operators = ['+', '-'];
                // match operator + section
                for operator_char in operators.iter() {
                    let mut match_string = String::new();
                    match_string += &operator_char.to_string();
                    match_string += &section;
                    if equation_raw.contains(&match_string) {
                        operator = match operator_char {
                            '+' => Operator::Add,
                            '-' => Operator::Subtract,
                            _ => panic!("Invalid operator")
                        };
                        break;
                    }
                }

                if operator != Operator::None {
                    operations.push(operator);
                } else {
                    println!("No operator found for section: {}", section);
                }
            } else if section == "" {
                // this is the biggest hack
                // if there's an negative operator before the first section
                // the split causes the first section to be empty
                negative = true;
                continue;
            }

            if variable.len() == 0 && section != "" {
                if operations.last() == Some(&Operator::Subtract) {
                    equation.constant += coefficient;
                } else {
                    equation.constant -= coefficient;
                }
                operations.remove(operations.len() - 1);
            } else {
                // check if variable exists
                let mut exists = false;
                let mut exists_index: i32 = -1;
                for v in variables.clone() {
                    exists_index += 1;
                    if v == variable {
                        exists = true;
                        break;
                    }
                }

                if negative {
                    coefficient *= -1.0;
                    negative = false;
                    operations.remove(operations.len() - 1);
                }

                if exists {
                    let coefficient_multiplier = match operations.last() {
                        Some(Operator::Add) => 1.0,
                        Some(Operator::Subtract) => -1.0,
                        _ => 1.0
                    };
                    coefficient *= coefficient_multiplier;
                    if operations.last() != None {
                        operations.remove(operations.len() - 1);
                    }
                    coefficients[exists_index as usize] += coefficient;
                } else {
                    let coefficient_multiplier = match operations.last() {
                        Some(Operator::Add) => 1.0,
                        Some(Operator::Subtract) => -1.0,
                        _ => 1.0
                    };
                    coefficient *= coefficient_multiplier;
                    if operations.last() != None {
                        operations.remove(operations.len() - 1);
                    }
                    variables.push(variable);
                    coefficients.push(coefficient);
                }
            }
        }

        equation.coefficients = coefficients;
        equation.variables = variables;
        equation.operations = operations;
        equation
    }
}

impl ToString for Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Add => "+".to_string(),
            Operator::Subtract => "-".to_string(),
            Operator::None => panic!("Operator::None cannot be converted to a string")
        }
    }
}

impl ToString for Equation {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for (i, variable) in self.variables.iter().enumerate() {
            string.push_str(&format!("{}{} ", self.coefficients[i], variable));
        }
        string.push_str(&format!("{:?} ", self.operations));
        string.push_str(&format!("= {}", self.constant));
        string
    }
}