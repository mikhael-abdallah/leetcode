pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut zero_count = 0;
    let length = nums.len();
    for i in 0..length {
        if nums[i] == 0 {
            zero_count += 1;
        } else {
            nums[i - zero_count] = nums[i];
        }
    }
    for i in 0..zero_count {
        nums[length - i - 1] = 0;
    }
}

pub fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    println!("{:?}", nums);
}

