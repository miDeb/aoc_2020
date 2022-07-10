use std::collections::VecDeque;

use fxhash::FxHashSet;

#[aoc(day22, part1)]
fn part1(input: &str) -> usize {
    let mut decks = parse(input);
    let winning_deck = loop {
        step(&mut decks);
        if decks[0].is_empty() {
            break &decks[1];
        }
        if decks[1].is_empty() {
            break &decks[0];
        }
    };
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| val * (idx as usize + 1))
        .sum()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> usize {
    let decks = parse(input);
    let mut game = Game2::new(decks.clone());
    let winning_deck = match game.play() {
        Player::One => &game.current_positions[0],
        Player::Two => &game.current_positions[1],
    };
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| val * (idx as usize + 1))
        .sum()
}

fn parse(input: &str) -> [VecDeque<usize>; 2] {
    let mut decks = [VecDeque::new(), VecDeque::new()];
    let mut deck = 0;
    for line in input.lines() {
        if line.is_empty() {
            deck += 1;
            continue;
        }
        if line.starts_with('P') {
            // Player n:
            assert!(decks[deck].is_empty());
            continue;
        }
        decks[deck].push_back(line.parse().unwrap());
    }
    decks
}

fn step(decks: &mut [VecDeque<usize>; 2]) {
    let first = decks[0].pop_front().unwrap();
    let second = decks[1].pop_front().unwrap();
    if first > second {
        decks[0].push_back(first);
        decks[0].push_back(second);
    } else {
        assert_ne!(first, second);

        decks[1].push_back(second);
        decks[1].push_back(first);
    }
}

type RoundState = [VecDeque<usize>; 2];

enum Player {
    One,
    Two,
}

struct Game2 {
    encountered_positions: FxHashSet<RoundState>,
    current_positions: RoundState,
}

impl Game2 {
    fn new(current_positions: RoundState) -> Self {
        Self {
            current_positions,
            encountered_positions: FxHashSet::default(),
        }
    }

    fn step(&mut self) -> Option<Player> {
        if self.encountered_positions.contains(&self.current_positions) {
            return Some(Player::One);
        }
        self.encountered_positions
            .insert(self.current_positions.clone());

        let first = self.current_positions[0].pop_front().unwrap();
        let second = self.current_positions[1].pop_front().unwrap();

        let winner = if self.current_positions[0].len() >= first
            && self.current_positions[1].len() >= second
        {
            Game2::new([
                self.current_positions[0]
                    .iter()
                    .take(first)
                    .copied()
                    .collect(),
                self.current_positions[1]
                    .iter()
                    .take(second)
                    .copied()
                    .collect(),
            ])
            .play()
        } else if first > second {
            Player::One
        } else {
            assert_ne!(first, second);
            Player::Two
        };

        match winner {
            Player::One => {
                self.current_positions[0].push_back(first);
                self.current_positions[0].push_back(second);
                if self.current_positions[1].is_empty() {
                    return Some(Player::One);
                }
            }
            Player::Two => {
                self.current_positions[1].push_back(second);
                self.current_positions[1].push_back(first);
                if self.current_positions[0].is_empty() {
                    return Some(Player::Two);
                }
            }
        }

        None
    }

    fn play(&mut self) -> Player {
        loop {
            if let Some(winner) = self.step() {
                return winner;
            }
        }
    }
}

#[test]
fn e1() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
    assert_eq!(part1(input), 306);
}

#[test]
fn e2() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
    assert_eq!(part2(input), 291);
}

#[test]
fn no_infinite_loop() {
    let input = "Player 1:
43
19

Player 2:
2
29
14
";
    assert_eq!(part2(input), 105);
}
