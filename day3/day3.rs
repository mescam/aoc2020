use std::fs;

fn parse(input: String) -> Vec<Vec<char>> {
    input
    .lines()
    .map(|x| x.chars().collect())
    .collect()
}

fn pos(y: usize, x: usize, input: &Vec<Vec<char>>) -> Option<char> {
    // we start with x = 0. y = 0
    // with x we go right infintely
    // with y we go to the bottom and then return an error
    let y_size = input.len();
    assert!(y_size > 0);
    let x_size = input[0].len();

    if y >= y_size { 
        None
    } else {
        Some(input[y][x % x_size])
    }
}

fn slide(y: usize, x: usize, step_y: usize, step_x: usize, input: &Vec<Vec<char>>, acc: i32) -> i32 {
    match pos(y, x, input) {
        Some('.') => slide(y + step_y, x + step_x, step_y, step_x, input, acc),
        Some('#') => slide(y + step_y, x + step_x, step_y, step_x, input, acc + 1),
        Some(_) => panic!("this should not happen"),
        None => acc
    }
}

fn main() {
    let contents_example = fs::read_to_string("input_example.txt")
    .expect("error loading test file");

    let test_vec = parse(contents_example);

    assert_eq!(pos(0, 0, &test_vec), Some('.'));
    assert_eq!(pos(1, 3, &test_vec), Some('.'));
    assert_eq!(pos(0, 13, &test_vec), Some('#'));
    assert_eq!(pos(11, 0, &test_vec), None);

    let test_result = slide(0, 0, 1, 3, &test_vec, 0);
    println!("test result = {}", test_result);

    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");

    let vec = parse(contents);
    let result1 = slide(0, 0, 1, 3, &vec, 0);
    println!("ex 1 result = {}", result1);

    let slopes = [(1,1), (1,3), (1,5), (1,7), (2,1)];
    let result2 = slopes.iter().fold(1, |acc, &x| acc * slide(0, 0, x.0, x.1, &vec, 0));
    println!("ex 2 result = {}", result2);
}