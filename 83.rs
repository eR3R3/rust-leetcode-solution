// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut mark: [i32; 201] = [0; 201];
        let mut current = &head;

        while let Some(node) = current {
            mark[(node.val + 100) as usize] += 1;
            current = &node.next;
        }

        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_head;

        for (index, time) in mark.into_iter().enumerate() {
            let value = (index - 100) as i32;
            if time >= 1 {
                tail.next = Some(Box::new(ListNode::new(value)));
                tail = tail.next.as_mut().unwrap();
            }
        }
        
        dummy_head.next
    }
}
