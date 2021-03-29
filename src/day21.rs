use fxhash::{FxHashMap, FxHashSet};

#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    let menu = parse(input);
    let ingredients_to_allergens = solve(&menu);
    let mut count = 0;
    for (ingredients, _allergens) in menu {
        for ingredient in ingredients {
            if !ingredients_to_allergens.contains_key(ingredient) {
                count += 1;
            }
        }
    }
    count
}
#[aoc(day21, part2)]
fn part2(input: &str) -> String {
    let menu = parse(input);
    let ingredients_to_allergens = solve(&menu);
    let mut ingredients: Vec<_> = ingredients_to_allergens.into_iter().collect();
    ingredients.sort_by_key(|(_ingredient, allergen)| *allergen);
    ingredients
        .into_iter()
        .map(|(ingredient, _allergen)| ingredient)
        .intersperse(",")
        .collect()
}

struct Puzzle<'a> {
    allergens_to_ingredients: FxHashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Puzzle<'a> {
    fn new(allergens_to_ingredients: FxHashMap<&'a str, Vec<&'a str>>) -> Self {
        Self {
            allergens_to_ingredients,
        }
    }

    fn solve(&mut self) {
        let mut solved_allergens = vec![];
        // Check for allergens that have only one possible ingredient
        for (&allergen, ingredients) in &self.allergens_to_ingredients {
            assert!(!ingredients.is_empty());
            if ingredients.len() == 1 {
                solved_allergens.push((allergen, ingredients[0]));
            }
        }
        for (allergen, ingredient) in solved_allergens {
            self.solved_allergen(allergen, ingredient);
        }
    }

    // We have found that this allergen has only one possibility left.
    fn solved_allergen(&mut self, solved_allergen: &str, source: &str) {
        let mut solved_allergens = vec![];
        for (&allergen, ingredients) in &mut self.allergens_to_ingredients {
            if allergen != solved_allergen {
                if let Some(idx) = ingredients.iter().position(|&i| i == source) {
                    ingredients.swap_remove(idx);
                    if ingredients.len() == 1 {
                        solved_allergens.push((allergen, ingredients[0]));
                    }
                }
            }
        }
        for (solved_allergen, source) in solved_allergens {
            self.solved_allergen(solved_allergen, source);
        }
    }
}

fn solve<'a>(input: &[(Vec<&'a str>, Vec<&'a str>)]) -> FxHashMap<&'a str, &'a str> {
    let mut allergens_to_ingredients = FxHashMap::<&str, Vec<&str>>::default();

    let mut line_ingredients = FxHashSet::default();
    for line in input {
        line_ingredients.clear();
        for i in &line.0 {
            line_ingredients.insert(i);
        }
        for allergen in &line.1 {
            if let Some(ingredients) = allergens_to_ingredients.get_mut(allergen) {
                ingredients.retain(|ingredient| line_ingredients.contains(ingredient));
            } else {
                allergens_to_ingredients.insert(allergen, line.0.clone());
            }
        }
    }

    let mut puzzle = Puzzle::new(allergens_to_ingredients);
    puzzle.solve();

    let mut result = FxHashMap::default();
    for allergen in puzzle.allergens_to_ingredients {
        assert_eq!(allergen.1.len(), 1);
        result.insert(allergen.1[0], allergen.0);
    }
    result
}

fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input.lines().map(parse_line).collect()
}

#[inline]
fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let mut ingredients = Vec::with_capacity(100);
    let mut allergens = Vec::with_capacity(5);
    let mut tokens = line.split(' ');
    while let Some(token) = tokens.next() {
        if token.starts_with('(') {
            break;
        }
        ingredients.push(token);
    }
    for token in tokens {
        let token = token.trim_end_matches(&[')', ','][..]);
        allergens.push(token);
    }
    (ingredients, allergens)
}

#[test]
fn e1() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), "mxmxvkd,sqjhc,fvjkl");
}
#[test]
fn real() {
    let input = std::fs::read_to_string("input/2020/day21.txt").unwrap();

    assert_eq!(part1(&input), 2786);
    assert_eq!(
        part2(&input),
        "prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq"
    );
}
