

pub fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let input = std::fs::read_to_string(arg).unwrap();
    let (mut left, mut right) = input.lines().map(|l| {
        let mut iter = l.split_whitespace().map(|p| p.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    }).collect::<(Vec<_>, Vec<_>)>();
    left.sort_unstable();
    right.sort_unstable();
    let sum_of_differences = left.into_iter().zip(right.into_iter()).map(|(l,r)| (l - r).abs()).sum::<i32>();
    println!("Sum of differences: {sum_of_differences}");
}