fn main() {
    let input = include_str!("input.txt");

    let mut inputs = input.split("\n\n").map(|s| {
        return s
            .split("\n")
            .flat_map(|x| x.parse::<u32>())
            .sum::<u32>();
    }).collect::<Vec<u32>>();

    inputs.sort();
    let sum = inputs.get((inputs.len() - 3)..inputs.len()).unwrap().iter().sum::<u32>();
    println!("{:?}", sum)
}

