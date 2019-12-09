// Definition for singly-linked list.
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

pub struct Solution { }

impl Solution {
    /// For this problem, we are tasked with reversing a string
    /// The original problem called for taking as the only parameter, a &mut Vec<char>.  However, this doesn't really
    /// work in rust the way it would work in c(++).  In c(++), you could pass in a pointer and incremement on each call
    /// to the last index.  Rust doesn't work that way, at least, not without `unsafe`.  One could make it work by 
    /// copying the vector each time, but that would be prohibitively expensive.
    /// 
    /// This solution takes instead a reference to a slice.  Notice it is no longer mutable, as it does not need to be.
    /// This solution will pass a shortened slice on each recursive call (shorter from the left), until the length of 
    /// the slice is only 1.  At that point, the recursion will stop and print the 0th element of the slice.  The
    /// stack frame will pop off, and then go to the next stack frame.
    /// 
    /// Notice that we use a slice rather than a Vec.  This fulfills the original contract (it must take a mut ref to a
    /// Vec).  However, as noted above, a reference in Rust is not the same as a pointer in c(++).  Also, while a ref to
    /// a Vec _is_ a slice, a slice is not a reference to a Vec.
    pub fn reverse_string(s: &[char]){
      let len = s.len();
      let start = 0;

      if start < len {
          Solution::reverse_string(&s[(start+1)..]);
          println!("{}", s[start])
      }
    }

    /// For this problem, we must use recursion to reverse a string in-place.  This would be a rather easy problem to
    /// solve iteratively.  Since we can not allocate a new vector, we must swap the positions of the vector.  However,
    /// on each recursive call, we are only passing in the vector itself, and not a position.
    /// 
    /// This means we must "reduce" our way in.  On each successive call, we take off the ends of the slice until our
    /// slice is length 2 or 1.  Once our slice is length 2 or 1, we swap the ends (or middle, if the length is 1).
    pub fn reversed(s: &mut [char]) {
        let len = s.len();
        if len == 0 {
            return;
        }
        
        if len > 2 {
            Solution::reversed(&mut s[1..(len - 1)])
        }

        let swp = s[0];
        s[0] = s[len-1];
        s[len-1] = swp;
    }

    /// This function takes a linked list and swaps every two elements.  For example: (1 -> 2 -> 3 -> 4) will return
    /// (2 -> 1 -> 4 -> 3).  This is a bit tricky since using match and recursion has repercussions.  The fact that the
    /// return value is an Option<Box<ListNode>> can imply one of two things: 
    /// 
    /// - We return the original modified list
    /// - We generate a new list on the fly
    /// 
    /// Since head is not shown as mut, we have to do the latter, since we can not mutate the list in place. 
    pub fn swap_head(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut h) => {
                println!("recursing: {:?}", h);
                let swp_h = h.clone();
                match h.next {
                    Some(mut next) => {
                        let swp_n = next.clone();
                        let swp = Solution::swap_head(swp_n.next);
                        if let Some(swapped) = swp {
                            h = next;
                            
                            None
                        } else {
                            None
                        }
                    },
                    None => {
                        Some(swp_h)
                    }
                }
            },
            None => head
        }
    }

    fn test() {
        let val = 9;
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let test = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&test[..])
    }

    #[test]
    fn test_reversed() {
        let mut test = vec!['r', 'a', 'd', 'a', 'r'];
        Solution::reversed(&mut test);
        println!("{:?}", test);

        //test = vec!['s', 'e', 'a', 'n'];
        test = vec!['t', 'o', 'n', 'e', 'r'];
        //test = vec!['r', 'a', 't'];
        //test = vec!['o', 'n'];
        //test = vec!['a'];
        //test = vec![];
        Solution::reversed(&mut test);
        println!("{:?}", test);
    }

    #[test]
    fn test_swap_head() {
        let mut list = ListNode::new(1);
        let mut node2 = Box::new(ListNode::new(2));
        let mut node3 = Box::new(ListNode::new(3));
        let node4 = Box::new(ListNode::new(4));

        node3.next = Some(node4);
        node2.next = Some(node3);
        list.next = Some(node2);

        let nlist = Solution::swap_head(Some(Box::new(list)));
        println!("nlist is {:?}", nlist);
    }
}