mod regex_generator;  // Import the module that contains RegexGenerator

use regex_generator::RegexGenerator;

fn main() {
    
    // Test group capturing and backreference
    let mut generator = RegexGenerator::new(r"\i{:10}", Some("9998".to_string()), None);
    for _ in 0..5 {
        println!("Generated: {}", generator.generate());
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_ascending() {
        let pattern = r"\i";
        let increment_value = Some("1299".to_string());
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        let expected_results = vec!["1300", "1301", "1302", "1303", "1304"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }
    }

    #[test]
    fn test_increment_descending() {
        let pattern = r"\i-";
        let increment_value = Some("1300".to_string());
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        let expected_results = vec!["1299", "1298", "1297", "1296", "1295"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }
    }

    #[test]
    fn test_array_random() {
        let pattern = r"\a";
        let increment_value = None;
        let array_values = Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
        let mut generator = RegexGenerator::new(pattern, increment_value, array_values);

        for _ in 0..5 {
            let generated = generator.generate();
            assert!(["apple", "banana", "cherry"].contains(&generated.as_str()));
        }
    }

    #[test]
    fn test_array_ascending() {
        let pattern = r"\a+";
        let increment_value = None;
        let array_values = Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
        let mut generator = RegexGenerator::new(pattern, increment_value, array_values);

        let expected_results = vec!["apple", "banana", "cherry", "apple", "banana"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }
    }

    #[test]
    fn test_array_descending() {
        let pattern = r"\a-";
        let increment_value = None;
        let array_values = Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
        let mut generator = RegexGenerator::new(pattern, increment_value, array_values);

        let expected_results = vec!["cherry", "banana", "apple", "cherry", "banana"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }
    }

    #[test]
    fn test_combined_increment_and_array_ascending() {
        let pattern = r"\a+\i+";
        let increment_value = Some("1299".to_string());
        let array_values = Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
        let mut generator = RegexGenerator::new(pattern, increment_value, array_values);

        let expected_results = vec!["apple1300", "banana1301", "cherry1302", "apple1303", "banana1304"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }
    }

    #[test]
    fn test_combined_increment_and_array_descending() {
        let pattern = r"\a-\i-";
        let increment_value = Some("1304".to_string());
        let array_values = Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()]);
        let mut generator = RegexGenerator::new(pattern, increment_value, array_values);

        let expected_results = vec!["cherry1303", "banana1302", "apple1301", "cherry1300", "banana1299"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }
    }

    #[test]
    fn test_character_classes() {
        let pattern = r"\d\w\s";
        let increment_value = None;
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        for _ in 0..5 {
            let generated = generator.generate();
            print!("{}", generated);
            assert!(generated.len() == 3);
            assert!(generated.chars().nth(0).unwrap().is_digit(10));
            assert!(generated.chars().nth(1).unwrap().is_alphanumeric());
            assert!(generated.chars().nth(2).unwrap().is_whitespace());
        }
    }

    #[test]
    fn test_custom_repeat() {
        let pattern = r"\d{2,4}";
        let increment_value = None;
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        for _ in 0..5 {
            let generated = generator.generate();
            assert!(generated.len() >= 2 && generated.len() <= 4);
            assert!(generated.chars().all(|c| c.is_digit(10)));
        }
    }

    #[test]
    fn test_character_ranges() {
        let pattern = r"[a-c]{3}";
        let increment_value = None;
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        for _ in 0..5 {
            let generated = generator.generate();
            assert!(generated.len() == 3);
            assert!(generated.chars().all(|c| c >= 'a' && c <= 'c'));
        }
    }

    #[test]
    fn test_character_negation() {
        let pattern = r"[^a-c]{3}";
        let increment_value = None;
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        for _ in 0..5 {
            let generated = generator.generate();
            assert!(generated.len() == 3);
            assert!(generated.chars().all(|c| c < 'a' || c > 'c'));
        }
    }

    #[test]
    fn test_group_capturing_and_backreference() {
        let pattern = r"(ab)\+(cd)=\2\+\1";
        let increment_value = None;
        let mut generator = RegexGenerator::new(pattern, increment_value, None);

        for _ in 0..5 {
            let generated = generator.generate();
            let parts: Vec<&str> = generated.split('=').collect();
            assert_eq!(parts.len(), 2);

            let left_side: Vec<&str> = parts[0].split('+').collect();
            let right_side: Vec<&str> = parts[1].split('+').collect();

            assert_eq!(left_side[1], right_side[0]);
            assert_eq!(left_side[0], right_side[1]);
        }
    }
    #[test]
    fn test_leading_zero(){
        let pattern: &str = r"[0-9]{3:10}";
        let increment_value = None;
        let mut generator = RegexGenerator::new(pattern, increment_value, None);
        for _ in 0..5 {
            let generated = generator.generate();
            assert!(generated.len() == 10);
            assert!(generated.chars().nth(6).unwrap() == '0');

        }

    }

    #[test]
    fn test_increment_leading_zero(){
        let pattern:&str = r"\i{:5}";
        let increment_value = Some("998".to_string());
        let mut generator = RegexGenerator::new(pattern, increment_value, None);
        let expected_results = vec!["00999", "01000", "01001", "01002", "01003"];
        for expected in expected_results {
            let generated = generator.generate();
            assert_eq!(generated, expected);
        }

    }



}

