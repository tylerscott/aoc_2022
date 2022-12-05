fn main () {
    let input: &str = include_str!("../input/day4.txt").trim();
    let parts = input.split('\n').map(|pair| {
        let ranges: Vec<Vec<i64>> = pair.split(',')
            .map(|elf| {
                let r: Vec<i64> = elf.split('-').map(|end| {
                    return match end.parse::<i64>() {
                        Ok(end) => end,
                        Err(_) => panic!("Error inside end parse"),
                    }
                }).collect();
                if r.len() < 2 { panic!("Range has less than 2 elements");}
                return r
            }).collect();
        let left: &Vec<i64> = &ranges[0];
        let right: &Vec<i64> = &ranges[1];

        if left[0] < right[0] && left[1] < right[0] {
            return (0,0)
        } else if left[0] > right[0] && left[0] > right[1]  {
            return (0,0)
        }

        if left[0] <= right[0] && left[1] >= right[1] {
            return (1,1)
        }

        if right[0] <= left[0] && right[1] >= left[1] {
            return (1,1)
        }
        

        return (0,1)
        //}).sum::<i64>();
    }).fold((0,0), |acc, outcome: (i64, i64)| {
            return (acc.0 + outcome.0, acc.1 + outcome.1)
        });
    println!("Part 1: {:?}", parts.0);
    println!("Part 2: {:?}", parts.1);
}
