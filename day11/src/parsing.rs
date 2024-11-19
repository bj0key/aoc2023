#[derive(Debug)]
struct Coords {
    x: u64,
    y: u64,
}
impl Coords {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    fn x(&self) -> u64 {
        self.x
    }
    fn y(&self) -> u64 {
        self.y
    }
    fn taxicab(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

type UnexpandedGalaxy = Vec<Vec<bool>>;
/// Just directly translates the given input data into a 2D vector of bools, for later expansion
fn parse_input_naive(input: &str) -> UnexpandedGalaxy {
    input
        .lines()
        .map(|line| line.bytes().map(|c| c == b'#').collect())
        .collect()
}

pub type Galaxy = Vec<Coords>;

fn expand_naive_galaxy(galaxy: UnexpandedGalaxy, expansion_factor: usize) -> Galaxy {
    let empty_rows: Vec<bool> = (0..galaxy.len())
        .map(|i| galaxy[i].iter().all(|b| !b))
        .collect();

    let empty_cols: Vec<bool> = (0..galaxy[0].len())
        .map(|i| galaxy.iter().all(|row| !row[i]))
        .collect();

    let mut star_coords = vec![];
    let mut y = 0;
    for (i, row) in galaxy.into_iter().enumerate() {
        let mut x = 0;
        if empty_rows[i] {
            y += expansion_factor - 1;
            // continue;
        }
        for (j, cell) in row.into_iter().enumerate() {
            if empty_cols[j] {
                x += expansion_factor - 1;
                // continue;
            }
            if cell {
                // println!("Star @ {x}, {y}");
                star_coords.push(Coords::new(x as u64, y as u64));
            }

            x += 1;
        }

        y += 1;
    }

    star_coords
}

pub fn parse_input(input: &str, expansion_factor: usize) -> Galaxy {
    expand_naive_galaxy(parse_input_naive(input), expansion_factor)
}
