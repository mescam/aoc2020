use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("error loading file");
    let numbers = contents
    .lines()
    .filter_map(|s| s
        .parse::<i32>()
        .ok()
    ).collect::<Vec<_>>();

    for (i, x) in numbers.iter().enumerate() {
        let iter2 = numbers.iter().skip(i+1);
        for (j, y) in iter2.enumerate() {
            let iter3 = numbers.iter().skip(i+j+2);
            for z in iter3 {
                if x + y + z == 2020 {
                    println!("{}*{}*{}={}", x, y, z, x * y * z);
                    return;
                }
            }
            
        }
    }

}