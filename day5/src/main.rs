#![feature(step_trait)]

use std::collections::HashMap;
use std::str::Lines;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");

    let mut overlaps: HashMap<Point, usize> = HashMap::new();
    Parser::new(input.lines())
        // uncomment for part 1
        // .filter(|r| match r.orientation() {
        //     Orientation::Horizontal => true,
        //     Orientation::Vertical => true,
        //     Orientation::Other => false,
        // })
        .flat_map(|r| r.iter())
        .for_each(|p| *overlaps.entry(p).or_insert(0) += 1);

    let overlaps = overlaps.iter().filter(|(_, &count)| count >= 2).count();

    println!("Overlaps: {}", overlaps);

    Ok(())
}

struct Parser<'a> {
    lines: Lines<'a>,
}

impl<'a> Parser<'a> {
    fn new(lines: Lines<'a>) -> Self {
        Self { lines }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = PointRange;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|line| {
            let points = line
                .split(" -> ")
                .map(|parts| {
                    let numbers = parts
                        .split(',')
                        .map(|number| number.parse::<u16>().ok())
                        .collect::<Option<Vec<u16>>>()
                        .unwrap();
                    Point::new(*numbers.first().unwrap(), *numbers.last().unwrap())
                })
                .collect::<Vec<_>>();

            PointRange::new(*points.first().unwrap(), *points.last().unwrap())
        })
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    fn advance_towards(self, other: Point) -> Point {
        let adv = |current: u16, target: u16| match current.cmp(&target) {
            std::cmp::Ordering::Less => current + 1,
            std::cmp::Ordering::Equal => current,
            std::cmp::Ordering::Greater => current - 1,
        };

        Point {
            x: adv(self.x, other.x),
            y: adv(self.y, other.y),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct PointRange {
    start: Point,
    end: Point,
}

#[allow(dead_code)]
enum Orientation {
    Horizontal,
    Vertical,
    Other,
}

impl PointRange {
    fn new(start: Point, end: Point) -> PointRange {
        Self { start, end }
    }

    fn iter(&self) -> PointRangeIterator {
        PointRangeIterator {
            range: *self,
            current: Some(self.start),
            finished: false,
        }
    }

    #[allow(dead_code)]
    fn orientation(&self) -> Orientation {
        if self.start.x == self.end.x {
            Orientation::Vertical
        } else if self.start.y == self.end.y {
            Orientation::Horizontal
        } else {
            Orientation::Other
        }
    }
}

struct PointRangeIterator {
    range: PointRange,
    current: Option<Point>,
    finished: bool,
}

impl Iterator for PointRangeIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.finished = current == self.range.end;

            self.current = if self.finished {
                None
            } else {
                Some(current.advance_towards(self.range.end))
            };

            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_point_advance_towards() {
        let target = Point::new(5, 5);

        assert_eq!(Point::new(1, 5), Point::new(0, 5).advance_towards(target));
        assert_eq!(Point::new(5, 5), Point::new(4, 5).advance_towards(target));
        assert_eq!(Point::new(5, 5), Point::new(5, 5).advance_towards(target));
    }

    #[test]
    fn test_horizontal_range_iterator() {
        let range = PointRange::new(Point::new(0, 0), Point::new(5, 0));
        let mut iter = range.iter();
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(Some(Point::new(1, 0)), iter.next());
        assert_eq!(Some(Point::new(2, 0)), iter.next());
        assert_eq!(Some(Point::new(3, 0)), iter.next());
        assert_eq!(Some(Point::new(4, 0)), iter.next());
        assert_eq!(Some(Point::new(5, 0)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_vertical_range_iterator() {
        let range = PointRange::new(Point::new(0, 0), Point::new(0, 5));
        let mut iter = range.iter();
        assert_eq!(Some(Point::new(0, 0)), iter.next());
        assert_eq!(Some(Point::new(0, 1)), iter.next());
        assert_eq!(Some(Point::new(0, 2)), iter.next());
        assert_eq!(Some(Point::new(0, 3)), iter.next());
        assert_eq!(Some(Point::new(0, 4)), iter.next());
        assert_eq!(Some(Point::new(0, 5)), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
