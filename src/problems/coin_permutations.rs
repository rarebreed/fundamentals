//! This is a solution to the problem of finding all possible permutations of coins for a given
//! amount of money.  For example, 78 cents could be 3 quarters and 3 pennies, and 7 dimes, a nickel and
//! 3 pennies,etc. It must find all possible permutations

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
}

pub fn find_quarters(num: usize, remainder: usize, current: Change, change: &mut Vec<Change>) {
  if remainder >= 10 {
      find_dimes(0, remainder, current, change);
  } else if remainder >= 5 {
      find_nickels(0, remainder, current, change);
  } else {
    find_pennies(remainder, current, change);
  }

  if remainder >= 25 {
    let mut next = current;
    next.q += 1;
    find_quarters(num + 1, remainder - 25, next, change); 
  }
}

pub fn find_dimes(num: usize, remainder: usize, current: Change, change: &mut Vec<Change>) {
  if remainder >= 5 {
    find_nickels(0, remainder, current, change);
  } else {
    find_pennies(remainder, current, change);
  }
  
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
    if remainder > 5 {
        find_pennies(remainder, current, change);
        let mut next = current;
        next.n += 1;
        find_nickels(num + 1, remainder - 5, next, change);
    } else {
        find_pennies(remainder, current, change);
    }
}

pub fn find_pennies( remainder: usize, mut current: Change, change: &mut Vec<Change>) {
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

    //#[test]
    fn test_nickels() {
        let curr = Change::new();
        let mut combos: Vec<Change> = vec![];
        //find_nickels(0, 9, curr, combos);
        let find_nick = make_denom(5);
        find_nick(0, 33, curr, &mut combos);
        println!("HOF(5): {:#?}", combos);
    }

    #[test]
    fn test_dimes() {
        let curr = Change::new();
        let mut combos: Vec<Change> = vec![];
        //find_nickels(0, 9, curr, combos);
        find_dimes(0, 26, curr, &mut combos);
        println!("HOF(10): {:?}", combos);
    }

    #[test]
    fn test_quarters() {
        let curr = Change::new();
        let mut combos: Vec<Change> = vec![];
        //find_nickels(0, 9, curr, combos);
        find_quarters(0, 26, curr, &mut combos);
        println!("HOF(25): {:#?}", combos);
    }
}