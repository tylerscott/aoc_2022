
#![feature(iter_array_chunks)]

use std::collections::HashMap;
use std::collections::HashSet;

fn main () {
    let input: &str = include_str!("../input/day3.txt").trim();

    let ruck: Vec<Option<char>> = input.split('\n').map(|ruc| {
        let (left, right) = ruc.split_at(ruc.len() / 2);
        let mut dup: Option<char> = None;
        for c in left.chars() {
            if right.contains(c) {
                dup = Some(c);
                break;
            }
        }
        return dup
    }).collect();
    
    let score: Vec<usize> = ruck.iter().map(|pack_item| {
        let item: &char =  match pack_item {
            Some(pi) => pi,
            None => {
                println!("No Pack Item from Ruck");
                panic!();
            }
        };
        return char_into_int(item);
        
    }).collect();
    // println!("{:?}", ruck);
    // println!("{:?}", score);
    println!("Sum of Misplaced Items: {:?}", score.iter().sum::<usize>());

    let badges: usize = input.split('\n').array_chunks::<3>().map(|party| {
        // println!("{:?}", party);
        let badge  = party.iter()
            .flat_map(|member| member.chars().collect::<HashSet<_>>().into_iter())
            .fold(HashMap::new(), |mut map: HashMap<char, usize>, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
            .into_iter()
            .filter(|(_, v)| *v == 3)
            .last();
        let badge = match badge {
            Some(badge) => badge.0,
            None => panic!(),
        };
        //println!("{:?}", badge);
        return char_into_int(&badge)
    }).sum::<usize>();

    println!("Sum of Badges: {:?}", badges);
}

static KEY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn char_into_int(c: &char) -> usize {
    let pos: usize = match KEY.find(*c) {
        Some(n) => n,
        None => {
            println!("Item not found in Key");  
            panic!();
        },
    };
    return pos + 1
}
