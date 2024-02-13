#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

const SEQUENCE_SIZE: usize = 14;

trait Letter {
    const N: usize;

    fn to_usize(&self) -> usize;
}

impl Letter for u8 {
    const N: usize = 26;

    fn to_usize(&self) -> usize {
        assert!(self.is_ascii_lowercase());
        *self as usize - b'a' as usize
    }
}

struct State<L: Letter>
where
    [(); L::N]: Sized,
{
    data: [u8; L::N],
}

impl<L> Default for State<L>
where
    L: Letter,
    [(); L::N]: Sized,
{
    fn default() -> Self {
        Self { data: [0; L::N] }
    }
}

impl<L> State<L>
where
    L: Letter,
    [(); L::N]: Sized,
{
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

    let mut state = State::<u8>::default();

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

fn main() {
    dbg!(marker_pos(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::marker_pos;
    use test_case::test_case;

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker(index: usize, input: &str) {
        assert_eq!(marker_pos(input), Some(index));
    }
}
