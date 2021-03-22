use fxhash::FxHashSet;

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
    let mut space: Space<3> = Space::from(input);
    for _ in 0..6 {
        space = space.step();
    }
    space.cubes.len()
}
#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
    let mut space: Space<4> = Space::from(input);
    for _ in 0..6 {
        space = space.step();
    }
    space.cubes.len()
}

type Coordinates<const N: usize> = [i64; N];

const fn number_of_neighbors(dimensions: usize) -> usize {
    3usize.pow(dimensions as u32) - 1
}
/*
// This is slower
struct Neighbors< const N: usize> {
    coordinates: Coordinates<N>,
    counter: usize,
}

impl< const N: usize> Neighbors< N> {
    fn new(coordinates: Coordinates<N>) -> Self {
        Self {
            coordinates,
            counter: 0,
        }
    }
}

impl< const N: usize> Iterator for Neighbors< N> {
    type Item = Coordinates<N>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == number_of_neighbors(N) {
            return None;
        }
        let i = if self.counter >= number_of_neighbors(N) / 2 {
            self.counter + 1
        } else {
            self.counter
        };
        self.counter += 1;
        let mut coordinates = self.coordinates;
        for dimension in 0..N {
            match (i / 3usize.pow(dimension as u32)) % 3 {
                0 => coordinates[dimension] -= 1,
                1 => {}
                2 => coordinates[dimension] += 1,
                _ => unreachable!(),
            }
        }
        Some(coordinates)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (number_of_neighbors(N), Some(number_of_neighbors(N)))
    }
}
*/
fn neighbors<const N: usize>(
    input_coordinates: &Coordinates<N>,
) -> [Coordinates<N>; number_of_neighbors(N)] {
    let mut neighbors = [*input_coordinates; number_of_neighbors(N)];
    for (i, coordinates) in neighbors.iter_mut().enumerate() {
        let i = if i >= number_of_neighbors(N) / 2 {
            i + 1
        } else {
            i
        };
        for (dimension, coordinate) in coordinates.iter_mut().enumerate() {
            match (i / 3usize.pow(dimension as u32)) % 3 {
                0 => *coordinate -= 1,
                1 => {}
                2 => *coordinate += 1,
                _ => unreachable!(),
            }
        }
    }
    neighbors
}

#[derive(Clone)]
struct Space<const N: usize>
where
    [(); number_of_neighbors(N)]: ,
{
    cubes: FxHashSet<Coordinates<N>>,
}

impl<const N: usize> Space<N>
where
    [(); number_of_neighbors(N)]: ,
{
    #[inline]
    fn should_be_active(&self, coordinates: &Coordinates<N>) -> bool {
        let neighbors = neighbors(coordinates);
        let mut active_neighbors = neighbors.iter().filter(|&c| self.cubes.contains(c));
        if self.cubes.contains(coordinates) {
            // cube is currently active and should remain active if 2 or 3 neighbors are active
            active_neighbors.nth(1).is_some() && active_neighbors.nth(1).is_none()
        } else {
            // cube is currently inactive and should become active if 3 neighbors are active
            active_neighbors.nth(2).is_some() && active_neighbors.next().is_none()
        }
    }

    fn step(&self) -> Self {
        let mut to_visit: FxHashSet<Coordinates<N>> = FxHashSet::default();

        for &coordinates in self.cubes.iter() {
            to_visit.insert(coordinates);
            to_visit.extend(&neighbors(&coordinates));
        }
        to_visit.retain(|coordinates| self.should_be_active(coordinates));

        Self { cubes: to_visit }
    }
}

impl<const N: usize> From<&str> for Space<N>
where
    [(); number_of_neighbors(N)]: ,
{
    fn from(input: &str) -> Self {
        let mut cubes = FxHashSet::default();
        for (x, line) in input.lines().enumerate() {
            for (y, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        let mut coordinates = [0; N];
                        coordinates[0] = x as i64;
                        coordinates[1] = y as i64;
                        cubes.insert(coordinates);
                    }
                    '.' => {}
                    _ => unreachable!(),
                };
            }
        }
        Self { cubes }
    }
}

#[test]
fn find_neighbors() {
    let neighbors = neighbors(&[0]);
    assert_eq!(neighbors, [[-1], [1]]);
}
#[test]
fn e1() {
    let input = ".#.
..#
###";
    assert_eq!(part1(input), 112);
    assert_eq!(part2(input), 848);
}
