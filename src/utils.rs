use count_digits::CountDigits;
pub fn split_number(num: u64) -> (u64, u64) {
    let digit_count = num.count_digits();
    let half_digits = digit_count / 2;
    let divisor = 10_u64.pow(half_digits as u32);
    let right = num % divisor;
    let left = num / divisor;
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1234), (12, 34));
        assert_eq!(split_number(1001), (10, 1));
    }
}
