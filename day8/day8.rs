use std::fs;

fn parse(contents: &String) -> Vec<(&str, i32)> {
    contents.lines()
    .map(|s| { let w: Vec<&str> = s.split(" ").collect(); (w[0], w[1].parse::<i32>().unwrap()) })
    .collect()
}

fn reverse_op(op: &str) -> &str {
    match op {
        "jmp" => "nop",
        "nop" => "jmp",
        n => n,
    }
}
fn patch(inst: Vec<(&str, i32)>, idx: usize) -> Vec<(&str, i32)> {
    inst.iter().enumerate().map(|(i, (op, value))| { if i == idx { (reverse_op(op), *value) } else { (*op, *value)} }).collect()
}

fn compute(idx: i32, acc: i32, inst: Vec<(&str, i32)>, mut history: Vec<i32>, p: bool) -> Option<i32> {
    match history.contains(&idx) {
        true => {
            if p {
                history.iter().map(|i| compute(0, 0, patch(inst.clone(), *i as usize), Vec::new(), false)).find(|x| *x != None).unwrap()
            } else {
                None
            }
        },
        false => {
            if idx as usize >= inst.len() { return Some(acc); }
            let (code, val) = inst[idx as usize];
            history.push(idx);
            match code {
                "nop" => compute(idx + 1, acc, inst, history, p),
                "acc" => compute(idx + 1, acc + val, inst, history, p),
                "jmp" => compute(idx + val, acc, inst, history, p),
                _ => panic!("operation not implemented")
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");

    let instructions = parse(&contents);

    let result1 = compute(0, 0, instructions, Vec::new(), true);

    println!("result1 = {:?}", result1);
}