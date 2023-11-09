impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let sub_arrays = get_subarrays(nums, k);
        let almost_unique_sub_arrays = sub_arrays.iter().filter(|v| is_almost_unique(*v, m));

        return almost_unique_sub_arrays.fold(0, |max, v| std::cmp::max(max, get_sum(v)))
    }
}

fn get_subarrays(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    return nums.windows(k as usize).map(|w| return w.to_vec()).collect();
}

fn is_almost_unique(nums: &Vec<i32>, m: i32) -> bool {
    let num_unique = nums.iter().filter(|n| is_unique(nums, **n)).count();

    return (num_unique as i32) >= m;
}

fn is_unique(nums: &Vec<i32>, n: i32) -> bool {
    let num_occurances = nums.iter().filter(|m| **m == n).count();

    return is_odd(num_occurances);
}

fn is_odd(n: usize) -> bool {
    return n % 2 == 1;
}

fn get_sum(nums: &Vec<i32>) -> i64 {
    return nums.iter().fold(0, |max, f| max + (*f) as i64);
}

pub(crate) struct Solution;