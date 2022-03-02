use median::Filter;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse::<u16>())
        .collect::<Result<Vec<_>, _>>()?;

    let consumption = consumption_const(&input, median(&input));
    println!("consumption const: {}", consumption);

    let target = mean(&input);
    let consumption =
        consumption_linear(&input, target.0).min(consumption_linear(&input, target.1));

    println!("consumption linear: {}", consumption);

    Ok(())
}

fn median(data: &[u16]) -> u16 {
    let mut filter = Filter::new(data.len());
    data.iter()
        .cloned()
        .reduce(|_, value| filter.consume(value))
        .unwrap_or(0)
}

fn mean(data: &[u16]) -> (u16, u16) {
    let m = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
    (m.floor().trunc() as _, m.ceil().trunc() as _)
}

fn consumption_const(data: &[u16], target: u16) -> usize {
    data.iter()
        .map(|&x| {
            x.checked_sub(target)
                .or_else(|| target.checked_sub(x))
                .unwrap_or(0) as usize
        })
        .sum()
}

fn consumption_linear(data: &[u16], target: u16) -> usize {
    data.iter()
        .map(|&x| {
            let steps = x
                .checked_sub(target)
                .or_else(|| target.checked_sub(x))
                .unwrap_or(0) as usize;
            steps * (steps + 1) / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{consumption_const, consumption_linear, mean, median};
    const TEST: [u16; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_median() {
        assert_eq!(2, median(&TEST));
    }

    #[test]
    fn test_consumption_const() {
        assert_eq!(37, consumption_const(&TEST, 2));
    }

    #[test]
    fn test_mean() {
        assert_eq!((4, 5), mean(&TEST));
    }

    #[test]
    fn test_consumption_linear() {
        assert_eq!(168, consumption_linear(&TEST, 5));
        // assert_eq!(206, consumption_linear(&TEST, 2));
    }
}
