use std::collections::HashMap;

const REQUIRED_FIELDS: &[&str] = &[
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /*"cid"*/
];

const VALID_E_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pass| parse_pass(pass))
        .filter(|pass| REQUIRED_FIELDS.iter().all(|f| pass.contains_key(f)))
        .count()
}
#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pass| parse_pass(pass))
        .filter(|pass| REQUIRED_FIELDS.iter().all(|f| pass.contains_key(f)) && validate(pass))
        .count()
}

fn parse_pass(pass: &str) -> HashMap<&str, &str> {
    pass.split_whitespace()
        .map(|p| {
            let mut s = p.split(':');
            (s.next().unwrap(), s.next().unwrap())
        })
        .collect()
}

fn validate(pass: &HashMap<&str, &str>) -> bool {
    pass["byr"]
        .parse::<u32>()
        .map_or(false, |p| (1920..=2002).contains(&p))
        && pass["eyr"]
            .parse::<u32>()
            .map_or(false, |p| (2020..=2030).contains(&p))
        && pass["iyr"]
            .parse::<u32>()
            .map_or(false, |p| (2010..=2020).contains(&p))
        && valid_height(pass["hgt"])
        && valid_color(pass["hcl"])
        && VALID_E_COLORS.contains(&pass["ecl"])
        && valid_pid(pass["pid"])
}

fn valid_height(h: &str) -> bool {
    let n = &h[..h.len() - 2].parse();
    let n = if let Ok(n) = n {
        n
    } else {
        return false;
    };
    match &h[h.len() - 2..] {
        "cm" => (150..=193).contains(n),
        "in" => (59..=76).contains(n),
        _ => false,
    }
}

fn valid_color(color: &str) -> bool {
    let mut chars = color.chars();
    if chars.next().unwrap() != '#' {
        return false;
    }
    let mut count = 0;
    for char in chars {
        match char {
            '0'..='9' | 'a'..='f' => count += 1,
            _ => return false,
        }
    }
    count == 6
}
fn valid_pid(pid: &str) -> bool {
    let chars = pid.chars();
    let mut count = 0;
    for char in chars {
        match char {
            '0'..='9' => count += 1,
            _ => return false,
        }
    }
    count == 9
}

#[test]
fn invalid() {
    let invalid = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    assert!(invalid
        .split("\n\n")
        .map(|pass| parse_pass(pass))
        .all(|pass| !validate(&pass)))
}
#[test]
fn valid() {
    let invalid = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    assert!(invalid
        .split("\n\n")
        .map(|pass| parse_pass(pass))
        .all(|pass| validate(&pass)))
}
