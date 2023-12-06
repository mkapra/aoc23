pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));
            let first_num = it.next().expect("Should be a number");
            match it.last() {
                Some(last) => first_num * 10 + last,
                None => first_num * 10 + first_num,
            }
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

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
