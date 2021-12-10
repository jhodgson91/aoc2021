use std::{ops::RangeInclusive, str::FromStr, collections::HashSet};

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
struct Coord {
    x: u16,
    y: u16,
}

#[derive(Debug, Default)]
struct Line(Coord, Coord);

impl Line {
    const COORD_SEPERATOR: &'static str = " -> ";
    const VALUE_SEPARATOR: char = ',';

    fn is_diagonal(&self) -> bool {
        self.0.x != self.1.x && self.0.y != self.1.y
    }

    fn covers_point(&self, point: (u16, u16)) -> bool {
        let on_x = self.0.x <= point.0 && point.0 <= self.1.x;
        let on_y = self.0.y <= point.1 && point.1 <= self.1.y;

        on_x && on_y
    }

    fn points(&self) -> Box<dyn Iterator<Item = Coord>> {
        if self.0.x == self.1.x {
            let x = self.0.x;
            Box::new((self.0.y..=self.1.y).map(move |v| Coord { x, y: v }))
        } else if self.0.y == self.1.y {
            let y = self.0.y;
            Box::new((self.0.x..=self.1.x).map(move |v| Coord { x: v, y }))
        } else {
            Box::new((self.0.x..=self.1.x).map(move |v| Coord { x: v, y: v }))
        }
    }

    fn intersect(&self, other: &Line) -> impl Iterator<Item = Coord> {
        self.points().collect::<HashSet<Coord>>();
        std::iter::once(Coord::default())
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::default();

        let coords: Vec<&str> = s.split(Self::COORD_SEPERATOR).collect();
        let first: Vec<u16> = coords[0]
            .split(Self::VALUE_SEPARATOR)
            .map(|s| s.parse::<u16>().expect("invalid value"))
            .collect();
        let second: Vec<u16> = coords[1]
            .split(Self::VALUE_SEPARATOR)
            .map(|s| s.parse::<u16>().expect("invalid value"))
            .collect();

        result.0.x = first[0];
        result.1.x = second[0];

        result.0.y = first[1];
        result.1.y = second[1];

        Ok(result)
    }
}

fn main() {
    let input: Vec<Line> = std::fs::read_to_string("src/5/input.txt")
        .expect("failed to read input")
        .lines()
        .map(|s| s.parse::<Line>().expect("invalid line"))
        .filter(|l| !l.is_diagonal())
        .collect();

    println!("{:?}", input.len());
}

mod tests {
    use super::*;

    #[test]
    fn cover_point() {
        let line = Line(Coord { x: 0, y: 0 }, Coord { x: 0, y: 10 });

        for p in 0..=10 {
            assert!(line.covers_point((0, p)));
        }

        let line = Line(Coord { x: 0, y: 0 }, Coord { x: 10, y: 0 });
        for p in 0..=10 {
            assert!(line.covers_point((p, 0)));
        }

        assert!(!line.covers_point((1, 1)));
    }
}
