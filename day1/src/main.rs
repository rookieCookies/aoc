use std::ops::Range;

const DATA : &str = include_str!("../data.txt");


fn main() {
    let timer = std::time::Instant::now();

    solve_part_one();

    let end = timer.elapsed();

    println!("Solved in {}μs", end.as_micros());

}


fn solve_part_one() {
    let mut result = 0;

    let to_digit = |ch: char| -> Option<u32> { ch.to_digit(10) };
    for line in DATA.lines() {
        let first = line.chars().find_map(to_digit).unwrap();
        let last  = line.chars().rev().find_map(to_digit).unwrap();

        result += first * 10 + last;
    }

    println!("Result is {result}");
}


fn solve_part_two() {

    let check_buff = |buff: &[u8], str: &str| -> bool {
        buff.len() >= str.len() && &buff[0..str.len()] == str.as_bytes()
    };

    let to_digit = |str: &str, iter: &mut dyn Iterator<Item=usize>| -> u32 {
        let buff = str.as_bytes();
        loop {
            let i = iter.next().unwrap();
            let buff = &buff[i..];

            if let Some(val) = (buff[0] as char).to_digit(10) {
                return val;
            }


            if check_buff(buff, "one"  ) { return 1 };
            if check_buff(buff, "two"  ) { return 2 };
            if check_buff(buff, "three") { return 3 };
            if check_buff(buff, "four" ) { return 4 };
            if check_buff(buff, "five" ) { return 5 };
            if check_buff(buff, "six"  ) { return 6 };
            if check_buff(buff, "seven") { return 7 };
            if check_buff(buff, "eight") { return 8 };
            if check_buff(buff, "nine" ) { return 9 };
        }
        
    };

    let mut result = 0;

    for line in DATA.lines() {
        let first = to_digit(line, &mut (0..line.len()));
        let last = to_digit(line, &mut (0..line.len()).rev());

        result += first * 10 + last;
    }

    println!("Result is {result}");
}
