use std::fs;

fn parse(contents: &String) -> Vec<i64> {
    contents.lines()
    .map(|x| x.parse::<i64>().unwrap())
    .collect()
}

fn is_ok(f: i64, arr: &[i64]) -> bool {
    for i in arr.iter() {
        for j in arr.iter() {
            if i + j == f {
                return true;
            }
        }
    }
    false
}

fn analyze(preamble: usize, numbers: &Vec<i64>) -> i64 {
    for w in numbers.windows(preamble + 1) {
        let my = &w[preamble];
        let rest = &w[..preamble];

        if !is_ok(*my, rest) {
            return *my;
        }
    }
    unreachable!();
}

fn find_cont_sum(numbers: &Vec<i64>, target: i64) -> i64 {
    for window_size in 2..numbers.len() {
        for window in numbers.windows(window_size) {
            if window.iter().sum::<i64>() == target {
                return window.iter().max().unwrap() + window.iter().min().unwrap();
            }
        }
    }
    unreachable!();
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");
    let stream = parse(&contents);

    let result1 = analyze(25, &stream);

    println!("result 1 = {}", &result1);

    let result2 = find_cont_sum(&stream, result1);
    println!("result 2 = {}", &result2);
}