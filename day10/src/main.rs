use std::fmt::Display;

use itertools::Itertools;
use median::Filter;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .trim()
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

fn part_1(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|&line| {
            if let Err(MatchingError::InvalidMatch(_, chunk)) = TryInto::<Line>::try_into(line) {
                chunk.score
            } else {
                0_u32
            }
        })
        .sum()
}

fn part_2(lines: &[&str]) -> u64 {
    let mut filter = Filter::new(lines.len());
    lines.iter().for_each(|&line| {
        if let Err(MatchingError::Incomplete(ref stack)) = TryInto::<Line>::try_into(line) {
            let score = stack
                .iter()
                .rev()
                .fold(0_u64, |acc, chunk| acc * 5_u64 + chunk.score2 as u64);
            filter.consume(score);
        }
    });

    filter.median()
}

#[derive(Debug, Clone, PartialEq)]
struct Line;

impl TryFrom<&str> for Line {
    type Error = MatchingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut stack = Vec::<Chunk>::with_capacity(value.len());

        value
            .chars()
            .try_for_each(|c| match c.try_into() {
                Ok(Side::Open(chunk)) => {
                    stack.push(chunk);
                    Ok(())
                }
                Ok(Side::Close(chunk)) => {
                    if let Some(open) = stack.pop() {
                        if open == chunk {
                            Ok(())
                        } else {
                            Err(MatchingError::InvalidMatch(open, chunk))
                        }
                    } else {
                        Err(MatchingError::InvalidMatch(chunk, chunk))
                    }
                }
                Err(e) => Err(e),
            })
            .and({
                if stack.is_empty() {
                    Ok(())
                } else {
                    Err(MatchingError::Incomplete(stack))
                }
            })
            .map(|_| Line {})
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MatchingError {
    Incomplete(Vec<Chunk>),
    InvalidMatch(Chunk, Chunk),
    InvalidSymbol(char),
}

impl Display for MatchingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MatchingError::Incomplete(ref stack) => {
                write!(
                    f,
                    "Incomplete input, stack={:?}",
                    stack.iter().map(|chunk| chunk.close).collect::<String>()
                )
            }
            MatchingError::InvalidMatch(expected, given) => {
                write!(
                    f,
                    "Invalid match, expected={}, given={}",
                    expected.close, given.close
                )
            }
            MatchingError::InvalidSymbol(c) => write!(f, "Invalid symbol '{}'", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Chunk {
    open: char,
    close: char,
    score: u32,
    score2: u32,
}

impl Chunk {
    const fn new(open: char, close: char, score: u32, score2: u32) -> Self {
        Self {
            open,
            close,
            score,
            score2,
        }
    }

    const fn side(&self, c: char) -> Option<Side> {
        if c == self.open {
            Some(Side::Open(*self))
        } else if c == self.close {
            Some(Side::Close(*self))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Side {
    Open(Chunk),
    Close(Chunk),
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Side::Open(ref chunk) => write!(f, "'{}'", chunk.open),
            Side::Close(ref chunk) => write!(f, "'{}'", chunk.close),
        }
    }
}

impl TryFrom<char> for Side {
    type Error = MatchingError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        ROUND
            .side(value)
            .or_else(|| SQUARE.side(value))
            .or_else(|| CURLY.side(value))
            .or_else(|| ANGLE.side(value))
            .ok_or(MatchingError::InvalidSymbol(value))
    }
}

const ROUND: Chunk = Chunk::new('(', ')', 3, 1);
const SQUARE: Chunk = Chunk::new('[', ']', 57, 2);
const CURLY: Chunk = Chunk::new('{', '}', 1197, 3);
const ANGLE: Chunk = Chunk::new('<', '>', 25137, 4);

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2, Line, MatchingError, Side, ANGLE, CURLY, ROUND, SQUARE};
    #[test]
    fn test_parse_side() {
        assert_eq!(Side::Open(ROUND), '('.try_into().unwrap());
        assert_eq!(Side::Close(ROUND), ')'.try_into().unwrap());
        assert_eq!(Side::Open(SQUARE), '['.try_into().unwrap());
        assert_eq!(Side::Close(SQUARE), ']'.try_into().unwrap());
        assert_eq!(Side::Open(CURLY), '{'.try_into().unwrap());
        assert_eq!(Side::Close(CURLY), '}'.try_into().unwrap());
        assert_eq!(Side::Open(ANGLE), '<'.try_into().unwrap());
        assert_eq!(Side::Close(ANGLE), '>'.try_into().unwrap());

        assert_eq!(
            MatchingError::InvalidSymbol('/'),
            TryInto::<Side>::try_into('/').unwrap_err()
        );
    }

    type MatchingResult = Result<Line, MatchingError>;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            MatchingResult::Err(MatchingError::Incomplete(vec![
                SQUARE, ROUND, CURLY, ROUND, SQUARE, SQUARE, CURLY, CURLY
            ])),
            "[({(<(())[]>[[{[]{<()<>>".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::Incomplete(vec![
                ROUND, CURLY, SQUARE, ANGLE, CURLY, ROUND
            ])),
            "[(()[<>])]({[<{<<[]>>(".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::InvalidMatch(SQUARE, CURLY)),
            "{([(<{}[<>[]}>{[]{[(<()>".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::Incomplete(vec![
                ROUND, ROUND, ROUND, ROUND, ANGLE, CURLY, ANGLE, CURLY, CURLY
            ])),
            "(((({<>}<{<{<>}{[]{[]{}".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::InvalidMatch(SQUARE, ROUND)),
            "[[<[([]))<([[{}[[()]]]".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::InvalidMatch(ROUND, SQUARE)),
            "[{[{({}]{}}([{[{{{}}([]".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::Incomplete(vec![
                ANGLE, CURLY, SQUARE, CURLY, SQUARE, CURLY, CURLY, SQUARE, SQUARE
            ])),
            "{<[[]]>}<{[{[{[]{()[[[]".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::InvalidMatch(ANGLE, ROUND)),
            "[<(<(<(<{}))><([]([]()".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::InvalidMatch(SQUARE, ANGLE)),
            "<{([([[(<>()){}]>(<<{{".try_into()
        );
        assert_eq!(
            MatchingResult::Err(MatchingError::Incomplete(vec![ANGLE, CURLY, ROUND, SQUARE])),
            "<{([{{}}[<[[[<>{}]]]>[]]".try_into()
        );
    }

    #[test]
    fn test_part_1() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        assert_eq!(26397, part_1(&lines));
    }

    #[test]
    fn test_part_2() {
        let lines = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];
        assert_eq!(288957, part_2(&lines));
    }
}
