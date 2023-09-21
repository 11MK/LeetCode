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

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list1 = list1;
    let mut list2 = list2;
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    while list1.is_some() && list2.is_some() {
        let (l1, l2) = (list1.as_mut().unwrap(), list2.as_mut().unwrap());
        if l1.val < l2.val {
            let next = l1.next.take();
            tail.next = list1;
            list1 = next;
        } else {
            let next = l2.next.take();
            tail.next = list2;
            list2 = next;
        }
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = if list1.is_some() { list1 } else { list2 };
    head.next
}

fn main() {
}
