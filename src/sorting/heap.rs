//! Defines how to sort a heap

use std::cmp::{ PartialEq, Eq, PartialOrd, Ord };
use std::fmt::{Display, Debug};

#[derive(Debug)]
pub struct Heap<T> {
  pub heap: Vec<T>,
}

impl<T> Heap<T>
  where T: PartialEq + Eq + PartialOrd + Ord + Copy + Display + Debug {
  pub fn new(t: Vec<T>) -> Self {
    Heap {
      heap: t
    }
  }

  /// Helper: finds the parentof an element
  pub fn get_parent(&self, i: usize) -> (usize, T) {
    if i > self.heap.len() {
      panic!("{} is past the bounds of the heap", i);
    }
    if i == 0 {
      return self.get_val(i);
    }
    let parent_node = i / 2;
    self.get_val(parent_node)
  }

  pub fn get_children(&self, i: usize) -> Vec<(usize, T)> {
    // [0, 1, 2, 3, 4, 5, 6]
    if self.heap.len() > i * 2 + 2 {
      return vec![(i*2+1, self.heap[i*2+1]), (i*2+2, self.heap[i*2+2])];
    }
    match (i * 2 + 2) - (self.heap.len() - 1) {
      0 => vec![(i*2+1, self.heap[i*2+1]), (i*2+2, self.heap[i*2+2])],
      1 => vec![(i*2+1, self.heap[i*2+1])],
      _ => vec![]
    }
  }

  /// Helper: gets the value of a node from a given position
  pub fn get_val(&self, i: usize) -> (usize, T) {
    (i, self.heap[i])
  }

  /// Helper: Returns a pair of tuples representing parent and child
  pub fn pair_up(&mut self, child: usize) -> ((usize, T), (usize, T)) {
    // get parent
    let par = self.get_parent(child); 
    let ctup = self.get_val(child);
    (par, ctup)
  }

  pub fn pair_down(&mut self, parent: usize) -> ((usize, T), Vec<(usize, T)>) {
    let par = self.get_val(parent);
    let kids = self.get_children(parent);
    (par, kids)
  }

  pub fn swap(&mut self, source: (usize, T), target: (usize, T)) -> bool {
    if target.1 > source.1 {
      println!("Swapping index [{}]={} with index [{}]={}", source.0, source.1, target.0, target.1);
      self.heap[source.0] = target.1;
      self.heap[target.0] = source.1;
      true
    }  else {
      false
    }
  }

  pub fn swap_fn( &mut self
                , source: (usize, T)
                , target: (usize, T)
                , cmp: impl Fn(T, T) -> bool)
                -> bool {
    if cmp(source.1, target.1) {
      println!("Swapping index [{}]={} with index [{}]={}", source.0, source.1, target.0, target.1);
      self.heap[source.0] = target.1;
      self.heap[target.0] = source.1;
      println!("Heap is now {:?}", self.heap);
      true
    } else {
      false
    }
  }

  /// Checks if a newly added node should replace the parent
  pub fn float_up(&mut self, pos: usize) {
    if pos == 0 {
      return;
    }
    let (parent, child) = self.pair_up(pos);
    let _pt = parent.1;
    let _ct = child.1;
    let swapped = self.swap_fn(parent, child, |_pt, _ct| {
      _ct > _pt
    });

    if swapped && pos > 1 {
      self.float_up(parent.0);
    }
  }

  pub fn sink_down(&mut self, pos: usize) {
    if pos >= self.heap.len() {
      return;
    }
    let (parent, child) = self.pair_down(pos);
    let (pi, _pv) = parent;

    let mut swapped = false;
    for cd in child.into_iter() {
      let (ci, _cv) = cd;
      swapped = self.swap_fn(parent, cd, |_pv, _cv| {
        _pv < _cv
      });

      // if parent less than child, sink down and recurse
      if swapped && (pos * 2 + 2) < self.heap.len() {
        self.sink_down(ci);
        break;
      }
    }
    // if we swapped, we now have to check the new parent, and sink it possibly
    if swapped {
      println!("Checking if new swapped parent at {} is smaller than children: {:?}", pi, self.heap);
      self.sink_down(pi);
    }
  }


  pub fn add(&mut self, val: T) {
    self.heap.push(val);

    self.float_up(self.heap.len() - 1);
    println!("{:?}", self.heap);
  }

  pub fn remove(&mut self) -> T {
    let head = self.heap[0];
    if let Some(last) = self.heap.pop() {
      self.heap[0] = last;
    }
    self.sink_down(0);
    head
  }
}

impl<T> Default for Heap<T>
  where T: PartialEq + Eq + PartialOrd + Ord {
  fn default() -> Self {
    Heap {
      heap: Vec::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_1() {
    let mut hp = Heap::default();
    hp.add(10);

    println!("{:?}", hp);
  }

  #[test]
  fn test_add_2() {
    let mut hp = Heap::default();
    hp.add(20);
    hp.add(5);
    hp.add(10);
    hp.add(22);
    hp.add(30);
    hp.add(7);
    hp.add(15);
    hp.add(25);

    println!("{:?}", hp)
  }

  #[test]
  fn test_get_children() {
    let heap = Heap::new(vec![0,1,2,3,4,5,6, 7,8,9,10]);

    let kids = heap.get_children(3);
    println!("{:?}", kids)
  }

  #[test]
  fn test_sink() {
    let mut heap = Heap::new(vec![5,4,3,2,1]);
    let head = heap.remove();
    let h2 = heap.remove();
    let h3 = heap.remove();

    println!("head = {}, h2 = {}, h3 = {}, {:?}", head, h2, h3, heap);
  }
}
