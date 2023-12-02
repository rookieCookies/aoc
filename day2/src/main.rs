fn solve_part_one() -> u32 {
    let mut sum = 0;
    'l: for line in DATA.lines() {
        let (game_id, data) = line[5..].split_once(": ").unwrap();
        let game_id = game_id.parse::<u32>().unwrap();

        for block in data.split("; ") {
            for cube in block.split(", ") {
                let (num, ty) = cube.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();

                match ty.as_bytes()[0] {
                    b'r' => if num > 12 { continue 'l },
                    b'b' => if num > 14 { continue 'l },
                    b'g' => if num > 13 { continue 'l },
                    _ => unreachable!()
                }
            }
        }

        sum += game_id;

    }

    sum
}


fn solve_part_two() -> u32 {
    let mut sum = 0;
    for line in DATA.lines() {
        let (_, data) = line.split_once(": ").unwrap();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for block in data.split("; ") {
            for cube in block.split(", ") {
                let (num, ty) = cube.split_once(' ').unwrap();
                let num = num.parse::<u32>().unwrap();

                match ty.as_bytes()[0] {
                    b'r' => red = red.max(num),
                    b'b' => blue = blue.max(num),
                    b'g' => green = green.max(num),
                    _ => unreachable!()
                }
            }
        }

        let power = red * blue * green;
        sum += power;
    }

    sum
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


