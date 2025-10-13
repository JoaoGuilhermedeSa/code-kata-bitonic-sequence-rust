pub fn get_bitonic_sequence(n: i32, l: i32, r: i32) -> Vec<i32> {
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