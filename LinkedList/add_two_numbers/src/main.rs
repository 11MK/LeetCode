// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let (mut s1, mut s2) = (0, 0);
        let (mut i, mut j) = (0, 0);
        let mut res: Option<Box<ListNode>> = None;
        while l1.is_some() {
            let n: ListNode = *l1.clone().unwrap();
            s1 += n.val * (10_i32.pow(i));
            l1 = n.next;
            i += 1;
        }
        while l2.is_some() {
            let n: ListNode = *l2.clone().unwrap();
            s2 += n.val * (10_i32.pow(j));
            l2 = n.next;
            j += 1;
        }
        let binding = (s1 + s2).to_string();
        println!("{:?}", binding);
        let sum = binding.chars();
        for n in sum {
            let mut node = ListNode::new(n.to_digit(10).unwrap() as i32);
            node.next = res;
            res = Some(Box::new(node));
        }
        res
    }
}

fn main() {
    println!("Solution:")
}

#[cfg(test)]
mod tests {
    use crate::ListNode;

    use super::Solution;

    #[test]
    fn test_1() {
        let l1 = ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };
        let l2 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        };
        let expected = ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        };
        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(expected))
        );
    }
    #[test]
    fn test_2() {
        let l1 = ListNode { val: 0, next: None };
        let l2 = ListNode { val: 0, next: None };
        let expected = ListNode { val: 0, next: None };
        assert_eq!(
            Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(expected))
        );
    }
    // Test case for l1 =[9], l2 = [1,9,9,9,9,9,9,9,9,9], output: [0,0,0,0,0,0,0,0,0,0,1]
}
