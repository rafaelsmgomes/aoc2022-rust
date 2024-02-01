// A Y -> A = rocks, Y = Paper
// B X -> B = paper, X = Rock
// C Z -> C = Scissors, Z = Scissors
//
// Win = 6 points, Draw = 3, Lost = 0

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
struct Round {
    ours: Move,
    theirs: Move,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

impl Move {
    fn inherent_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    fn outcome(self, theirs: Self) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loose
        } else {
            Outcome::Draw
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}

impl Outcome {
    fn inherent_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0,
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(theirs), Some(' '), Some(ours), None) =
            (chars.next(), chars.next(), chars.next(), chars.next())
        else {
            return Err(color_eyre::eyre::eyre!(
                "expected <theirs>SP<ours>EOF, got {s:?}"
            ));
        };

        Ok(Self {
            ours: ours.try_into()?,
            theirs: theirs.try_into()?,
        })
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.ours.outcome(self.theirs)
    }

    fn our_score(self) -> usize {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let total_score: usize = itertools::process_results(
        include_str!("../input.txt")
            .lines()
            .map(Round::from_str)
            .map(|r| r.map(|r| r.our_score())),
        |it| it.sum(),
    )?;

    dbg!(total_score);

    Ok(())
}
