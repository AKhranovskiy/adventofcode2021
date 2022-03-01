use memoize::memoize;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1 - 80 days: {}", solution(&input, 80));
    println!("Part 1 - 256 days: {}", solution(&input, 256));

    Ok(())
}

fn solution(initial: &[u8], days: usize) -> usize {
    initial
        .iter()
        .map(|&age| calculate_fishes(days.checked_sub(1 + age as usize)))
        .sum()
}

#[memoize]
fn calculate_fishes(days: Option<usize>) -> usize {
    days.map_or(1, |days| {
        calculate_fishes(days.checked_sub(7)) + calculate_fishes(days.checked_sub(9))
    })
}

#[cfg(test)]
mod tests {
    use crate::solution;

    const TEST: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test_solution() {
        assert_eq!(26, solution(&TEST, 18));
        assert_eq!(5934, solution(&TEST, 80));
    }
}
