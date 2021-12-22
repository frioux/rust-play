fn max<T: std::cmp::PartialOrd + Copy>(l: &[T]) -> T {
    let mut ret = l[0];
    for v in l {
        if v > &ret {
            ret = *v
        }
    }
    return ret
}

fn max2<T: std::cmp::PartialOrd + Clone>(l: &[T]) -> T {
    let mut ret = l[0].clone();
    for v in l {
        if v > &ret {
            ret = v.clone()
        }
    }
    return ret
}

// Clone largest
// Copy largest

// Try designing more experiments that vary the values and lifetimes of the references passed in to
// the longest function and how the returned reference is used. Make hypotheses about whether or
// not your experiments will pass the borrow checker before you compile; then check to see if
// you’re right!

fn main() {
    println!("{}", max2(&[1, 2]));
}
