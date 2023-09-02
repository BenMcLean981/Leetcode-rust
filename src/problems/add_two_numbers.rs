impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut builder = SolutionBuilder::new(l1, l2);
        builder.solve();

        return vec_to_linked_list(builder.solution);
    }
}

struct SolutionBuilder {
    solution: Vec<i32>,
    n1: Option<Box<ListNode>>,
    n2: Option<Box<ListNode>>,
    carry: bool
}

impl SolutionBuilder {
    pub fn new(n1: Option<Box<ListNode>>, n2: Option<Box<ListNode>>) -> Self {
        return SolutionBuilder {
            solution: vec![],
            n1,
            n2,
            carry: false
        }
    }

    pub fn solve(&mut self) {
        loop {
            let state = self.get_state();

            match state {
                State::Done => {
                    if self.carry {
                        self.append(0); // append handles the carry
                    }
                    break;
                },
                State::First(val) => self.append(val),
                State::Second(val) => self.append(val),
                State::Both(val1, val2) => self.append(val1 + val2),
            }
            
            self.index();
        }
    }

    fn get_state(&self) -> State {
        match &self.n1 {
            Some(v1) => self.get_state_with_first(v1),
            None => self.get_state_without_first(),
        }
    }

    fn get_state_with_first(&self, v1: &Box<ListNode>) -> State {
        match &self.n2 {
            Some(v2) => State::Both(v1.val, v2.val),
            None => State::First(v1.val),
        }
    }
    
    fn get_state_without_first(&self) -> State {
        match &self.n2 {
            Some(v2) => State::Second(v2.val),
            None => State::Done
        }
    }

    fn append(&mut self, val: i32) {
        let n = val + self.get_carry_digit();
        let tenths = n % 10;

        self.carry = tenths != n;
        self.solution.push(tenths);
    }

    fn get_carry_digit(&self) -> i32 {
        match self.carry {
            true => 1,
            false => 0,
        }
    }

    fn index(&mut self) {
        if let Some(mut node) = self.n1.take() {
            self.n1 = node.next.take();  // Move to the next node.
        }

        if let Some(mut node) = self.n2.take() {
            self.n2 = node.next.take();  // Move to the next node.
        }
    }
}

enum State {
    Done,
    First(i32),
    Second(i32),
    Both(i32, i32)
}


pub (crate) fn vec_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        let new_node = Box::new(ListNode::new(num));
        *tail = Some(new_node);
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

pub (crate) fn make_vector(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }

    result
}

pub (crate) struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub (crate) struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
