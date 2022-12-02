
fn main () {
    let input: &str = include_str!("../input/day1.txt");
    
    let max_calories = calorie_iterator(&input).max().unwrap();
    let mut top_calories = calorie_iterator(&input).collect::<Vec<usize>>();
    top_calories.sort_by(|a,b| b.cmp(a));

    println!("Max Calories: {:?}", max_calories);
    println!("Top Three: {:?}", top_calories.iter().take(3).sum::<usize>());
}

fn calorie_iterator(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split("\n\n")
        .map(|calories: &str| {
        return calories
                .split("\n")
                .flat_map(str::parse::<usize>)
                .sum::<usize>()
        })
}

