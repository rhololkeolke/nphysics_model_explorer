use failure::Fail;
use nalgebra as na;
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug, Fail)]
pub enum ParseRealAttributeError {
    #[fail(display = "Failed to parse string \"{}\" to real value", 0)]
    ParseError(String),
    #[fail(
        display = "Incorrect number of values. Expected {}. Got {}",
        expected_len, actual_len
    )]
    IncorrectNumberOfValues {
        expected_len: usize,
        actual_len: usize,
    },
}

pub fn parse_real_vector_attribute<N: na::RealField, D: na::DimName>(
    text_attribute: &str,
) -> Result<na::VectorN<N, D>, ParseRealAttributeError>
where
    na::DefaultAllocator: na::allocator::Allocator<N, D>,
    N: FromStr,
{
    let mut output = na::VectorN::<N, D>::zeros();

    let text_values: Vec<&str> = text_attribute.split_whitespace().collect();
    let expected_num_values = D::try_to_usize().expect("Failed to get dimension as usize");
    if text_values.len() != expected_num_values {
        return Err(ParseRealAttributeError::IncorrectNumberOfValues {
            expected_len: expected_num_values,
            actual_len: text_values.len(),
        });
    }

    for (i, text_value) in text_values.iter().enumerate() {
        match text_value.parse::<N>() {
            Ok(value) => {
                let index = output.get_mut(i).unwrap();
                *index = value;
            }
            Err(_) => {
                return Err(ParseRealAttributeError::ParseError(text_value.to_string()));
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;
    use proptest::prelude::*;

    impl From<ParseRealAttributeError> for TestCaseError {
        fn from(error: ParseRealAttributeError) -> Self {
            TestCaseError::fail(format!("{}", error))
        }
    }

    proptest! {
        #[test]
        fn parse_two_real_values(real_values in proptest::collection::vec(proptest::num::f32::NORMAL, 2)) {
            let text_attribute = format!("{} {}", real_values[0], real_values[1]);

            let parsed_values = parse_real_vector_attribute::<f32, na::U2>(&text_attribute)?;

            prop_assert_eq!(*parsed_values.get(0).unwrap(), real_values[0]);
            prop_assert_eq!(*parsed_values.get(1).unwrap(), real_values[1]);
        }

        #[test]
        fn parse_three_real_values(real_values in proptest::collection::vec(proptest::num::f32::NORMAL, 2)) {
            let text_attribute = format!("{} {}", real_values[0], real_values[1]);

            let parsed_values = parse_real_vector_attribute::<f32, na::U2>(&text_attribute)?;

            prop_assert_eq!(*parsed_values.get(0).unwrap(), real_values[0]);
            prop_assert_eq!(*parsed_values.get(1).unwrap(), real_values[1]);
        }

        #[test]
        fn parse_incorrect_number_of_attributes(real_values in proptest::collection::vec(proptest::num::f32::NORMAL, 1..10)) {
            prop_assume!(real_values.len() != 2);

            let text_attribute = real_values.iter().map(f32::to_string).collect::<Vec<String>>().join(" ");

            if let Err(error) = parse_real_vector_attribute::<f32, na::U2>(&text_attribute) {
                match error {
                    ParseRealAttributeError::IncorrectNumberOfValues { expected_len, actual_len} => {
                        prop_assert_eq!(expected_len, 2);
                        prop_assert_eq!(actual_len, real_values.len());
                    },
                    _ => {
                        return Err(TestCaseError::fail(format!("Unexpected parsing error {}", error)));
                    }
                }
            } else {
                return Err(TestCaseError::fail("Parsed successfully even though sizes don't match"));
            }
        }

        #[test]
        fn parse_invalid_float(attributes in proptest::collection::vec("[A-Za-z0-9]+", 3)) {
            prop_assume!(attributes[0].parse::<f32>().is_err() ||
                         attributes[1].parse::<f32>().is_err() ||
                         attributes[2].parse::<f32>().is_err());
            let text_attribute = attributes.join(" ");

            if let Err(error) = parse_real_vector_attribute::<f32, na::U3>(&text_attribute) {
                match error {
                    ParseRealAttributeError::ParseError { .. } => {},
                    _ => {
                        return Err(TestCaseError::fail(format!("Unexpected parsing error {}", error)));
                    }
                }
            } else {
                return Err(TestCaseError::fail("Parsed successfully despite invalid floats"));
            }
        }
    }
}
