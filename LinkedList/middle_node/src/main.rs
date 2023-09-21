#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    use std::collections::HashMap;
    let mut head = head;
    let node_map: HashMap<usize, Option<Box<ListNode>>> = HashMap::new();
    while head.is_some() {
        let node = head.take();
    }
}

fn main() {

}
