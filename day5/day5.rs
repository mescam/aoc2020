use std::fs;

const ROWS: i32 = 128;
const COLUMNS: i32 = 8;

fn parse_seat_id(code: String) -> i32 {
    let mut lower = 0;
    let mut upper = ROWS;

    for i in code[..7].chars() {
        let m = (lower + upper) / 2;
        if i == 'F' {
            upper = m;
        } else if i == 'B' {
            lower = m;
        } else {
            panic!("Something went wrong");
        }
    }
    let result_row = lower * 8;
    
    lower = 0;
    upper = COLUMNS;
    for i in code[7..].chars() {
        let m = (lower + upper) / 2;
        if i == 'L' {
            upper = m;
        } else if i == 'R' {
            lower = m;
        } else {
            panic!("Something went wrong");
        }
    }

    result_row + lower
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");
    let seat_list: Vec<i32> = contents.lines().map(|x| parse_seat_id(String::from(x))).collect();
    let max = seat_list.iter().max().unwrap();
    let min = seat_list.iter().min().unwrap();

    println!("result1 = {}", max);

    let my = (*min..*max).step_by(1).find(|x| seat_list.iter().all(|y| x != y)).unwrap();
    println!("result2 = {}", my);

}