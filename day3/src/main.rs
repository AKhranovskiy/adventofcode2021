use std::ops::{BitAnd, Shr};

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .split('\n')
        .map(|s| usize::from_str_radix(s, 2))
        .collect::<Result<Vec<usize>, _>>()?;

    println!("Power consumption: {}", power_consumption(&input).unwrap());
    println!(
        "Life support rating: {}",
        life_support_rating(&input).unwrap()
    );

    Ok(())
}

fn power_consumption(data: &[usize]) -> Option<usize> {
    let threshold: usize = data.len() / 2;
    let mut mask: usize = 1 << (usize::BITS - 1);
    let mut gamma: usize = 0;

    while mask > 0 {
        let count = data.iter().filter(|&v| v.bitand(mask) == mask).count();
        gamma |= if count >= threshold { mask } else { 0 };
        mask >>= 1;
    }

    let data_mask = (most_significant_bit(data) << 1) - 1;
    let epsilon = (!gamma).bitand(data_mask);
    Some(gamma * epsilon)
}

fn life_support_rating(data: &[usize]) -> Option<usize> {
    OxygenGeneratorRating
        .calculate(data)
        .zip(CO2ScrubberRating.calculate(data))
        .map(|(oxy, co2)| oxy * co2)
}

struct OxygenGeneratorRating;
struct CO2ScrubberRating;

enum Selector {
    Fewer,
    More,
}

enum ImportantBit {
    Zero,
    One,
}

trait LifeSupportRating {
    fn selector(&self) -> Selector;
    fn important_bit(&self) -> ImportantBit;

    fn calculate(&self, data: &[usize]) -> Option<usize> {
        self._calculate_impl(data, most_significant_bit(data))
    }

    fn _calculate_impl(&self, data: &[usize], mask: usize) -> Option<usize> {
        if data.is_empty() {
            return None;
        }

        if data.len() == 1 {
            return data.first().copied();
        }

        if mask == 0 {
            return None;
        }

        let (ones, zeros): (Vec<usize>, Vec<usize>) =
            data.iter().partition(|&v| v.bitand(mask) == mask);

        let data = match ones.len().cmp(&zeros.len()) {
            std::cmp::Ordering::Greater => match self.selector() {
                Selector::Fewer => zeros,
                Selector::More => ones,
            },
            std::cmp::Ordering::Less => match self.selector() {
                Selector::Fewer => ones,
                Selector::More => zeros,
            },
            std::cmp::Ordering::Equal => match self.important_bit() {
                ImportantBit::Zero => zeros,
                ImportantBit::One => ones,
            },
        };

        self._calculate_impl(&data, mask.shr(1))
    }
}

impl LifeSupportRating for OxygenGeneratorRating {
    fn selector(&self) -> Selector {
        Selector::More
    }

    fn important_bit(&self) -> ImportantBit {
        ImportantBit::One
    }
}

impl LifeSupportRating for CO2ScrubberRating {
    fn selector(&self) -> Selector {
        Selector::Fewer
    }

    fn important_bit(&self) -> ImportantBit {
        ImportantBit::Zero
    }
}

fn most_significant_bit(data: &[usize]) -> usize {
    data.iter()
        .map(|&v| {
            usize::BITS
                .checked_sub(v.leading_zeros() + 1)
                .map_or(0, |s| 1 << s)
        })
        .max()
        .unwrap_or(0)
}

#[allow(dead_code)]
const TEST_DATA: &[usize] = &[
    0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
    0b00010, 0b01010,
];

#[test]
fn test_most_significant_bit() {
    assert_eq!(0b0, most_significant_bit(&[0b0]));
    assert_eq!(0b1, most_significant_bit(&[0b1]));
    assert_eq!(0b1000, most_significant_bit(&[0b1111]));
    assert_eq!(1 << 63, most_significant_bit(&[usize::MAX]));
}

#[test]
fn test_calculate_rating() {
    assert_eq!(Some(23), OxygenGeneratorRating.calculate(TEST_DATA));
    assert_eq!(Some(10), CO2ScrubberRating.calculate(TEST_DATA));
}

#[test]
fn test_power_consumption() {
    assert_eq!(Some(198), power_consumption(TEST_DATA));
}

#[test]
fn test_life_support_rating() {
    assert_eq!(Some(230), life_support_rating(TEST_DATA));
}
