use std::collections::VecDeque;
//usize to non negative integer
//i32 to integer
pub fn get_bitonic_sequence(n: usize, start: i32, end: i32) -> Vec<i32> {
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

pub fn get_bitonic_sequence_v2(n: i32, l: i32, r: i32) -> Vec<i32> {
  if n > (r - l) * 2 + 1 {
    return [-1].to_vec();
  }

  let mut dq = vec![];

  dq.push(r - 1);

  let mut i = r;
  while i >=l && dq.len() < n as usize {
    dq.push(i);
    i -= 1;
  }

  let mut j = r - 2;
  while j >= l && dq.len() < n as usize {
    dq.insert(0, j);
    j -= 1;
  }

  dq
}