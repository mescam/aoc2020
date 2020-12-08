use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn parse(contents: &String) -> HashMap<String, HashSet<String>> {
    let mut results: HashMap<String, HashSet<String>> = HashMap::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        let target = [words[0], words[1]].join("");
        println!("{}", target);

        let mut i = 4;
        while i + 4 <= words.len() {
            let from = [words[i+1], words[i+2]].concat();
            println!("-> {}", &from);

            i += 4;

            if !results.contains_key(&from) {
                results.insert(String::from(&from), HashSet::new());
            }

            results.get_mut(&from).unwrap().insert(String::from(&target));
            
        }
    }

    results
}

fn find_bags(key: String, map: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    println!("Running for {}", &key);
    let initial: HashSet<String> = [key.clone()].iter().cloned().collect();
    let result = match map.get(&key) {
        Some(k) => match k.iter().count() {
            0 => initial,
            n => {
                let s1 = map.get(&key).unwrap();
                let more = s1.iter().map(|x| find_bags(x.to_string(), map));
                more.fold(initial, |acc, x| acc.union(&x).map(|x| x.clone()).collect())
            }
        },
        None => initial,
    };
    println!("For key {} returning {:?}", &key, &result);
    result
}

fn parse2(contents: &String) -> HashMap<String, Vec<(String, i32)>> {
    let mut results: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(" ").collect();
        let target = [words[0], words[1]].join("");
        let mut i = 4;
        while i + 4 <= words.len() {
            let count = words[i].parse().unwrap();
            let contains = [words[i+1], words[i+2]].concat();

            i += 4;
            if !results.contains_key(&target) {
                results.insert(String::from(&target), Vec::new());
            }

            results.get_mut(&target).unwrap().push((String::from(contains), count));
        }
    }

    results
} 

fn count_bags(key: String, map: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    match map.get(&key) {
        Some(v) => v.iter().fold(1, |acc, (k, c)| acc + (c * count_bags(k.clone(), map))),
        None => 1
    }
}

fn main() {
    let contents_test = fs::read_to_string("input.txt")
    .expect("error loading file");

    let rtest = parse(&contents_test);
    let mtest = parse2(&contents_test);

    println!("{:?}", rtest);
    println!("result_test = {:?}", find_bags(String::from("shinygold"), &rtest).len() - 1);

    println!("{:?}", mtest);
    println!("result 2 = {}", count_bags(String::from("shinygold"), &mtest) - 1);
}
