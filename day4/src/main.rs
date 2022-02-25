use std::num::ParseIntError;
use std::str::Lines;

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();

    // First line - numbers.
    let numbers = lines
        .next()
        .and_then(|s| parse_draw_numbers(s).ok())
        .unwrap();

    let mut boards: Vec<Board> = BoardParser::new(lines).collect();
    let mut scores: Vec<usize> = Vec::new();

    for num in numbers {
        if let Some(score) = boards
            .iter_mut()
            .filter(|b| b.score().is_none())
            .filter_map(|b| b.draw(num))
            .last()
        {
            scores.push(score);
        }
    }

    println!("First score: {}", scores.first().unwrap());
    println!("Last score: {}", scores.last().unwrap());

    Ok(())
}

fn parse_draw_numbers(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
}

struct BoardParser<'a> {
    lines: Lines<'a>,
}

impl<'a> BoardParser<'a> {
    fn new(lines: Lines<'a>) -> Self {
        Self { lines }
    }
}

impl<'a> Iterator for BoardParser<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        // skip an empty line.
        loop {
            let first = self.lines.next();
            if first == Some("") {
            } else if first == None {
                return None;
            } else {
                return [
                    first,
                    self.lines.next(),
                    self.lines.next(),
                    self.lines.next(),
                    self.lines.next(),
                ]
                .iter()
                .flat_map(|row| {
                    row.unwrap()
                        .split_ascii_whitespace()
                        .map(|d| d.parse::<u8>().ok())
                })
                .collect::<Option<Vec<_>>>()
                .map(|numbers| {
                    assert_eq!(25, numbers.len());
                    let mut buf = [0_u8; 25];
                    buf.copy_from_slice(&numbers[..25]);
                    Board::new(buf)
                });
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Board {
    values: [u8; 25],
    flags: u32,
    score: Option<usize>,
}

#[allow(dead_code)]
impl Board {
    fn new(values: [u8; 25]) -> Self {
        Self {
            values,
            flags: 0_u32,
            score: None,
        }
    }

    fn score(&self) -> Option<usize> {
        self.score
    }

    fn draw(&mut self, num: u8) -> Option<usize> {
        self.score().or_else(|| {
            self.score = self.values.iter().position(|&x| x == num).and_then(|pos| {
                self.set_flag(pos);

                if self.has_all_marked() {
                    Some(self.calculate_result(num))
                } else {
                    None
                }
            });
            self.score()
        })
    }

    fn set_flag(&mut self, pos: usize) {
        assert!(pos < 25);
        self.flags |= 1 << pos;
    }

    fn has_all_marked(&self) -> bool {
        [
            // rows
            0b11111,
            0b11111 << 5,
            0b11111 << 10,
            0b11111 << 15,
            0b11111 << 20,
            // cols
            0b100001000010000100001,
            0b100001000010000100001 << 1,
            0b100001000010000100001 << 2,
            0b100001000010000100001 << 3,
            0b100001000010000100001 << 4,
        ]
        .iter()
        .any(|&mask| ((self.flags & mask) == mask))
    }

    fn calculate_result(&self, num: u8) -> usize {
        num as usize
            * self
                .values
                .iter()
                .enumerate()
                .filter(|(pos, _)| (self.flags & (1 << pos)) == 0)
                .map(|(_, &v)| v as usize)
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use crate::*;

    #[test]
    fn test_board_rows() {
        let drawer = |r: Range<u8>| {
            let mut board = Board::new([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24,
            ]);

            r.map(|x| board.draw(x)).last().flatten()
        };

        let summator = |r: Range<u8>| {
            Some(
                (r.end - 1) as usize
                    * ((0..25).sum::<usize>() - r.map(|x| x as usize).sum::<usize>()),
            )
        };

        assert_eq!(summator(0..5), drawer(0..5));
        assert_eq!(summator(5..10), drawer(5..10));
        assert_eq!(summator(10..15), drawer(10..15));
        assert_eq!(summator(15..20), drawer(15..20));
        assert_eq!(summator(20..25), drawer(20..25));
    }

    #[test]
    fn test_board_columns() {
        let drawer = |r: [u8; 5]| {
            let mut board = Board::new([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24,
            ]);

            r.iter().map(|&x| board.draw(x)).last().flatten()
        };

        let summator = |r: [u8; 5]| {
            Some(
                (r[4]) as usize
                    * ((0..25).sum::<usize>() - r.iter().map(|&x| x as usize).sum::<usize>()),
            )
        };

        let verify = |r: [u8; 5]| assert_eq!(summator(r), drawer(r));

        verify([0, 5, 10, 15, 20]);
        verify([1, 6, 11, 16, 21]);
        verify([2, 7, 12, 17, 22]);
        verify([3, 8, 13, 18, 23]);
        verify([4, 9, 14, 19, 24]);
    }
}
