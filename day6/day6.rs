use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(content: &String) -> Vec<HashSet<char>> {
    let mut vec: Vec<HashSet<char>> = Vec::new();
    let groups_it = content
    .split("\n\n");

    for group in groups_it {
        let mut ans: HashSet<char> = HashSet::new();
        let merged = group.replace("\n", "");
        for c in merged.chars() {
            ans.insert(c);
        }
        vec.push(ans);
    }
    vec
}

fn parse2(content: &String) -> Vec<HashSet<char>> {
    let mut vec: Vec<HashSet<char>> = Vec::new();
    let groups_it = content
    .split("\n\n");

    for group in groups_it {
        let mut ans: HashSet<char> = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
        for person in group.lines() {
            ans = ans.intersection(&HashSet::from_iter(person.chars())).copied().collect();
        }
        vec.push(ans);
    }
    vec
}


fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");
    let answers = parse(&contents);
    let answers2 = parse2(&contents);

    // part 1
    let result1 = answers.iter().fold(0, |acc, x| acc + x.len());
    println!("result1 = {}", result1);

    // part 2
    let result2 = answers2.iter().fold(0, |acc, x| acc + x.len());
    println!("result2 = {}", result2);
}