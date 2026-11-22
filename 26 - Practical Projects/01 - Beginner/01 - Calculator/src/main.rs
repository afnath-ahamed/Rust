use std::{env, process::ExitCode};

#[derive(Debug, PartialEq, Eq)]
enum CalcError {
    UnknownOperator,
    DivideByZero,
    Overflow,
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::UnknownOperator => "expected one of +, -, *, /",
            Self::DivideByZero => "cannot divide by zero",
            Self::Overflow => "result does not fit in a signed 64-bit integer",
        })
    }
}

fn calculate(left: i64, operator: &str, right: i64) -> Result<i64, CalcError> {
    let value = match operator {
        "+" => left.checked_add(right),
        "-" => left.checked_sub(right),
        "*" => left.checked_mul(right),
        "/" if right == 0 => return Err(CalcError::DivideByZero),
        "/" => left.checked_div(right),
        _ => return Err(CalcError::UnknownOperator),
    };
    value.ok_or(CalcError::Overflow)
}

fn run(arguments: &[String]) -> Result<i64, String> {
    let [left, operator, right] = arguments else {
        return Err("usage: roadmap-calculator LEFT OP RIGHT".into());
    };
    let left = left.parse().map_err(|_| "LEFT must be an i64")?;
    let right = right.parse().map_err(|_| "RIGHT must be an i64")?;
    calculate(left, operator, right).map_err(|error| error.to_string())
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(answer) => {
            println!("{answer}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_integer_arithmetic() {
        assert_eq!(calculate(6, "*", 7), Ok(42));
        assert_eq!(calculate(-7, "/", 2), Ok(-3));
    }

    #[test]
    fn reports_each_arithmetic_failure() {
        assert_eq!(calculate(1, "/", 0), Err(CalcError::DivideByZero));
        assert_eq!(calculate(1, "%", 2), Err(CalcError::UnknownOperator));
        assert_eq!(calculate(i64::MAX, "+", 1), Err(CalcError::Overflow));
        assert_eq!(calculate(i64::MIN, "/", -1), Err(CalcError::Overflow));
    }

    #[test]
    fn rejects_malformed_arguments() {
        assert!(run(&[]).is_err());
        assert!(run(&["x".into(), "+".into(), "1".into()]).is_err());
    }
}
