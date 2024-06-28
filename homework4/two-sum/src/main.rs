
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h = HashMap::<i32, i32>::new();

    for (i, &num) in nums.iter().enumerate() {
        match h.get(&(target-num)) {
            Some(&index) => return vec![index as i32, i as i32],
            _ => ()
        }
        h.insert(num, i as i32);
    }

    vec![]

}

fn main() {
    println!("{:?}", two_sum(vec![2,3,4,5], 9));
}
