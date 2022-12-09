#![feature(iter_array_chunks)]
use std::{fs, str::FromStr};

struct Crane {
    stack: Vec<Vec<char>>
}

impl Crane {
    fn new() -> Self {
        Crane {
            stack: Vec::<Vec<char>>::new()
        }
    }
    
    fn move_single_box(&mut self, m: &Move) {
        //println!("{:?}", m);
        for i in 0..m.count {
            //println!("{:?}, {:?}", m.count - i, self.stack[m.start]);
            let item = self.stack[m.start].pop().expect("Cant remove from empty stack");
            self.stack[m.end].push(item);
        }

    }

    fn move_5001(&mut self, m: &Move) {
        let stack_len = self.stack[m.start].len();
        let items = self.stack[m.start].split_off(stack_len - m.count);
        self.stack[m.end].extend(items);
    }
}

impl FromStr for Crane { 
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_,mut stack, _) = s.chars().rev().array_chunks::<4>().fold((0 as usize, Vec::<Vec<char>>::new(), 0 as usize), |mut acc, block| {
            //println!("{:?}", block);
            //println!("{:?}", block);
            if acc.2 == 0 {
                let cols = block[1].to_digit(10).expect("Not a digit");
                for _ in 0..cols {
                    acc.1.push(Vec::new());
                }
                //println!("Initating Crane with {:?} columns", acc.1.len());
                return (acc.0 + 1, acc.1, block[1].to_digit(10).expect("Not a digit") as usize)
            }

            if block[1].is_alphabetic() {
                //println!("{:?}", block[1]);
                //println!("in stack: {:?}", acc.2 - acc.0);
                acc.1[acc.2  - acc.0 - 1].push(block[1]);
                
            }
            if block[3] == '\n' {
                return (0, acc.1, acc.2)
            }

            (acc.0 + 1, acc.1, acc.2)
        });

        
    
        match s.chars().array_chunks::<4>().next() {
            Some(testing) => {
                stack[0].push(testing[1]);
            },
            None => {
                println!("No Remainder");
            },
        }
        
        return Ok(Crane {stack})
    }
        
}

#[derive(Debug)]
struct Move {
    count: usize,
    start: usize,
    end: usize,
}

impl Move {
    fn new(count: usize, start: usize, end: usize) -> Self {
        Move {count, start, end}
    }
}

impl FromIterator<usize> for Move {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        return Move {
            count: iter.next().expect("count must exist"),
            start: iter.next().expect("start must exist") - 1,
            end: iter.next().expect("end must exist") - 1,
        }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s.split_whitespace().flat_map(|x| x.parse::<usize>())
            .collect::<Move>();
        return Ok(moves)
    }
}

fn main () {
    let input = fs::read_to_string("./src/input/day5.txt").unwrap();
    let input = input.trim();
    let (crane, moves) = input.split_once("\n\n").expect("Input should be standard");
    let mut crane = crane.parse::<Crane>().expect("Error in crane parse");
    //println!("{:?}", crane.stack);
    let moves: Vec<Move> = moves.lines().flat_map(|x| x.parse::<Move>()).collect();

    moves.iter().enumerate().for_each(|(_i, m)| {
        crane.move_5001(m);
    });

    println!("part 1: {:?}", crane.stack.iter().fold(String::from(""), |mut out, stack| {
        let c = stack.last().unwrap();
        out.push(*c);
        return out
    }));
}
