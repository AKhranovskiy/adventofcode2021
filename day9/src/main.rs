use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let heightmap = Heightmap::from(include_str!("input.txt"));

    println!("Part 1: {}", part_1(&heightmap));
    println!("Part 2: {}", part_2(&heightmap));

    Ok(())
}

fn part_1(heightmap: &Heightmap) -> usize {
    heightmap
        .iter()
        .filter(|pos| pos.min_amongst_neighbours())
        .map(|pos| pos.risk_level())
        .sum::<usize>()
}

fn part_2(heightmap: &Heightmap) -> usize {
    heightmap
        .low_points()
        .map(|pos| pos.basin().len())
        .sorted()
        .rev()
        .take(3)
        .reduce(|x, y| x * y)
        .unwrap_or_default()
}

#[derive(Debug)]
struct Heightmap {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>,
}

impl Heightmap {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Self {
            width: data.get(0).map(|x| x.len()).unwrap_or_default(),
            height: data.len(),
            data,
        }
    }

    fn iter(&self) -> HeightmapIterator<'_> {
        HeightmapIterator {
            position: Position::new(0, 0, self),
        }
    }

    fn value(&self, col: usize, row: usize) -> Option<u8> {
        self.data.get(row).and_then(|x| x.get(col).copied())
    }

    fn low_points(&self) -> impl Iterator<Item = Position<'_>> {
        self.iter().filter(|pos| pos.min_amongst_neighbours())
    }
}

impl From<&str> for Heightmap {
    fn from(s: &str) -> Self {
        Heightmap::new(
            s.trim()
                .lines()
                .map(|s| {
                    s.chars()
                        .map(|c| c.to_digit(10).map(|d| d as u8).ok_or(()))
                        .collect::<Result<Vec<u8>, _>>()
                })
                .collect::<Result<Vec<Vec<u8>>, _>>()
                .unwrap_or_default(),
        )
    }
}

#[derive(Debug, Copy, Clone)]
struct Position<'h> {
    col: usize,
    row: usize,
    heightmap: &'h Heightmap,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl<'h> Position<'h> {
    fn new(col: usize, row: usize, heightmap: &'h Heightmap) -> Self {
        assert!(
            col < heightmap.width,
            "col={} must be less than width={}",
            col,
            heightmap.width
        );

        assert!(
            row < heightmap.height,
            "row={} must be less than height={}",
            row,
            heightmap.height
        );

        Self {
            col,
            row,
            heightmap,
        }
    }
    fn value(&self) -> Option<u8> {
        self.heightmap.value(self.col, self.row)
    }

    fn risk_level(&self) -> usize {
        self.value().unwrap_or_default() as usize + 1
    }

    fn moved(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Left => self
                .col
                .checked_sub(1)
                .map(|col| Position::new(col, self.row, self.heightmap)),
            Direction::Up => self
                .row
                .checked_sub(1)
                .map(|row| Position::new(self.col, row, self.heightmap)),
            Direction::Right => {
                if self.col + 1 < self.heightmap.width {
                    Some(Position::new(self.col + 1, self.row, self.heightmap))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.row + 1 < self.heightmap.height {
                    Some(Position::new(self.col, self.row + 1, self.heightmap))
                } else {
                    None
                }
            }
        }
    }

    fn neighbours(&self) -> Vec<Position> {
        [
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ]
        .iter()
        .filter_map(|d| self.moved(*d))
        .collect()
    }

    fn min_amongst_neighbours(&self) -> bool {
        self.neighbours()
            .iter()
            .min_by(|a, b| a.value().cmp(&b.value()))
            .map_or(false, |m| m.value() > self.value())
    }

    fn pos(&self) -> (usize, usize) {
        (self.col, self.row)
    }
    fn visit(&self, nodes: &mut HashSet<(usize, usize)>) {
        if !nodes.contains(&self.pos()) {
            nodes.insert(self.pos());
            let n = self
                .neighbours()
                .iter()
                .copied()
                .filter(|pos| pos.value().unwrap_or_default() != 9 && !nodes.contains(&pos.pos()))
                .collect::<Vec<_>>();
            n.iter().for_each(|pos| pos.visit(nodes))
        }
    }
    fn basin(&self) -> Vec<Position<'_>> {
        let mut nodes = HashSet::new();

        self.visit(&mut nodes);
        Vec::from_iter(
            nodes
                .into_iter()
                .map(|p| Position::new(p.0, p.1, self.heightmap)),
        )
    }
}

impl<'h> PartialEq for Position<'h> {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}

impl<'h> Eq for Position<'h> {}

impl<'h> Hash for Position<'h> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.col.hash(state);
        self.row.hash(state);
    }
}

struct HeightmapIterator<'h> {
    position: Position<'h>,
}

impl<'h> Iterator for HeightmapIterator<'h> {
    type Item = Position<'h>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.position;
        let next = Position {
            col: (pos.col + 1) % pos.heightmap.width,
            row: pos.row + (pos.col + 1) / pos.heightmap.width,
            heightmap: pos.heightmap,
        };

        if next.row < next.heightmap.height {
            self.position = next;
            Some(pos)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    use crate::part_2;
    use crate::Direction;
    use crate::Heightmap;
    use crate::Position;

    const DATA: &str = indoc::indoc!(
        "
        2199943210
        3987894921
        9856789892
        8767896789
        98999656788
        "
    );

    fn data() -> &'static str {
        DATA.trim()
    }

    #[test]
    fn test_from() {
        let heightmap = Heightmap::from(data());
        assert_eq!(10, heightmap.width);
        assert_eq!(5, heightmap.height);
    }

    #[test]
    fn test_iterator() {
        let heightmap = Heightmap::from(data());
        let mut iter = heightmap.iter();

        assert_eq!(Some((0, 0)), iter.next().map(|x| (x.row, x.col)));
        assert_eq!(Some((0, 1)), iter.next().map(|x| (x.row, x.col)));
        assert_eq!(Some(9), iter.next().and_then(|x| x.value()));
    }

    #[test]
    fn test_position() {
        let heightmap = Heightmap::from(data());
        let pos = Position::new(0, 0, &heightmap);

        assert_eq!(Some(2), pos.value());
        assert_eq!(None, pos.moved(Direction::Up));
        assert_eq!(None, pos.moved(Direction::Left));

        assert_eq!(
            Some(Position::new(1, 0, &heightmap)),
            pos.moved(Direction::Right)
        );
        assert_eq!(
            Some(Position::new(0, 1, &heightmap)),
            pos.moved(Direction::Down)
        );

        let pos = Position::new(9, 4, &heightmap);
        assert_eq!(Some(8), pos.value());
        assert_eq!(None, pos.moved(Direction::Right));
        assert_eq!(None, pos.moved(Direction::Down));
    }

    #[test]
    fn test_neighbours() {
        let heightmap = Heightmap::from(data());

        // Top left position.
        assert_eq!(
            [
                Position::new(1, 0, &heightmap),
                Position::new(0, 1, &heightmap)
            ]
            .as_ref(),
            &Position::new(0, 0, &heightmap).neighbours()
        );

        // Top right position.
        assert_eq!(
            [
                Position::new(8, 0, &heightmap),
                Position::new(9, 1, &heightmap)
            ]
            .as_ref(),
            &Position::new(9, 0, &heightmap).neighbours()
        );

        // Bottom right position.
        assert_eq!(
            [
                Position::new(8, 4, &heightmap),
                Position::new(9, 3, &heightmap)
            ]
            .as_ref(),
            &Position::new(9, 4, &heightmap).neighbours()
        );

        // Bottom left position.
        assert_eq!(
            [
                Position::new(0, 3, &heightmap),
                Position::new(1, 4, &heightmap)
            ]
            .as_ref(),
            &Position::new(0, 4, &heightmap).neighbours()
        );

        // Middle position.
        assert_eq!(
            [
                Position::new(3, 2, &heightmap),
                Position::new(4, 1, &heightmap),
                Position::new(5, 2, &heightmap),
                Position::new(4, 3, &heightmap),
            ]
            .as_ref(),
            &Position::new(4, 2, &heightmap).neighbours()
        );
    }

    #[test]
    fn test_min_amongst_neighbours() {
        let heightmap = Heightmap::from(data());
        assert!(Position::new(1, 0, &heightmap).min_amongst_neighbours());
        assert!(Position::new(9, 0, &heightmap).min_amongst_neighbours());
        assert!(Position::new(2, 2, &heightmap).min_amongst_neighbours());
        assert!(Position::new(6, 4, &heightmap).min_amongst_neighbours());

        assert!(!Position::new(0, 0, &heightmap).min_amongst_neighbours());
        assert!(!Position::new(1, 1, &heightmap).min_amongst_neighbours());
        assert!(!Position::new(9, 2, &heightmap).min_amongst_neighbours());
        assert!(!Position::new(2, 3, &heightmap).min_amongst_neighbours());
    }

    #[test]
    fn test_low_points() {
        let heightmap = Heightmap::from(data());
        let mut points = heightmap.low_points();

        assert_eq!(Position::new(1, 0, &heightmap), points.next().unwrap());
        assert_eq!(Position::new(9, 0, &heightmap), points.next().unwrap());
        assert_eq!(Position::new(2, 2, &heightmap), points.next().unwrap());
        assert_eq!(Position::new(6, 4, &heightmap), points.next().unwrap());
        assert_eq!(None, points.next());
    }

    #[test]
    fn test_part_1() {
        let heightmap = Heightmap::from(data());
        assert_eq!(15, part_1(&heightmap));
    }

    #[test]
    fn test_basin() {
        let heightmap = Heightmap::from(data());
        assert_eq!(3, Position::new(1, 0, &heightmap).basin().len());
        assert_eq!(9, Position::new(9, 0, &heightmap).basin().len());
        assert_eq!(14, Position::new(2, 2, &heightmap).basin().len());
        assert_eq!(9, Position::new(6, 4, &heightmap).basin().len());
    }

    #[test]
    fn test_part_2() {
        let heightmap = Heightmap::from(data());
        assert_eq!(1134, part_2(&heightmap));
    }
}
