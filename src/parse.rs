use crate::{
    operations::BinaryOperator,
    time::{TimeUnit, TimeValue},
};
use nom::bytes::complete::{tag, tag_no_case};

fn parse_time_unit(input: &str) -> nom::IResult<&str, TimeUnit> {
    // Longer names are first since we want to capture the longest value possible
    // This prevents just "s" from being captured in "seconds"
    nom::branch::alt((
        nom::combinator::value(TimeUnit::Nanoseconds, tag_no_case("nanoseconds")),
        nom::combinator::value(TimeUnit::Nanoseconds, tag_no_case("nanos")),
        nom::combinator::value(TimeUnit::Nanoseconds, tag_no_case("ns")),
        // -----
        nom::combinator::value(TimeUnit::Microseconds, tag_no_case("microseconds")),
        nom::combinator::value(TimeUnit::Microseconds, tag_no_case("micros")),
        nom::combinator::value(TimeUnit::Microseconds, tag_no_case("us")),
        // -----
        nom::combinator::value(TimeUnit::Milliseconds, tag_no_case("milliseconds")),
        nom::combinator::value(TimeUnit::Milliseconds, tag_no_case("millis")),
        nom::combinator::value(TimeUnit::Milliseconds, tag_no_case("ms")),
        // -----
        nom::combinator::value(TimeUnit::Seconds, tag_no_case("seconds")),
        nom::combinator::value(TimeUnit::Seconds, tag_no_case("sec")),
        nom::combinator::value(TimeUnit::Seconds, tag_no_case("s")),
    ))(input)
}

fn parse_time_value(input: &str) -> nom::IResult<&str, TimeValue> {
    let (input, count) = nom::character::complete::i64(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, unit) = parse_time_unit(input)?;
    Ok((input, TimeValue::new(count, unit)))
}

fn parse_unit_conversion(input: &str) -> nom::IResult<&str, TimeUnit> {
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, _) = nom::branch::alt((tag_no_case("as"),))(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, unit) = parse_time_unit(input)?;
    Ok((input, unit))
}

fn parse_binary_operator(input: &str) -> nom::IResult<&str, BinaryOperator> {
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, op) = nom::branch::alt((
        nom::combinator::value(BinaryOperator::Plus, tag("+")),
        nom::combinator::value(BinaryOperator::Minus, tag("-")),
    ))(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    Ok((input, op))
}

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
    fn test_parse_nanos() {
        let res = parse_time_unit("nanoseconds");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let (_, unit) = res.unwrap();
        assert!(matches!(unit, TimeUnit::Nanoseconds));
    }

    #[test]
    fn test_parse_time_value() {
        let res = parse_time_value("20 seconds");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let (_, value) = res.unwrap();

        assert!(matches!(value.nanos(), 20_000_000_000));
        assert!(matches!(value.unit(), TimeUnit::Seconds));
    }

    #[test]
    fn test_calculate_expression_time_value() {
        let res = calculate("10 us");
        assert!(res.is_ok(), "{}", res.unwrap_err());
        let value = res.unwrap();

        assert_eq!(format!("{}", value), "10us");
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
}
