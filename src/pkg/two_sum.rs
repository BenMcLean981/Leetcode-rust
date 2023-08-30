pub (crate) struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort();

        let ans = Solution::two_sum_sorted(sorted.clone(), target);

        let lower_num = sorted[ans[0] as usize];
        let upper_num = sorted[ans[1] as usize];

        return vec![
            Solution::find_original_lower_index(&nums, lower_num),
            Solution::find_original_upper_index(&nums, upper_num),
        ]
    }

    fn find_original_lower_index(nums: &Vec<i32>, lower: i32) -> i32 {
        return nums.iter().position(|&x| x == lower).unwrap() as i32;
    }

    fn find_original_upper_index(nums: &Vec<i32>, upper: i32) -> i32 {
        let from_back = nums.iter().rev().position(|&x| x == upper).unwrap();
        
        return (nums.len() - from_back - 1) as i32;
    }

    fn two_sum_sorted(sorted: Vec<i32>, target: i32) -> Vec<i32> {
        let mut solver = TwoSumSolver::new(sorted, target);

        loop {
            if solver.is_correct() {
                break;
            }

            Solution::try_increase(&mut solver);
            Solution::try_decrease(&mut solver);
        }

        return solver.get_pair();
    }

    fn try_increase(solver: &mut TwoSumSolver) {
        loop {
            if !solver.is_low() {
                break;
            }

            solver.increase();
        }
    }

    fn try_decrease(solver: &mut TwoSumSolver) {
        loop {
            if !solver.is_high() {
                break;
            }

            solver.decrease();
        }
    }
}

struct TwoSumSolver {
    lower: i32,
    upper: i32,
    nums: Box<Vec<i32>>,
    target: i32,
}

impl TwoSumSolver {
    fn new(nums: Vec<i32>, target: i32) -> TwoSumSolver {
        return TwoSumSolver {
            lower: 0,
            upper: (nums.len() - 1) as i32,
            nums: Box::new(nums),
            target: target
        }
    }

    pub fn get_pair(&self) -> Vec<i32> {
        return vec![self.lower, self.upper];
    }

    pub fn is_correct(&self) -> bool {
        return self.get_value() == self.target;
    }

    pub fn is_high(&self) -> bool {
        return self.get_value() > self.target;
    }

    pub fn is_low(&self) -> bool {
        return self.get_value() < self.target;
    }

    pub fn increase(&mut self) {
        if self.lower >= self.upper {
            panic!()
        }

        self.lower += 1;
    }

    pub fn decrease(&mut self) {
        if self.upper <= self.lower {
            panic!()
        }

        self.upper -= 1;
    }

    fn get_value(&self) -> i32 {
        let low = self.get_low();
        let high = self.get_high();

        return low + high;
    }

    fn get_low(&self) -> i32 {
        let index = self.lower as usize;

        return self.nums[index];
    }

    fn get_high(&self) -> i32 {
        let index = self.upper as usize;

        return self.nums[index];
    }
}