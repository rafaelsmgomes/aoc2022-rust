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
    input
        .as_bytes()
        .windows(4)
        .position(|window| {
            window
                .iter()
                .map(|c| c.to_u32_for_bitset())
                .fold(0, |acc, x| acc | x)
                .count_ones()
                == 4
        })
        .map(|pos| pos + 4)
}

const SEQUENCE_SIZE: usize = 14;

struct State {
    data: [u8; 256],
}

impl Default for State {
    fn default() -> Self {
        Self { data: [0; 256] }
    }
}

impl State {
    fn push(&mut self, c: u8) {
        self.data[c as usize] = self.data[c as usize].checked_add(1).unwrap();
    }
    fn pop(&mut self, c: u8) {
        self.data[c as usize] = self.data[c as usize].checked_sub(1).unwrap();
    }
    fn is_unique(&self) -> bool {
        self.data.iter().all(|&x| x <= 1)
    }
}

fn marker_pos(input: &str) -> Option<usize> {
    assert!(input.len() > SEQUENCE_SIZE);

    let mut state = State::default();
    input
        .bytes()
        .take(SEQUENCE_SIZE)
        .for_each(|c| state.push(c));

    if state.is_unique() {
        return Some(0);
    }

    for (index, window) in input.as_bytes().windows(SEQUENCE_SIZE + 1).enumerate() {
        let removed = window[0];
        let added = window[SEQUENCE_SIZE];
        state.pop(removed);
        state.push(added);

        if state.is_unique() {
            return Some(index + 1);
        }
    }

    None
}

fn message_start(input: &str) -> usize {
    const PREV_SIZE: usize = 13;

    let mut prev = [' '; PREV_SIZE];
    prev.copy_from_slice(&input.chars().collect::<Vec<_>>()[..PREV_SIZE]);
    for (ix, c) in input.chars().skip(PREV_SIZE).enumerate() {
        if !prev.contains(&c) && is_unique(&prev) {
            return ix + PREV_SIZE + 1;
        } else {
            prev[ix % PREV_SIZE] = c;
        }
    }
    unreachable!("Input contains no message marker")
}

fn main() {
    dbg!(message_start(include_str!("input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker(index: usize, input: &str) {
        assert_eq!(Some(index), find_marker(input));
    }
}
