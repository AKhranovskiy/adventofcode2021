fn main() -> anyhow::Result<()> {
    let positions = include_str!("input.txt")
    .split('\n')
    .filter_map(|line| {
        let mut s = line.split(' ');
        let direction = s.next()?;
        let value = s.next()?.parse::<usize>().ok()?;
       Some((direction, value))}
    )
    .fold((0_usize, 0_usize, 0_usize), |(hor, ver, aim), (direction, value)| match direction {
        "forward" => (hor + value, ver + aim * value, aim),
        "up" => (hor, ver, aim - value),
        "down" => (hor, ver, aim + value),
        _ => (hor, ver, aim)
    });

    println!("{}", positions.0 * positions.1);

    Ok(())
}
