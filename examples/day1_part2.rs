use std::collections::HashMap;



pub fn main() {
    let arg = std::env::args().skip(1).next().unwrap();
    let input = std::fs::read_to_string(arg).unwrap();
    let (mut left, mut right) = input.lines().map(|l| {
        let mut iter = l.split_whitespace().map(|p| p.parse::<i32>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    }).collect::<(Vec<_>, Vec<_>)>();
    left.sort_unstable();
    right.sort_unstable();
    let mut occurences = HashMap::new();
    for v in right{
        *occurences.entry(v).or_insert(0) += 1;
    }
    let similarity = left.into_iter().map(|l| l * occurences.get(&l).unwrap_or(&0)).sum::<i32>();
    println!("Similarity: {similarity}");
}