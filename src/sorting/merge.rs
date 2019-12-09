//! Classic merge algorithm
//! 
//! Shows how to do an in-place sort using a merge sort algorithm.  The idea is to split the 
//! array, until it has only 1 or 2 elements.  we can then sort the sub arrays trivially. 
//! once all the sub arrays are sorted, we walk back up, comparing each element.
//! 
//! For example, suppose we have [5, 6, 3, 4, 8, 7, 1]
//! [5, 6, 3], [4, 8, 7, 1]
//! [5], [6, 3], [4, 8], [7, 1]
//! [5], [3, 6], [4, 8], [1, 7] <= At this point each sub-slice is ordered and the recursion ends
//! ([5], [3, 6]), ([4, 8], [1, 7]) <= now we recombine.  compare the first element of each pair in the tuple
//! (3): [5], [6] - (1): [4, 8], [7]
//! (3): [5, 6] - (1): [4, 8]
//! [3, 5, 6] - [1, 4, 8]


use std::ops::Range;

pub trait NumTrait: PartialEq + PartialOrd + Ord { }

pub fn split<T: NumTrait>(coll: &[T]) -> (Range<usize>, Range<usize>) {
  if coll.len() < 2 {
      return (0..0, 0..0)
  }
  let split_pt = (coll.len() / 2) - 1;  // if we have length 2 array, split at 0
  let half1 = 0..split_pt;
  let half2 = split_pt+1..coll.len()-1;
  (half1, half2)
}

pub fn combine<T: NumTrait>(_coll1: &mut [T], _coll2: &mut [T]) {
  
}