const DATA : &str = include_str!("../data.txt");


fn main() {
    let timer = std::time::Instant::now();

    solve();

    let end = timer.elapsed();

    println!("Solved in {}μs", end.as_micros());

}


fn solve() {
    let mut result = 0;

    let to_digit = |ch: char| -> Option<u32> { ch.to_digit(10) };
    for line in DATA.lines() {
        let first = line.chars().find_map(to_digit).unwrap();
        let last  = line.chars().rev().find_map(to_digit).unwrap();

        result += first * 10 + last;
    }

    println!("Result is {result}");
}
