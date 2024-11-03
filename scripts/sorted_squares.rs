pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![0; n];

    let mut left = 0;
    let mut right = n - 1;
    let mut pos = n - 1;

    (0..n).for_each(|_| {
        if nums[left].abs() > nums[right].abs() {
            result[pos] = nums[left].pow(2);
            left += 1;
        } else {
            result[pos] = nums[right].pow(2);
            right = right.saturating_sub(1);
        }
        if pos > 0 {
            pos -= 1;
        }
    });

    result
}

fn main() {
    let r = sorted_squares(vec![-4, -1, 0, 3, 10]);
    println!("{:?}", r);
}
