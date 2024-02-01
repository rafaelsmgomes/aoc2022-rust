mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                    "{} is not a valid item",
                    value as char
                )),
            }
        }
    }

    impl Item {
        pub fn score(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }
}

use std::collections::HashSet;

use item::Item;

fn main() -> color_eyre::Result<()> {
    let mut total_score = 0;

    let input = include_str!("../sample_input.txt");
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()?;

        println!("{first_items:?}");

        let dupe_score = second
            .bytes()
            .map(Item::try_from)
            .find_map(|item| {
                item.ok().and_then(|item| {
                    first_items
                        .iter()
                        .copied()
                        .find(|&first_item| first_item == item)
                })
            })
            .expect("There should be exactly one duplicate")
            .score();

        dbg!(dupe_score);
        total_score += dupe_score;
    }

    dbg!(total_score);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert!(1 == 1);
    }
}
