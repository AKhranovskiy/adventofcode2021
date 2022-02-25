use std::{num::ParseIntError, str::Lines};

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();

    // First line - numbers.
    let numbers = lines
        .next()
        .and_then(|s| parse_draw_numbers(s).ok())
        .unwrap();

    // while lines.next() {}
    Ok(())
}

fn parse_draw_numbers(input: &str) -> Result<Vec<u8>, ParseIntError> {
    input
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
}

type Board = [Option<u8>; 25];

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
        while self.lines.next() == Some("") {}

        [
            self.lines.next(),
            self.lines.next(),
            self.lines.next(),
            self.lines.next(),
            self.lines.next(),
        ]
        .map(|row| row.and_then(|r| r.split_ascii_whitespace().map(|d| d.parse::<u8>().ok()).collect::<Vec<_>,_>>()))
        .
    }
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use crate::*;

    const TEST_DATA: &'static str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    fn lines() -> Lines<'static> {
        TEST_DATA.lines()
    }

    #[test]
    fn test_parse_draw_numbers() {
        let data = lines().next().unwrap();
        assert_eq!(
            &vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            &parse_draw_numbers(&data).ok().unwrap()
        )
    }

    #[test]
    fn playground() {
        let mut lines = lines();
        lines.next();
        lines.next();

        println!("{:?}", lines.next());
        println!("{:?}", lines.next());
        println!("{:?}", lines.next());
        println!("{:?}", lines.next());
        println!("{:?}", lines.next());

        lines.next();

        println!("{:?}", lines.next());
        println!("{:?}", lines.next());
        println!("{:?}", lines.next());
        println!("{:?}", lines.next());
        println!("{:?}", lines.next());

        assert!(false)
    }
}
