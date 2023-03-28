fn evaluate(expression: &str) -> Result<f64, String> {
    let operators = ['+', '-', '*', '/'];
    
    let current_operator = operators.iter().find(|op| expression.contains(**op));
    
    let current_operator = match current_operator {
        Some(op) => op,
        None => return Err("Invalid expression: Operator not found".to_string()),
    };
    
    let parts: Vec<&str> = expression.split(*current_operator).collect();
    let left_operand = parts[0].parse::<f64>().map_err(|_| "Invalid left operand".to_string())?;
    let right_operand = parts[1].parse::<f64>().map_err(|_| "Invalid right operand".to_string())?;
    
    let result = match current_operator {
        &'+' => left_operand + right_operand,
        &'-' => left_operand - right_operand,
        &'*' => left_operand * right_operand,
        &'/' => left_operand / right_operand,
        _ => return Err("Invalid operator".to_string()),
    };
    
    Ok(result)
}

fn main() {
    let expressions = vec!["1+1", "5-2", "3*4", "10/2"];
    
    for expression in expressions {
        let result = evaluate(expression);
        match result {
            Ok(value) => println!("{} = {}", expression, value),
            Err(error) => println!("Error: {}", error),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn test_addition() {
        assert_eq!(evaluate("1+1").unwrap(), 2.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(evaluate("5-2").unwrap(), 3.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(evaluate("3*4").unwrap(), 12.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(evaluate("10/2").unwrap(), 5.0);
    }

    #[test]
    fn test_invalid_expression() {
        assert!(evaluate("1+").is_err());
    }

    #[test]
    fn test_invalid_expression_2() {
        assert!(evaluate("+1").is_err());
    }

    #[test]
    fn test_invalid_expression_3() {
        assert!(evaluate("1++1").is_err());
    }

    #[test]
    fn test_invalid_expression_4() {
        assert!(evaluate("1+-1").is_err());
    }

    #[test]
    fn test_invalid_expression_5() {
        assert!(evaluate("1+*1").is_err());
    }

    #[test]
    fn test_invalid_expression_6() {
        assert!(evaluate("1+/1").is_err());
    }

    #[test]
    fn test_invalid_operator() {
        assert!(evaluate("1^2").is_err());
    }
}
