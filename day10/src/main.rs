use std::{collections::BTreeSet, vec};

type Coords = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    H,
    V,
    NE,
    SE,
    NW,
    SW,
    Start,
    Ground,
}
impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            '-' => Pipe::H,
            '|' => Pipe::V,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            'S' => Pipe::Start,
            '.' => Pipe::Ground,
            _ => panic!("Invalid pipe char: {c}"),
        }
    }

    fn double(&self) -> [[Pipe; 2]; 2] {
        use Pipe::*;
        match self {
            Pipe::H => [[H, H], [Ground, Ground]],
            Pipe::V => [[V, Ground], [V, Ground]],
            Pipe::NE => [[NE, H], [Ground, Ground]],
            Pipe::SE => [[SE, H], [V, Ground]],
            Pipe::NW => [[NW, Ground], [Ground, Ground]],
            Pipe::SW => [[SW, Ground], [V, Ground]],
            Pipe::Start => unimplemented!("Convert Start pipe to regular pipe before calling."),
            Pipe::Ground => [[Ground; 2]; 2],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pipes {
    rows: Vec<Vec<Pipe>>,
}
impl Pipes {
    fn from_input(input: &str) -> Self {
        let rows: Vec<Vec<Pipe>> = input
            .lines()
            .map(|l| l.chars().map(Pipe::from_char).collect())
            .collect();

        if !rows.is_empty() {
            assert!(rows.iter().all(|r| r.len() == rows[0].len()));
        } else {
            panic!("Pipe data is empty, something went very very wrong...");
        }

        Self { rows }
    }

    fn start_location(&self) -> (usize, usize) {
        let row = self
            .rows
            .iter()
            .position(|r| r.contains(&Pipe::Start))
            .expect("There should be at least one start");
        let col = self.rows[row]
            .iter()
            .position(|p| p == &Pipe::Start)
            .unwrap();
        (row, col)
    }

    fn get(&self, (r, c): Coords) -> Option<Pipe> {
        self.rows.get(r).and_then(|row| row.get(c)).copied()
    }

    fn targets(&self, (r, c): Coords) -> [Coords; 2] {
        let mut pipe = self.rows[r][c];
        let north = || (r - 1, c);
        let south = || (r + 1, c);
        let east = || (r, c + 1);
        let west = || (r, c - 1);
        if pipe == Pipe::Start {
            pipe = self.start_real_value();
        }
        match pipe {
            Pipe::H => [east(), west()],
            Pipe::V => [north(), south()],
            Pipe::NE => [north(), east()],
            Pipe::SE => [south(), east()],
            Pipe::NW => [north(), west()],
            Pipe::SW => [south(), west()],
            _ => panic!("Targetless pipe section: {pipe:?}"),
        }
    }

    fn start_real_value(&self) -> Pipe {
        let (sr, sc) = self.start_location();
        // N S E W
        let neighbour_coords = [(sr - 1, sc), (sr + 1, sc), (sr, sc + 1), (sr, sc - 1)];
        let mut conns = [false; 4]; // north, south, east, west
        for ((r, c), conn) in neighbour_coords.into_iter().zip(conns.iter_mut()) {
            if self.get((r, c)).is_none_or(|p| p == Pipe::Ground) {
                continue;
            }

            if self.targets((r, c)).contains(&(sr, sc)) {
                *conn = true;
            }
        }
        match conns {
            // Remember, N, S, E, W!
            [true, true, false, false] => Pipe::H,
            [false, false, true, true] => Pipe::V,
            [true, false, true, false] => Pipe::NE,
            [true, false, false, true] => Pipe::NW,
            [false, true, true, false] => Pipe::SE,
            [false, true, false, true] => Pipe::SW,
            _ => panic!("Invalid connection config: {conns:?}"),
        }
    }

    fn make_loop_map(&self) -> PipesMask {
        // Make a 2D vec of bools, with the exact same size as
        let mut map = PipesMask::same_size_as(&self);

        let mut last_pos = self.start_location();

        // make the first move
        let mut curr_pos = self.targets(last_pos)[0];
        map.set(curr_pos, Mask::Pipe);
        let mut curr_pipe = self.get(curr_pos).unwrap();
        while curr_pipe != Pipe::Start {
            let targets = self.targets(curr_pos);
            let next_pos = if targets[0] == last_pos {
                targets[1]
            } else {
                targets[0]
            };
            last_pos = curr_pos;
            curr_pos = next_pos;
            curr_pipe = self.get(curr_pos).unwrap();
            map.set(curr_pos, Mask::Pipe);
        }
        map
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }
    fn col_count(&self) -> usize {
        self.rows.get(0).unwrap_or(&Vec::new()).len()
    }

    fn double(&self) -> Self {
        let mut rows = Vec::with_capacity(self.row_count() * 2);
        for r in self.rows.iter() {
            let mut top = Vec::with_capacity(self.col_count() * 2);
            let mut bot = Vec::with_capacity(self.col_count() * 2);

            for mut p in r.iter().copied() {
                if p == Pipe::Start {
                    p = self.start_real_value();
                    let [[_, tr], b] = p.double();
                    top.extend([Pipe::Start, tr]);
                    bot.extend(b);
                } else {
                    let [t, b] = p.double();
                    top.extend(t);
                    bot.extend(b);
                }
            }
            rows.push(top);
            rows.push(bot);
        }

        Self { rows }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mask {
    Inside,
    Outside,
    Pipe,
    Unknown,
}
/// struct for representing a "mask" of what pipes are part of the loop or not.
struct PipesMask {
    rows: Vec<Vec<Mask>>,
}
impl PipesMask {
    fn same_size_as(pipes: &Pipes) -> Self {
        let n_rows = pipes.rows.len();
        let n_cols = pipes.rows[0].len();
        let mut rows = Vec::with_capacity(n_rows);
        for _ in 0..n_rows {
            rows.push(vec![Mask::Unknown; n_cols]);
        }
        Self { rows }
    }

    fn get(&self, (r, c): Coords) -> Option<Mask> {
        self.rows.get(r).and_then(|row| row.get(c)).copied()
    }

    fn get_mut(&mut self, (r, c): Coords) -> Option<&mut Mask> {
        self.rows.get_mut(r).and_then(|row| row.get_mut(c))
    }

    fn set(&mut self, coords: Coords, val: Mask) {
        let Some(b) = self.get_mut(coords) else {
            return;
        };
        *b = val;
    }

    fn loop_len(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter(|b| **b == Mask::Pipe)
            .count()
    }

    fn n_rows(&self) -> usize {
        self.rows.len()
    }

    fn n_cols(&self) -> usize {
        self.rows.get(0).unwrap_or(&Vec::new()).len()
    }

    fn get_neighbours(&self, (r, c): Coords) -> Vec<Coords> {
        let mut v = vec![];
        if r > 0 {
            v.push((r - 1, c));
        }
        if c > 0 {
            v.push((r, c - 1));
        }
        if r <= self.rows.len() - 1 {
            v.push((r + 1, c));
        }
        if c <= self.rows[0].len() - 1 {
            v.push((r, c + 1));
        }
        v
    }

    fn flood_fill_from(&mut self, start: Coords) {
        // Walk around, visiting every square we can, until there are no more seen but unvisited tiles.
        // If any points were along edges, we were on the outside
        // Otherwise, we are in the inside
        let mut unvisited = BTreeSet::new();
        let mut visited = BTreeSet::new();
        unvisited.insert(start);
        while let Some((r, c)) = unvisited.pop_first() {
            for coords in self.get_neighbours((r, c)) {
                if !visited.contains(&coords) {
                    if let Some(Mask::Unknown) = self.get(coords) {
                        unvisited.insert(coords);
                    }
                }
            }
            visited.insert((r, c));
        }

        let can_escape = visited
            .iter()
            .any(|(r, c)| *r == 0 || *r == self.n_rows() - 1 || *c == 0 || *c == self.n_cols() - 1);
        let fill_value = if can_escape {
            Mask::Outside
        } else {
            Mask::Inside
        };
        for coords in visited.into_iter() {
            self.set(coords, fill_value);
        }
    }

    fn next_unknown_pos(&self) -> Option<Coords> {
        let r = self
            .rows
            .iter()
            .position(|row| row.contains(&Mask::Unknown))?;
        let c = self.rows[r].iter().position(|p| p == &Mask::Unknown)?;
        Some((r, c))
    }

    fn flood_fill_all(&mut self) {
        while let Some(coords) = self.next_unknown_pos() {
            self.flood_fill_from(coords);
        }
    }

    fn count_insides_on_evens(&self) -> u64 {
        let mut total = 0;
        let iter = self
            .rows
            .iter()
            .enumerate()
            .flat_map(|(c, row)| row.iter().enumerate().map(move |(r, m)| ((c, r), m)));

        for ((c, r), m) in iter {
            if c % 2 == 0 && r % 2 == 0 {
                if let &Mask::Inside = m {
                    total += 1;
                }
            }
        }
        total
    }

    fn iter(&self) -> impl Iterator<Item = &Mask> {
        self.rows.iter().flatten()
    }
}

fn main() {
    let input = include_str!("../input");
    let pipes = Pipes::from_input(input);
    let map = pipes.make_loop_map();

    let part1 = map.loop_len() / 2;
    println!("Part 1: {part1}");

    let mut doubled_map = pipes.double().make_loop_map();
    doubled_map.flood_fill_all();

    let part2 = doubled_map.count_insides_on_evens();
    println!("Part 2: {part2}");
}
