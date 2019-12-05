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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let test = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&test[..])
    }
}