fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|count| {
        let partial_line = &line[count..];

        let res = if partial_line.starts_with("one") {
            '1'
        } else if partial_line.starts_with("two") {
            '2'
        } else if partial_line.starts_with("three") {
            '3'
        } else if partial_line.starts_with("four") {
            '4'
        } else if partial_line.starts_with("five") {
            '5'
        } else if partial_line.starts_with("six") {
            '6'
        } else if partial_line.starts_with("seven") {
            '7'
        } else if partial_line.starts_with("eight") {
            '8'
        } else if partial_line.starts_with("nine") {
            '9'
        } else {
            partial_line.chars().next().unwrap()
        };

        res.to_digit(10)
    });
    let first_num = it.next().expect("Should contain a value");
    match it.last() {
        Some(last) => first_num * 10 + last,
        None => first_num * 10 + first_num,
    }
}

pub fn solve(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_test_input() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(
            solve(input),
            281,
            "The sum of all calibration values does not match the expected result"
        )
    }

    #[test]
    fn test_input() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(
            solve(input),
            142,
            "The sum of all calibration values does not match the expected result"
        )
    }
}
