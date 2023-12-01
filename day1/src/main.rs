
fn solve_part_one() -> u32 {
    let mut result = 0;

    let to_digit = |str: &str, iter: &mut dyn Iterator<Item=usize>| -> u32 {
        let buff = str.as_bytes();
        loop {
            let i = iter.next().unwrap();
            if let Some(val) = (buff[i] as char).to_digit(10) {
                return val
            }
        }
    };

    for line in DATA.lines() {
        let first = to_digit(line, &mut (0..line.len()));
        let last  = to_digit(line, &mut (0..line.len()).rev());

        result += first * 10 + last;
    }

    result
}


fn solve_part_two() -> u32 {
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

    result
}


//
// Running
//
const DATA : &str = include_str!("../data.txt");

fn main() {
    // Wind up time
    for _ in 0..5 {
        solve_part_one();
        solve_part_two();
    }


    {
        let timer = std::time::Instant::now();
        let result = solve_part_one();
        let end = timer.elapsed();

        println!("Result is {result}");
        println!("Part 1 solved in {}μs", end.as_micros());
    }

    
    {
        let timer = std::time::Instant::now();

        let result = solve_part_two();

        let end = timer.elapsed();

        println!("Result is {result}");
        println!("Part 2 solved in {}μs", end.as_micros());
    }
    
}


