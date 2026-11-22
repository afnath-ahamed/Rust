use std::{env, process::ExitCode};

fn convert(input: f64, source: &str) -> Result<(f64, &'static str), &'static str> {
    if !input.is_finite() {
        return Err("temperature must be finite");
    }
    let result = match source {
        "c" | "C" => (input * 1.8 + 32.0, "F"),
        "f" | "F" => ((input - 32.0) / 1.8, "C"),
        _ => return Err("scale must be C or F"),
    };
    if !result.0.is_finite() {
        return Err("converted temperature is outside the numeric range");
    }
    Ok(result)
}

fn run(arguments: &[String]) -> Result<(f64, &'static str), &'static str> {
    let [input, scale] = arguments else {
        return Err("usage: roadmap-temperature VALUE C_OR_F");
    };
    convert(input.parse().map_err(|_| "invalid temperature")?, scale)
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok((value, scale)) => {
            println!("{value:.2} {scale}");
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
    fn covers_freezing_boiling_and_negative_values() {
        assert_eq!(convert(0.0, "C"), Ok((32.0, "F")));
        assert_eq!(convert(212.0, "F"), Ok((100.0, "C")));
        assert_eq!(convert(-40.0, "F"), Ok((-40.0, "C")));
    }

    #[test]
    fn round_trips_within_tolerance() {
        let (fahrenheit, _) = convert(37.0, "C").unwrap();
        let (celsius, _) = convert(fahrenheit, "F").unwrap();
        assert!((celsius - 37.0).abs() < 1e-9);
    }

    #[test]
    fn rejects_invalid_inputs_and_overflow() {
        assert!(convert(f64::NAN, "C").is_err());
        assert!(convert(f64::INFINITY, "F").is_err());
        assert!(convert(f64::MAX, "C").is_err());
        assert!(convert(1.0, "K").is_err());
    }
}
