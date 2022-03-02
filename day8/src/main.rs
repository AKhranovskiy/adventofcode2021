use std::str::Lines;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").trim().lines();

    println!("Part 1: {}", part_1(lines.clone()));
    println!("Part 2: {}", part_2(lines.clone()));
    Ok(())
}

fn part_1(lines: Lines) -> usize {
    lines
        .map(|line| {
            line.split(" | ").last().map_or(0, |line| {
                line.split_ascii_whitespace()
                    .map(|s| s.len())
                    .filter(|x| [2, 4, 3, 7].contains(x))
                    .count()
            })
        })
        .sum::<usize>()
}

fn part_2(lines: Lines) -> usize {
    let parse = |w: &str| {
        w.chars()
            .map(|c| 1_u16 << (c as u16 - 'a' as u16))
            .reduce(|acc, x| acc | x)
            .unwrap_or_default()
    };

    lines
        .map(|line| {
            let mut chunks = line.split(" | ");

            let patterns = chunks
                .next()
                .unwrap_or_default()
                .split_ascii_whitespace()
                .filter(|&w| w.len() == 2 || w.len() == 4)
                .map(parse)
                .collect::<Vec<_>>();

            let one = patterns.iter().find(|&d| d.count_ones() == 2).unwrap();
            let four = patterns.iter().find(|&d| d.count_ones() == 4).unwrap();

            chunks
                .next()
                .unwrap_or_default()
                .split_ascii_whitespace()
                .map(parse)
                .map(|d: u16| {
                    match (
                        d.count_ones(),
                        (d & four).count_ones(),
                        (d & one).count_ones(),
                    ) {
                        (2, _, _) => 1,
                        (3, _, _) => 7,
                        (4, _, _) => 4,
                        (5, 2, _) => 2,
                        (5, 3, 1) => 5,
                        (5, 3, 2) => 3,
                        (6, 3, 1) => 6,
                        (6, 3, 2) => 0,
                        (6, 4, _) => 9,
                        (7, 4, 2) => 8,
                        _ => unreachable!("Unknown pattern"),
                    }
                })
                .fold(0, |acc, x| acc * 10 + x)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part_1, part_2};

    const TEST: &str = r#"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_part_1() {
        assert_eq!(26, part_1(TEST.lines()));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(66582, part_2(TEST.lines()));
    }
}
