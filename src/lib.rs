//! # Bitonic Sequence Generator
//!
//! This crate provides functionality to generate bitonic sequences of length n
//! from integers within a specified range [l, r].
//!
//! A bitonic sequence is a sequence that first increases and then decreases,
//! or vice versa. This implementation uses a deque-based approach for O(n)
//! time complexity.
//!
//! ## Example
//!
//! ```rust
//! use code_kata_bitonic_sequence_rust::get_bitonic_sequence;
//!
//! let sequence = get_bitonic_sequence(5, 3, 10);
//! println!("{:?}", sequence);
//! ```
//!

/// Generates a bitonic sequence of length `n` using integers from the range `[start, end]`.
///
/// A bitonic sequence is one that first increases monotonically and then decreases
/// monotonically (or vice versa). This function implements an efficient O(n) algorithm
/// using a deque data structure.
///
/// # Arguments
///
/// * `n` - The desired length of the bitonic sequence
/// * `start` - The minimum value (inclusive) of the range
/// * `end` - The maximum value (inclusive) of the range
///
/// # Returns
///
/// Returns a `Vec<i32>` containing the bitonic sequence. If it's impossible to create
/// a bitonic sequence of the requested length with the given range, returns `vec![-1]`.
///
/// # Algorithm
///
/// 1. Check if `n > (end - start) * 2 + 1`. If true, return `[-1]` as it's impossible.
/// 2. Start with `end - 1` as the initial value (maximum value less than peak).
/// 3. Build the sequence using a deque:
///    - Add decreasing elements from `end` down to `start` at the tail
///    - Add increasing elements from `end - 2` down to `start` at the head
/// 4. Convert the deque to a vector and return.
///
/// # Examples
///
/// ```rust
/// # use code_kata_bitonic_sequence_rust::get_bitonic_sequence;
/// // Generate a bitonic sequence of length 5 from range [3, 10]
/// let result = get_bitonic_sequence(5, 3, 10);
/// assert_ne!(result, vec![-1]);
/// assert_eq!(result.len(), 5);
///
/// // Impossible case - too long for the given range
/// let impossible = get_bitonic_sequence(100, 1, 5);
/// assert_eq!(impossible, vec![-1]);
/// ```
///
/// # Time Complexity
/// O(n) - Each element is added to the deque exactly once.
///
/// # Space Complexity
/// O(n)² - The deque stores exactly n elements.
pub fn get_bitonic_sequence(n: u32, l: i32, r: i32) -> Vec<i32> {

  //Step 1: If it's not possible
  if n > ((r - l) * 2 + 1) as u32 {
    return [-1].to_vec();
  }

  let mut dq = vec![];

  //2. Start with 'end'-1 as it is the maximum value less than peak value 'end'.
  dq.push(r - 1);


  //3. While the size of deque is less than n:  
  //- Add decreasing elements from r down to l at the tail of deque.
  let mut i = r;
  while i >=l && dq.len() < n as usize {
    dq.push(i);
    i -= 1;
  }
  //- Add increasing elements from r-2 down to L at the head of deque.
  let mut j = r - 2;
  while j >= l && dq.len() < n as usize {
    dq.insert(0, j);
    j -= 1;
  }

  dq
}