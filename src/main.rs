pub fn part_one(input: &str) -> Option<i32> {
    let (mut lhs, mut rhs) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|n| n.parse().ok()).collect();
        (lhs.push(nums[0]), rhs.push(nums[1]));
    }

    (lhs.sort(), rhs.sort());

    let sum = lhs.iter().zip(rhs.iter()).map(|(a, b)| (a - b).abs()).sum();

    Some(sum)
}
fn main() {
    let test_input = "3 4\n4 3\n2 5\n1 3\n3 9\n 3 3";
    let output = part_one(test_input);

    

    println!("output: {:?}", output.unwrap());
}