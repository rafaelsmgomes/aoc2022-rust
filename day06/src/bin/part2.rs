trait LowercaseLetter {
    fn to_u32_for_bitset(&self) -> u32;
}

impl LowercaseLetter for u8 {
    fn to_u32_for_bitset(&self) -> u32 {
        assert!(self.is_ascii_lowercase());
        1 << (*self as u32 - 'a' as u32)
    }
}

fn find_marker(input: &str) -> Option<usize> {
    const SEQUENCE_SIZE: usize = 14;
    input
        .as_bytes()
        .windows(SEQUENCE_SIZE)
        .position(|window| {
            window
                .iter()
                .map(|c| c.to_u32_for_bitset())
                .fold(0, |acc, x| acc | x)
                .count_ones() as usize
                == SEQUENCE_SIZE
        })
        .map(|pos| pos + SEQUENCE_SIZE)
}

fn main() {
    dbg!(find_marker(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker(index: usize, input: &str) {
        assert_eq!(find_marker(input), Some(index));
    }
}
