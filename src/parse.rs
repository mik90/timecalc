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

pub fn parse_time_value(input: &str) -> nom::IResult<&str, TimeValue> {
    let (input, count) = nom::character::complete::i64(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, unit) = parse_time_unit(input)?;
    Ok((input, TimeValue::new(count, unit)))
}

pub fn parse_unit_conversion(input: &str) -> nom::IResult<&str, TimeUnit> {
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, _) = nom::branch::alt((tag_no_case("as"),))(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, unit) = parse_time_unit(input)?;
    Ok((input, unit))
}

pub fn parse_binary_operator(input: &str) -> nom::IResult<&str, BinaryOperator> {
    let (input, _) = nom::character::complete::space0(input)?;
    let (input, op) = nom::branch::alt((
        nom::combinator::value(BinaryOperator::Plus, tag("+")),
        nom::combinator::value(BinaryOperator::Minus, tag("-")),
    ))(input)?;
    let (input, _) = nom::character::complete::space0(input)?;
    Ok((input, op))
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
}
