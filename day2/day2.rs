use std::fs;

struct Entry {
    lo: i32,
    hi: i32,
    lttr: char,
    pwd: String
}

fn parse_to_entry(input: &str) -> Entry {
    let parsed: Vec<&str> = input.split(" ").collect();
    let policy: Vec<&str> = parsed[0].split("-").collect();

    return Entry{
        lo: policy[0].parse::<i32>().ok().unwrap(),
        hi: policy[1].parse::<i32>().ok().unwrap(),
        lttr: parsed[1].chars().nth(0).unwrap(),
        pwd: String::from(parsed[2])
    };
}

fn validate(input: &Entry) -> bool {
    let collection: Vec<char> =  input.pwd.as_str().chars().collect();
    let ns = collection.iter().fold(0, |acc, &x| acc + ((x == input.lttr) as i32));
    if input.lo <= ns && ns <= input.hi {
        return true;
    }
    return false;
}

fn validate2(input: &Entry) -> bool {
    let lo = input.pwd.chars().nth((input.lo-1) as usize).unwrap();
    let hi = input.pwd.chars().nth((input.hi-1) as usize).unwrap();
    if (lo == input.lttr) ^ (hi == input.lttr) {
        return true;
    }
    return false;
}


fn main() {
    let l: &Entry = &parse_to_entry("2-9 c: ccccccccc");
    let l1: &Entry = &parse_to_entry("2-9 d: ccccccccc");
    let l3: &Entry = &parse_to_entry("1-3 a: abcde");
    let l4: &Entry = &parse_to_entry("1-3 b: cdefg");
    assert_eq!(l.lo, 2);
    assert_eq!(l.hi, 9);
    assert_eq!(l.lttr, 'c');
    assert_eq!(l.pwd, String::from("ccccccccc"));
    assert_eq!(validate(l), true);
    assert_eq!(validate(l1), false);
    assert_eq!(validate2(l3), true);
    assert_eq!(validate2(l4), false);
    assert_eq!(validate2(l), false);

    let result = fs::read_to_string("input.txt")
    .expect("error loading file")
    .lines()
    .filter(|x| validate(&parse_to_entry(x)))
    .count();

    let result2 = fs::read_to_string("input.txt")
    .expect("error loading file")
    .lines()
    .filter(|x| validate2(&parse_to_entry(x)))
    .count();

    println!("{} - {}", result, result2);
}
