use std::{fmt::Display, io::Write, mem};

use fxhash::{FxHashMap, FxHashSet};

#[aoc(day20, part1)]
fn part1(input: &str) -> u64 {
    let tiles = solve(input.split("\n\n").map(Tile::from).collect());
    corners_product(&tiles)
}

#[aoc(day20, part2)]
fn part2(input: &str) -> usize {
    let mut picture = Picture::new(solve(input.split("\n\n").map(Tile::from).collect()));
    picture.hit_test_all(
        "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
",
    );
    picture
        .pixels
        .iter()
        .filter(|p| matches!(p, Pixel::Black { hit_count: 0 }))
        .count()
}

struct Picture {
    pixels: Vec<Pixel>,
    edge_len: usize,
}

impl Picture {
    fn new(initial_tiles: Vec<Tile>) -> Self {
        let mut tiles = FxHashMap::<(i32, i32), Tile>::default();
        for tile in solve(initial_tiles) {
            tiles.insert(tile.coordinates.unwrap(), tile);
        }
        let [x_min, x_max, y_min, y_max] = min_max_coordinates(tiles.values());
        assert_eq!(x_max - x_min, y_max - y_min);
        let tiles_per_edge = (x_max - x_min + 1) as usize;
        let tile_len = tiles.values().next().unwrap().edges[0].len() - 2;
        let edge_len = tiles_per_edge * tile_len;
        let mut picture = Picture {
            pixels: vec![Pixel::White; edge_len * edge_len],
            edge_len,
        };
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let offset = (
                    (x - x_min) as usize * tile_len,
                    (y - y_min) as usize * tile_len,
                );
                let tile = tiles.get(&(x, y)).unwrap();
                for (tile_y, line) in tile.raw.iter().enumerate() {
                    if tile_y == 0 || tile_y == tile_len + 1 {
                        continue;
                    }
                    for (tile_x, char) in line.char_indices() {
                        if tile_x == 0 || tile_x == tile_len + 1 {
                            continue;
                        }
                        if char == '#' {
                            *picture.get_pixel_mut(offset.0 + tile_x - 1, offset.1 + tile_y - 1) =
                                Pixel::Black { hit_count: 0 };
                        }
                    }
                }
            }
        }
        picture
    }

    fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.pixels[y * self.edge_len + x]
    }
    fn get_pixel(&mut self, x: usize, y: usize) -> &Pixel {
        &self.pixels[y * self.edge_len + x]
    }

    fn hit_test(
        &mut self,
        coordinates: &[(usize, usize)],
        image_height: usize,
        image_width: usize,
    ) {
        for y in 0..=(self.edge_len - image_height) {
            for x in 0..=(self.edge_len - image_width) {
                let matches_all = coordinates.iter().all(|(c_x, c_y)| {
                    matches!(self.get_pixel(c_x + x, c_y + y), Pixel::Black { .. })
                });
                if matches_all {
                    for (c_x, c_y) in coordinates.iter() {
                        if let Pixel::Black { hit_count } = self.get_pixel_mut(c_x + x, c_y + y) {
                            *hit_count += 1;
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }

    fn hit_test_all(&mut self, input: &str) {
        /* for (idx, pixel) in self.pixels.iter().enumerate() {
            print!(
                "{}",
                match pixel {
                    Pixel::Black { .. } => "X",
                    _ => " ",
                }
            );
            if idx % self.edge_len == 0 {
                println!()
            }
        }
        println!(); */
        std::io::stdout().flush().unwrap();
        let mut coordinates: Vec<(usize, usize)> = vec![];
        let mut height = 0;
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.char_indices() {
                if char == '#' {
                    coordinates.push((x, y));
                }
                width = x;
            }
            height = y;
        }
        height += 1;
        width += 1;
        for _ in 0..4 {
            self.hit_test(&coordinates, height, width);
            rotate_coordinates(&mut coordinates, &mut height, &mut width);
        }
        flip_coordinates(&mut coordinates, &mut height, &mut width);
        for _ in 0..4 {
            self.hit_test(&coordinates, height, width);
            rotate_coordinates(&mut coordinates, &mut height, &mut width);
        }
    }
}

fn rotate_coordinates(
    coordinates: &mut Vec<(usize, usize)>,
    height: &mut usize,
    width: &mut usize,
) {
    for (x, y) in coordinates {
        (*x, *y) = (*height - *y - 1, *x);
    }
    mem::swap(height, width);
}
fn flip_coordinates(coordinates: &mut Vec<(usize, usize)>, height: &mut usize, width: &mut usize) {
    for (x, y) in coordinates {
        (*x, *y) = (*y, *x);
    }
    mem::swap(height, width);
}

#[derive(Clone)]
enum Pixel {
    White,
    Black { hit_count: u32 },
}

struct Tile {
    id: u64,
    raw: Vec<String>,
    edges: [String; 4],
    coordinates: Option<(i32, i32)>,
}

impl Tile {
    fn rotate_flip(&mut self, rotation: u8, flip: bool) {
        // flips are around a vertical axis
        if flip {
            for line in self.raw.iter_mut() {
                *line = line.chars().rev().collect();
            }
            self.edges.swap(1, 3);
            for edge in self.edges.iter_mut() {
                *edge = edge.chars().rev().collect();
            }
        }
        if rotation != 0 {
            self.edges.rotate_right(rotation as usize);
            if rotation == 2 {
                self.raw = self
                    .raw
                    .iter()
                    .rev()
                    .map(|line| line.chars().rev().collect())
                    .collect();
            } else if rotation == 1 {
                let mut new_raw = vec![String::new(); self.raw.len()];
                for line in self.raw.iter().rev() {
                    for (new_line, ch) in line.chars().enumerate() {
                        new_raw[new_line].push(ch);
                    }
                }
                self.raw = new_raw;
            } else {
                assert_eq!(rotation, 3);
                let mut new_raw = vec![String::new(); self.raw.len()];
                for line in self.raw.iter() {
                    for (new_line, ch) in line.chars().rev().enumerate() {
                        new_raw[new_line].push(ch);
                    }
                }
                self.raw = new_raw;
            }
        }
    }
}

impl From<&str> for Tile {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let header = lines.next().unwrap();
        let id = &header["Tile ".len()..];
        let id = id[..id.len() - 1].parse().unwrap();
        let raw: Vec<String> = lines.map(str::to_string).collect();
        let edges = [
            raw.first().unwrap().to_string(),
            raw.iter()
                .map(|line| line.chars().last().unwrap())
                .collect(),
            raw.last().unwrap().chars().rev().collect(),
            raw.iter()
                .rev()
                .map(|line| line.chars().next().unwrap())
                .collect(),
        ];
        Self {
            id,
            raw,
            edges,
            coordinates: None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.raw {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn solve(mut initial_tiles: Vec<Tile>) -> Vec<Tile> {
    let mut all_edges = FxHashMap::<String, (u64, u8, bool)>::default();
    let mut tiles = FxHashMap::<u64, Tile>::default();
    let mut debug_coordinates = FxHashSet::<(i32, i32)>::default();
    debug_coordinates.insert((0, 0));

    loop {
        let tile = {
            if let Some((tile_idx, rotation_self, &(other, rotation_other, reversed_other))) =
                initial_tiles
                    .iter()
                    .enumerate()
                    .find_map(|(tile_idx, tile)| {
                        tile.edges
                            .iter()
                            .enumerate()
                            .find_map(|(rotation_self, edge)| {
                                all_edges
                                    .get(edge)
                                    .map(|other| (tile_idx, rotation_self, other))
                            })
                    })
            {
                let mut tile = initial_tiles.remove(tile_idx);
                let mut rotation_distance = -(rotation_self as i8) + (rotation_other as i8) + 2;
                let should_flip = !reversed_other;
                if should_flip && rotation_self % 2 == 1 {
                    // flips are around a vertical axis
                    rotation_distance += 2;
                }
                let rotation_distance = rotation_distance.rem_euclid(4) as u8;
                tile.rotate_flip(rotation_distance, should_flip);
                let other_coordinates = tiles.get(&other).unwrap().coordinates.unwrap();
                tile.coordinates = Some(match rotation_other {
                    0 => (other_coordinates.0, other_coordinates.1 - 1),
                    1 => (other_coordinates.0 + 1, other_coordinates.1),
                    2 => (other_coordinates.0, other_coordinates.1 + 1),
                    3 => (other_coordinates.0 - 1, other_coordinates.1),
                    _ => unreachable!(),
                });
                debug_assert!(
                    debug_coordinates.insert(tile.coordinates.unwrap()),
                    "Duplicate coordinates: An entry for {:?} was already present.\nTried to attach to {:?}",
                    tile.coordinates.unwrap(),
                    other_coordinates,
                );
                tile
            } else if !tiles.is_empty() {
                continue;
            } else {
                let mut tile = initial_tiles.remove(initial_tiles.len() - 1);
                tile.coordinates = Some((0, 0));
                tile
            }
        };
        all_edges.extend(
            tile.edges
                .iter()
                .enumerate()
                .map(|(rotation, edge)| (edge.to_owned(), (tile.id, rotation as u8, false))),
        );
        all_edges.extend(tile.edges.iter().enumerate().map(|(rotation, edge)| {
            (
                edge.chars().rev().collect(),
                (tile.id, rotation as u8, true),
            )
        }));
        assert!(tiles.insert(tile.id, tile).is_none());
        if initial_tiles.is_empty() {
            break;
        }
    }
    tiles.into_values().collect()
}

fn min_max_coordinates<'a, T>(tiles: T) -> [i32; 4]
where
    T: Iterator<Item = &'a Tile>,
{
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    for tile in tiles {
        let coordinates = tile.coordinates.unwrap();
        if coordinates.0 < x_min {
            x_min = coordinates.0;
        }
        if coordinates.0 > x_max {
            x_max = coordinates.0;
        }
        if coordinates.1 < y_min {
            y_min = coordinates.1;
        }
        if coordinates.1 > y_max {
            y_max = coordinates.1;
        }
    }
    assert_eq!(x_max - x_min, y_max - y_min);
    [x_min, x_max, y_min, y_max]
}

fn corners_product(tiles: &[Tile]) -> u64 {
    let [x_min, x_max, y_min, y_max] = min_max_coordinates(tiles.iter());
    let corners: Vec<u64> = tiles
        .iter()
        .filter(|tile| {
            [
                tile.coordinates.unwrap().0 == x_min,
                tile.coordinates.unwrap().0 == x_max,
                tile.coordinates.unwrap().1 == y_min,
                tile.coordinates.unwrap().1 == y_max,
            ]
            .iter()
            .filter(|v| **v)
            .count()
                == 2
        })
        .map(|tile| tile.id)
        .collect();
    assert_eq!(corners.len(), 4);
    corners.iter().product()
}

#[test]
fn e1() {
    let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    assert_eq!(part1(input), 20899048083289);
    assert_eq!(part2(input), 273);
}

#[test]
fn real1() {
    let input = std::fs::read_to_string("input/2020/day20.txt").unwrap();
    assert_eq!(part1(input.trim()), 45079100979683);
}
#[test]
fn real2() {
    let input = std::fs::read_to_string("input/2020/day20.txt").unwrap();
    assert_eq!(part2(input.trim()), 1946);
}
