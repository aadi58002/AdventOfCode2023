use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<usize, AocError> {
    let mut count = 0;
    for line in input.lines() {
        count += line
            .chars()
            .skip_while(|ch| !ch.is_digit(10))
            .next()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap()
            * 10
            + line
                .chars()
                .rev()
                .skip_while(|ch| !ch.is_digit(10))
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
