//! Sorting algorithm implementations for reference.
//!
//! These are educational implementations. For production use,
//! prefer the standard library's `sort_by` method.

use crate::rasterizer::Triangle;

/// Bubble sort triangles by avg_depth in descending order (furthest first).
///
/// Time complexity: O(n²)
/// Space complexity: O(1)
#[allow(dead_code)]
pub fn bubble_sort_by_depth_descending(triangles: &mut [Triangle]) {
    let len = triangles.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if triangles[j].avg_depth < triangles[j + 1].avg_depth {
                triangles.swap(j, j + 1);
            }
        }
    }
}

/// Merge sort triangles by avg_depth in descending order (furthest first).
///
/// Time complexity: O(n log n)
/// Space complexity: O(n)
#[allow(dead_code)]
pub fn merge_sort_by_depth_descending(triangles: &mut Vec<Triangle>) {
    let len = triangles.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = triangles[..mid].to_vec();
    let mut right = triangles[mid..].to_vec();

    merge_sort_by_depth_descending(&mut left);
    merge_sort_by_depth_descending(&mut right);

    *triangles = merge_descending(left, right);
}

/// Merge two sorted vectors into one, maintaining descending order by avg_depth.
fn merge_descending(left: Vec<Triangle>, right: Vec<Triangle>) -> Vec<Triangle> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter().peekable();
    let mut right_iter = right.into_iter().peekable();

    while left_iter.peek().is_some() && right_iter.peek().is_some() {
        // Descending: take the larger depth first
        if left_iter.peek().unwrap().avg_depth >= right_iter.peek().unwrap().avg_depth {
            result.push(left_iter.next().unwrap());
        } else {
            result.push(right_iter.next().unwrap());
        }
    }

    result.extend(left_iter);
    result.extend(right_iter);
    result
}

/// Quick sort triangles by avg_depth in descending order (furthest first).
///
/// Uses the Lomuto partition scheme with the last element as pivot.
///
/// Time complexity: O(n log n) average, O(n²) worst case
/// Space complexity: O(log n) for recursion stack
#[allow(dead_code)]
pub fn quick_sort_by_depth_descending(triangles: &mut [Triangle]) {
    if triangles.len() <= 1 {
        return;
    }

    let pivot_idx = partition_descending(triangles);

    // Sort left partition (elements before pivot)
    if pivot_idx > 0 {
        quick_sort_by_depth_descending(&mut triangles[..pivot_idx]);
    }
    // Sort right partition (elements after pivot)
    if pivot_idx + 1 < triangles.len() {
        quick_sort_by_depth_descending(&mut triangles[pivot_idx + 1..]);
    }
}

/// Partition the slice around a pivot (Lomuto scheme).
///
/// After partitioning:
/// - Elements with depth >= pivot are on the left
/// - Elements with depth < pivot are on the right
/// - Pivot is in its final sorted position
///
/// Returns the final index of the pivot element.
fn partition_descending(triangles: &mut [Triangle]) -> usize {
    let pivot_idx = triangles.len() - 1;
    let pivot_depth = triangles[pivot_idx].avg_depth;

    // i tracks where the next "large" element should go
    let mut i = 0;

    for j in 0..pivot_idx {
        // For descending order: larger depths go first
        if triangles[j].avg_depth >= pivot_depth {
            triangles.swap(i, j);
            i += 1;
        }
    }

    // Place pivot in its final position
    triangles.swap(i, pivot_idx);

    i
}
