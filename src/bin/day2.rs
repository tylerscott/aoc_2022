fn main () {
    let input: &str = include_str!("../input/day2.txt").trim();

    let games1 = input.split('\n').map(|game| {
        match game {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => { panic!(); }
        }
    }).sum::<i32>();

    let games2 = input.split('\n').map(|game| {
        match game {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1+ 6,
            _ => { panic!(); }
        }
    }).sum::<i32>();

    println!("Simple Map: {:?}", games1);
    println!("Second Map: {:?}", games2);
}

