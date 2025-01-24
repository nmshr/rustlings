/// Returns the number of "good" starting indices in `arr`.
/// A "good" index `i` is one where you can reach the last index
/// by alternating odd-even jumps under the given rules.
fn odd_even_jumps(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return n as i32;  // If there's 0 or 1 element, all indices trivially reach the end
    }

    // Step 1: Precompute next indices for odd jumps (next >= current)
    let mut odd_next = vec![-1; n];
    // Sort indices by (value, index) ascending
    let mut indices_sorted_by_value: Vec<usize> = (0..n).collect();
    indices_sorted_by_value.sort_by_key(|&i| (arr[i], i));

    // Monotonic-stack style approach (similar to "next greater or equal")
    let mut stack = Vec::new();
    for &i in &indices_sorted_by_value {
        // While there's an index on top of the stack that should jump here
        // (the condition is simply `stack[-1] < i` because the sort order
        // guarantees arr[i] >= arr[stack[-1]], fulfilling ">= current value").
        while let Some(&top) = stack.last() {
            if top < i {
                stack.pop();
                odd_next[top] = i as isize;
            } else {
                break;
            }
        }
        stack.push(i);
    }

    // Step 2: Precompute next indices for even jumps (next <= current)
    let mut even_next = vec![-1; n];
    // Sort indices by (value descending, index ascending)
    let mut indices_sorted_by_value_desc: Vec<usize> = (0..n).collect();
    // We can do negative-value sorting by using -arr[i] as the primary key
    // and i as the secondary key (which keeps ties in ascending index order).
    indices_sorted_by_value_desc.sort_by_key(|&i| (-(arr[i] as i64), i));

    stack.clear();
    for &i in &indices_sorted_by_value_desc {
        // Similar logic, but for "next smaller or equal"
        while let Some(&top) = stack.last() {
            if top < i {
                stack.pop();
                even_next[top] = i as isize;
            } else {
                break;
            }
        }
        stack.push(i);
    }

    // Step 3: Dynamic Programming arrays
    // odd[i] = can we reach the end if our next jump from i is odd-numbered?
    // even[i] = can we reach the end if our next jump from i is even-numbered?
    let mut odd = vec![false; n];
    let mut even = vec![false; n];

    // If we're already at the end, we trivially can be "at the end"
    odd[n - 1] = true;
    even[n - 1] = true;

    // Step 4: Fill DP from right to left
    // odd[i] = even[ odd_next[i] ]
    // even[i] = odd[ even_next[i] ]
    for i in (0..(n - 1)).rev() {
        if odd_next[i] != -1 {
            odd[i] = even[odd_next[i] as usize];
        }
        if even_next[i] != -1 {
            even[i] = odd[even_next[i] as usize];
        }
    }

    // Step 5: Count indices where odd[i] = true (start with an odd jump)
    odd.iter().filter(|&&can_reach| can_reach).count() as i32
}

// ------------------------------------------------------
// Example usage:
fn main() {
    let arr = vec![10, 13, 12, 14, 15];
    let answer = odd_even_jumps(arr);
    println!("Number of good starting indices: {}", answer); 
    // For the example [10, 13, 12, 14, 15], output should be 2.
}
