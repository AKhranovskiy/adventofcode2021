use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let result = include_str!("input.txt")
        .split('\n')
        .filter_map(|s| s.parse::<usize>().ok())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .fold((0_usize, usize::MAX), |acc, v| {
            (acc.0 + (v > acc.1) as usize, v)
        })
        .0;

    println!("{}", result);

    Ok(())
}
