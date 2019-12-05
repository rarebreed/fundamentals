//! This is a solution to the problem of finding all possible permutations of coins for a given
//! amount of money.  For example, 78 cents could be 3 quarters and 3 pennies, and 7 dimes, a 
//! nickel and 3 pennies,etc. It must find all possible permutations
//! 
//! To solve this problem, build this up recursively both in the literal sense, and also in the
//! sense of breaking down the problem.  Imagine that you only have one type of coin, a penny.
//! This solution is trivial.  Next, consider if you only have nickels and pennies.  How would you
//! solve this problem then?
//! 
//! This solution works by taking the largest denomination, and assuming it starts at 0 coins.  It
//! then makes a call to the next highest denomination, also starting at 0 coins.  It does this
//! repeatedly, until it gets to the lowest coin type. It will recurse, as long as the remainder
//! is greater than the next coin type by adding one of that coin type, and repeating the process
//! 
//! Many recursive problems are reductive in the sense that they break the problem down into smaller
//! parts until they hit the base case.  This is more similiar to induction in that we are
//! increasing some value and decide when to stop the induction
//! 
//! TODO: Make an iterative rather than recursive solution to this problem to get around stack overflow

#[derive(Copy, Clone, Debug)]
pub struct Change {
    pub q: usize,
    pub d: usize,
    pub n: usize,
    pub p: usize
}

impl Change {
    pub fn new() -> Self {
        Change {
            q: 0, d: 0, n: 0, p: 0
        }
    }

    pub fn amount(&self) -> usize {
      (self.q * 25) + (self.d * 10) + (self.n * 5) + self.p
    }
}

pub fn find_quarters(num: usize, remainder: usize, current: Change, change: &mut Vec<Change>) {
  find_dimes(0, remainder, current, change);

  if remainder >= 25 {
    let mut next = current;
    next.q += 1;
    find_quarters(num + 1, remainder - 25, next, change); 
  }
}

pub fn find_dimes(num: usize, remainder: usize, current: Change, change: &mut Vec<Change>) {
  find_nickels(0, remainder, current, change);
  
  if remainder >= 10 {
    let mut next = current;
    next.d += 1;
    find_dimes(num + 1, remainder - 10, next, change);
  }
}

pub fn find_nickels( num: usize, 
                     remainder: usize,
                     current: Change,
                     change: &mut Vec<Change>) {
    set_amount(remainder, current, change);
    if remainder > 5 {
        let mut next = current;
        next.n += 1;
        find_nickels(num + 1, remainder - 5, next, change);
    }
}

pub fn set_amount( remainder: usize, mut current: Change, change: &mut Vec<Change>) {
  current.p = remainder;
  change.push(current);
}


pub fn make_denom(denom: usize) -> impl Fn(usize, usize, Change, &mut Vec<Change>) -> () {
  let fun = move |num: usize, remainder: usize, mut current: Change, change: &mut Vec<Change>| {
    current.p = remainder;
    change.push(current);
    if remainder > denom {
        let mut next = current;
        next.n += 1;
        find_nickels(num + 1, remainder - denom, next, change);
    }
  };
  fun
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_change(combos: &Vec<Change>, check: usize) {
      let x = combos.iter();
      x.for_each(|v| {
        let amt = v.amount();
        assert_eq!(amt, check);
      });
      println!("HOF(25): {:#?}", combos);
    }

    #[test]
    fn test_nickels() {
        let curr = Change::new();
        let mut combos: Vec<Change> = vec![];
        let find_nick = make_denom(5);
        find_nick(0, 33, curr, &mut combos);
        println!("HOF(5): {:#?}", combos);
    }

    #[test]
    fn test_dimes() {
        let curr = Change::new();
        let mut combos: Vec<Change> = vec![];
        find_dimes(0, 26, curr, &mut combos);
        println!("HOF(10): {:?}", combos);
    }

    #[test]
    fn test_quarters() {
        let curr = Change::new();
        let mut combos: Vec<Change> = vec![];
        find_quarters(0, 56, curr, &mut combos);

        check_change(&combos, 56);
    }
}