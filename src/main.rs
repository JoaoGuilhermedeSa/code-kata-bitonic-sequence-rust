use std::collections::VecDeque;

fn main() {
    // bitonic_array(n, l, r);
    let solution = bitonic_array(5, 3, 10);
    println!("Solution: {:?}", solution);
}

//usize to non negative integer
//i32 to integer
fn bitonic_array(n: usize, start: i32, end: i32) -> Vec<i32> {
    println!("Input values n:{} l:{} r:{}", n, start, end);
    
    //let mut my_array_solution = VecDeque::new();
    let mut my_array_solution = VecDeque::with_capacity(n);

    //Step 1: If it's not possible
    if n > ((end-start) *2 +1).try_into().unwrap()  {
        return vec![-1];
    }

    //2. Start with 'end'-1 as it is the maximum value less than peak value 'end'.
    let first = if end - 1 >= start { end - 1 } else { end };
    my_array_solution.push_back(first);

    if n >= 2 { my_array_solution.push_back(end); }

    //3. While the size of deque is less than n:
    //- Add increasing elements from r-2 down to L at the head of deque.
    //- Add decreasing elements from r down to l at the tail of deque.
    for k in 2..n {
        let val = end - ((k - 1) as i32);
        let val = if val < start { start } else { val };
        my_array_solution.push_back(val);
    }
    
    //Step 4. Convert the deque into array.
    my_array_solution.into_iter().collect()
}

//output 9 10 9 8 7 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let seq = bitonic_array(5, 3, 10);
        assert_eq!(seq, vec![9, 10, 9, 8, 7]);
    }

    #[test]
    fn test_case2_min_size() {
        let seq = bitonic_array(1, 3, 10);
        assert_eq!(seq, vec![9]); // só o primeiro elemento
    }

}