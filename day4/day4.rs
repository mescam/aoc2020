use std::fs;
use std::collections::HashMap;

const P1_REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const HEX_VALID: [char; 16] = ['a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const PID_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn is_valid_hex(c: &char) -> bool {
    HEX_VALID.iter().any(|x| x == c)
}

fn validate(input: &HashMap<String, String>) -> bool {
    for field in P1_REQUIRED.iter() {
        if !input.contains_key(*field) {
            return false;
        }
    }
    true
}

fn validate2(input: &HashMap<String, String>) -> bool {
    fn check_byr(byr: &String) -> bool {
        let b = byr.parse::<i32>().unwrap();
        byr.len() == 4 && b >= 1920 && b <= 2002
    }

    fn check_iyr(iyr: &String) -> bool {
        let i = iyr.parse::<i32>().unwrap();
        iyr.len() == 4 && i >= 2010 && i <= 2020
    }

    fn check_eyr(eyr: &String) -> bool {
        let i = eyr.parse::<i32>().unwrap();
        eyr.len() == 4 && i >= 2020 && i <= 2030
    }

    fn check_hgt(hgt: &String) -> bool {
        let len = hgt.len();
        if len < 3 { return false }
        let n = hgt[..(len-2)].parse::<i32>().unwrap();
        match &hgt[(len-2)..] {
            "in" => 59 <= n && n <= 76,
            "cm" => 150 <= n && n <= 193,
            _ => false
        }
    }

    fn check_hcl(hcl: &String) -> bool {
        if hcl.chars().nth(0).unwrap() != '#' { return false; }
        hcl[1..].chars().all(|x| is_valid_hex(&x))
    }

    fn check_ecl(ecl: &String) -> bool {
        EYE_COLOR.iter().any(|x| x == ecl)
    }

    fn check_pid(pid: &String) -> bool {
        pid.len() == 9 && pid.chars().all(|x| PID_CHARS.iter().any(|y| y == &x))
    }

    // validate fields
    for (key, val) in input.iter() {
        let valid = match key.as_str() {
            "byr" => check_byr(val),
            "iyr" => check_iyr(val),
            "eyr" => check_eyr(val),
            "hgt" => check_hgt(val),
            "hcl" => check_hcl(val),
            "ecl" => check_ecl(val),
            "pid" => check_pid(val),
            "cid" => true,
            _ => false
        };
        if !valid {
            println!("Invalidated value {} {}", key, val);
            return false;
        }
    }
    true
}

fn parse(content: String) -> Vec<HashMap<String, String>> {
    let mut vec: Vec<HashMap<String, String>> = Vec::new();
    let it = content
    .split("\n\n");

    for i in it {
        let w = i
        .replace("\n", " ");
        let ss = w.split(" ").filter(|&x| x.len() > 0)
        .collect::<Vec<&str>>(); // I don't know why I had to split it into 2 variables
        
        let mut hm: HashMap<String, String> = HashMap::new();
        for v in ss {
            let h: Vec<&str> = v.split(":").collect();
            hm.insert(String::from(h[0]), String::from(h[1]));
        }
        vec.push(hm);
    }

    vec
}

fn main() {
    let contents_test = fs::read_to_string("input_example.txt")
    .expect("error loading file");
    let pass_test = parse(contents_test);
    let result_test = pass_test.iter().filter(|&x| validate(x)).count();

    println!("test result 1 = {}", result_test);

    // PART 1
    let contents = fs::read_to_string("input.txt")
    .expect("error loading file");
    let pass = parse(contents);
    let result1 = pass.iter().filter(|&x| validate(x)).count();

    println!("result 1 = {}", result1);

    // PART 2
    let result2 = pass.iter().filter(|&x| validate(x) && validate2(x)).count();
    println!("result 2 = {}", result2);
}