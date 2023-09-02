impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let all_nums = get_sorted_list_of_all_nums(nums1, nums2);

        return get_median(&all_nums);
    }
}

fn get_sorted_list_of_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut all_nums: Vec<i32> = vec![nums1, nums2].concat();
    all_nums.sort();

    return all_nums;
}

fn get_median(nums: &Vec<i32>) -> f64 {
    if is_even(nums.len()) {
        return get_even_median(nums);
    } else {
        return get_odd_median(nums);
    }
}

fn is_even(n: usize) -> bool {
    n % 2 == 0
}

fn get_even_median(nums: &[i32]) -> f64 {
    let idx_high = nums.len() / 2;
    let idx_low = idx_high - 1;

    let low = f64::from(nums[idx_high]);
    let high = f64::from(nums[idx_low]);
    
    return (low + high) / 2.0;
}

fn get_odd_median(nums: &[i32]) -> f64 {
    let idx = nums.len() / 2;

    return f64::from(nums[idx]);
}

pub(crate) struct Solution;