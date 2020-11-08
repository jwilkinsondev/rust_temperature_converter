
fn main() -> std::io::Result<()> {
    println!("Enter a celsius temperature that you would like to convert to fahrenheit.");

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;

        if input.trim() == "exit" {
            return Ok(());
        }

        let number = input.trim().parse::<f32>();

        let converted = number.map(|n| convert_to_fahrenheit(n));
        match converted {
            Ok(c) => println!("In fahrenheit {}", c),
            Err(e) => println!("bad input {}", e),
        }
    }
}

fn convert_to_fahrenheit(celsius_temperature: f32) -> f32 {
    return (celsius_temperature * 9. / 5.) + 32.;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        assert_eq!(convert_to_fahrenheit(25.), 77.)
    }

    #[test]
    fn test_zero() {
        assert_eq!(convert_to_fahrenheit(0.), 32.)
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(convert_to_fahrenheit(-40.), -40.)
    }
}