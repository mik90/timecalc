use crate::{
    parse::{parse_binary_operator, parse_time_value, parse_unit_conversion},
    time::TimeValue,
};

pub fn calculate(input: &str) -> Result<TimeValue, String> {
    let (mut rest_of_input, mut value) = parse_time_value(input)
        .map_err(|_| format!("Expected '<number> <time_unit>' but saw '{}'", input))?;

    while !rest_of_input.trim().is_empty() {
        if rest_of_input.trim_start().starts_with("as") {
            let (input, new_unit) = parse_unit_conversion(rest_of_input).map_err(|_| {
                format!("Expected 'as <time_unit>' next but saw '{}'", rest_of_input)
            })?;
            rest_of_input = input;
            value.change_unit(new_unit);
        } else {
            let (input, op) = parse_binary_operator(rest_of_input).map_err(|_| {
                format!("Expected binary operator next but saw '{}'", rest_of_input)
            })?;
            rest_of_input = input;

            let (input, rhs) = parse_time_value(rest_of_input)
                .map_err(|_| format!("Expected time value next but saw '{}'", rest_of_input))?;
            rest_of_input = input;

            value.modify(op, rhs);
        }
    }
    Ok(value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_expression_time_value() {
        let res = calculate("10 us");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "10us");
    }

    #[test]
    fn test_calculate_expression_conversion_only() {
        let res = calculate("1000 us as ms");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "1ms");
    }

    #[test]
    fn test_calculate_expression_fractional_input() {
        let res = calculate("0.5sec as ms");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "500ms");
    }

    #[test]
    fn test_calculate_expression_addition() {
        let res = calculate("10 ms + 5 ms");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "15ms");
    }

    #[test]
    fn test_calculate_expression_subtraction() {
        let res = calculate("90 sec - 5 ms");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "89.995sec");
    }

    #[test]
    fn test_calculate_conversion_sec_ms() {
        let res = calculate("10 s - 5000 ms");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "5sec");
    }

    #[test]
    fn test_calculate_conversion_ms_us() {
        let res = calculate("10 ms - 5000 us");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "5ms");
    }

    #[test]
    fn test_calculate_conversion_us_ns() {
        let res = calculate("10 us - 5000 ns");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "5us");
    }

    #[test]
    fn test_calculate_with_unit_conversion() {
        let res = calculate("10 us - 5000 ns as ns");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "5000ns");
    }

    #[test]
    fn test_calculate_half_of_microsecond() {
        let res = calculate("1 us - 500 ns");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "0.5us");
    }

    #[test]
    fn test_calculate_quarter_of_microsecond() {
        let res = calculate("1 us - 750 ns");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "0.25us");
    }

    #[test]
    fn test_calculate_third_of_microsecond() {
        let res = calculate("1 us - 666 ns");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "0.334us");
    }

    #[test]
    fn test_calculate_expression_fractional_addition() {
        let res = calculate("0.5sec + 0.25sec");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "0.75sec");
    }
}
