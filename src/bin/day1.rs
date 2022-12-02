
fn main () {
    let input: &str = include_str!("../input/day1.txt");

    let line_parse: Option<usize> = input.split("\n\n").map(|line| line.split("\n")
        .flat_map(|num: &str| num.parse::<usize>()).sum::<usize>()
    ).max();

    let mut second: Vec<usize> = input.split("\n\n").map(|line| line.split("\n")
        .flat_map(|num: &str| num.parse::<usize>()).sum::<usize>()
    ).collect::<Vec<usize>>();
    second.sort_by(|a,b| b.cmp(a));

    println!("first answer: {:?}", line_parse.unwrap());
    println!("Second: {:?}", second.iter().take(3).sum::<usize>());
}
