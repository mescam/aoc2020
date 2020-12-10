use std::fs;


fn parse(contents: &String) -> Vec<i64> {
    contents.lines()
    .map(|x| x.parse::<i64>().unwrap())
    .collect()
}

fn tribonacci(n: i64) -> i64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        _ => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3)
    }
}

fn diffs(vec: &Vec<i64>, size: usize) -> Vec<i64> {
    vec.windows(size).map(|a| (a[size - 1] - a[0])).collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");

    let mut adapters = parse(&contents);
    
    adapters.push(0); // outlet jolt
    adapters.push(*adapters.iter().max().unwrap() + 3); // device jolts
    adapters.sort();

    let mut ones = 0;
    let mut threes = 0;
    for window in adapters.windows(2) {
        let diff = window[1] - window[0];
        if diff == 1 {
            ones += 1;
        } else if diff == 3 {
            threes += 1;
        }
    }

    println!("result1 = {}", ones * threes);

    let d2: i64 = diffs(&adapters, 2)
    .split(|x| *x == 3)
    .map(|s| s.len() as i64)
    .filter(|x| *x > 0)
    .map(|x| tribonacci(x))
    .product();


    println!("result2 = {:?}", d2);
}