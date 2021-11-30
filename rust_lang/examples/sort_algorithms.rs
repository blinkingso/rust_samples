use std::fmt::Debug;

use rand::prelude::SliceRandom;
use std::cmp::Ordering;

/// sorting algorithm.
fn main() {}

pub trait Sorted {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone;
}

pub fn sort<T: Ord + Debug + Clone, S: Sorted>(to_sort: &mut [T]) {
    S::sort(to_sort);
}

/// 1. Compare a pair of adjacent items (a,b)
/// 2. Swap that pair if the items are out of order(in this case, when a > b)
/// 3. Repeat Step 1 and 2 until we reach the end of array
/// 4. By now, the largest item will be at the last position. We then reduce N by 1 and repeat Step 1 until we have N = 1.
pub(crate) struct BubbleSort;

impl Sorted for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone,
    {
        if slice.is_empty() {
            return;
        }
        let mut swapped = true;
        while swapped {
            // if no swapping that means array is sorted.
            // for example: -3, -2, -1, 3, 5, 8, 10, 20
            swapped = false;
            for i in 0..slice.len() - 1 {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

/// selection sort
/// Given an array of N items and L = 0, Selection Sort will:
/// 1. find the position X of the smallest item in the range of [L..N-1]
/// 2. Swap X-th item with the L-th item.
/// 3. Increase the lower-bound L by 1 and repeat Step 1 until L = N-2.
pub(crate) struct SelectionSort;

impl Sorted for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone,
    {
        if slice.is_empty() {
            return;
        }
        for l in 0..slice.len() - 2 {
            let mut min_pos = l;
            let mut min = &slice[l];
            for x in l..slice.len() {
                if min > &slice[x] {
                    // store min and min_pos
                    min = &slice[x];
                    min_pos = x;
                }
            }

            // swap pos x and l;
            slice.swap(min_pos, l);

            println!("{}=> {:?}", l, slice);
        }
    }
}

/// Insertion Sort.
/// 1. Start with one card in your hand.
/// 2. Pick the next card and insert it into its proper sorted order,
/// 3. Repeat previous step for all cards.
pub(crate) struct InsertionSort;
impl Sorted for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone,
    {
        if slice.is_empty() {
            return;
        }

        for i in 1..slice.len() {
            let mut unsorted = i;
            while unsorted >= 1 && slice[unsorted] < slice[unsorted - 1] {
                // do swap
                slice.swap(unsorted, unsorted - 1);
                // move idx
                unsorted -= 1;
            }
        }
    }
}

/// MergeSort
/// Given an array of N items, Merge Sort will:
/// 1. Merge each pair of individual element (which is by default, sorted) into sorted arrays of 2
/// elements
/// 2. Merge each pair of sorted arrays of 2 elements into sorted arrays of 4 elements,
/// Repeat the process...
/// 3. Final step: Merge 2 sorted arrays of N/2 elements to obtain a fully sorted array of N elements.
pub(crate) struct MergeSort;

/// Merge Sort
impl Sorted for MergeSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone,
    {
        // 1. split to sub arrays.
        MergeSort::merge_sort(slice, 0, slice.len() - 1);
    }
}

impl MergeSort {
    /// sort sub arrays
    fn merge_sort<T: Ord + Debug + Clone>(slice: &mut [T], left: usize, right: usize) {
        if left < right {
            let mid = (left + right - 1) / 2;
            Self::merge_sort(slice, left, mid);
            Self::merge_sort(slice, mid + 1, right);
            Self::merge(slice, left, mid, right);
        }
    }

    /// merge together
    fn merge<T: Ord + Debug + Clone>(slice: &mut [T], left: usize, mid: usize, right: usize) {
        // 1. find the middle point to divide the array into two halves:
        // middle mid = left + right / 2
        // subarray1 = slice[left..mid], subarray2 = slice[mid+1..right], both sorted.
        // left subarray index low
        let mut left_idx_low = left;
        let left_idx_high = mid;

        let mut right_idx_low = mid + 1;
        let right_idx_high = right;

        let size = right - left + 1;
        let mut temp: Vec<T> = Vec::with_capacity(size);
        // merge two sub arrays.
        while left_idx_low <= left_idx_high && right_idx_low <= right_idx_high {
            let val = if slice[left_idx_low] < slice[right_idx_low] {
                let val = slice[left_idx_low].clone();
                left_idx_low += 1;
                val
            } else {
                let val = slice[right_idx_low].clone();
                right_idx_low += 1;
                val
            };
            temp.push(val);
        }

        // left remaining
        // print left remaining.
        if left_idx_low <= left_idx_high {
            let mut left_remaining = Vec::<T>::with_capacity(left_idx_high - left_idx_low + 1);
            for i in left_idx_low..=left_idx_high {
                left_remaining.push(slice[i].clone());
            }
            println!("left remaining: {:?}", left_remaining);
        }
        while left_idx_low <= left_idx_high {
            temp.push(slice[left_idx_low].clone());
            left_idx_low += 1;
        }
        // right remaining
        if right_idx_low <= right_idx_high {
            let mut right_remaining = Vec::<T>::with_capacity(right_idx_high - right_idx_low + 1);
            for i in right_idx_low..=right_idx_high {
                right_remaining.push(slice[i].clone());
            }
            println!("right remaining: {:?}", right_remaining);
        }
        while right_idx_low <= right_idx_high {
            temp.push(slice[right_idx_low].clone());
            right_idx_low += 1;
        }

        // copy back to slice
        for k in 0..size {
            slice[left + k] = temp[k].clone();
        }

        println!("now slice is : {:?}", temp);
    }
}

/// Quick Sort
/// Another Divide and Conquer sorting algorithm.
pub(crate) struct QuickSort;

impl Sorted for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone,
    {
        // shuffle the array.
        slice.shuffle(&mut rand::rngs::OsRng);
        QuickSort::quick_sort(slice, 0, slice.len() - 1);
    }
}

impl QuickSort {
    /// recursive partition to sort
    /// ```
    /// #[low] start of subarray.
    /// #[high] end of subarray.
    /// ```
    fn partition<T: Ord + Debug + Clone>(slice: &mut [T], low: usize, high: usize) -> usize {
        // shift element which less than slice[p] to the left of
        let v = slice[low].clone();
        let mut i = low + 1;
        let mut j = high;
        let p = loop {
            // from left to right to find out bigger ones.
            while v > slice[i] {
                if i == high {
                    break;
                }

                i += 1;
            }
            // from right ... left
            while v < slice[j] {
                if j == low {
                    break;
                }
                j -= 1;
            }

            // not need to swap
            if i >= j {
                break j;
            }

            slice.swap(i, j);
        };
        // set p to the actual position.
        slice.swap(low, p);

        p
    }

    fn quick_sort<T: Ord + Debug + Clone>(slice: &mut [T], low: usize, high: usize) {
        if low < high {
            // partition the array
            // let p = QuickSort::partition(slice, low, high);
            let p = QuickSort::simple_partition(slice, low, high);

            // sort
            QuickSort::quick_sort(slice, low, p - 1);
            QuickSort::quick_sort(slice, p + 1, high);
        }
    }

    fn simple_partition<T>(slice: &mut [T], low: usize, high: usize) -> usize
    where
        T: Debug + Clone + Ord,
    {
        let p = &slice[low].clone();
        let mut m = low;
        for k in (low + 1)..high {
            if &slice[k] < p {
                m += 1;
                slice.swap(k, m);
            }
        }

        slice.swap(low, m);

        m
    }
}

/// Quicksort with 3-way partitioning.
pub(crate) struct Quick3WaySort;

impl Quick3WaySort {
    fn quick_3way_sort<T>(slice: &mut [T], low: usize, high: usize)
    where
        T: Ord + Debug + Clone,
    {
        if high <= low {
            return;
        }
        let mut lt = low;
        let mut gt = high;
        let val = slice[low].clone();
        let mut i = low + 1;
        while i <= gt {
            match slice[i].cmp(&val) {
                Ordering::Less => {
                    slice.swap(i, lt);
                    lt += 1;
                    i += 1;
                }
                Ordering::Equal => {
                    i += 1;
                }
                Ordering::Greater => {
                    slice.swap(i, gt);
                    gt -= 1;
                }
            }
        }

        // data overflow.
        let hi = if lt as isize - 1 > 0 { lt - 1 } else { 0 };
        Self::quick_3way_sort(slice, low, hi);
        Self::quick_3way_sort(slice, gt + 1, high);
    }
}

impl Sorted for Quick3WaySort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Clone,
    {
        // slice.shuffle(&mut rand::rngs::OsRng);
        Quick3WaySort::quick_3way_sort(slice, 0, slice.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_bubble_sort() {
        let mut to_sort = vec![10, 9, 20, -1, 3, 5, 9, 8];
        let mut to_sort = vec![-1, 3, 5, 8, 10, -2, -3, 7, 20];
        /*
        0=> -1, 3, 5, 8, 10, -2, -3, 7, 20
        1=> -1, 3, 5, 8, -2, -3, 7, 10, 20
        2=> -1, 3, 5, -2, -3, 7, 8, 10, 20
        3=> -1, 3, -2, -3, 5, 7, 8, 10, 20
        4=> -1, -2, -3, 3, 5, 7, 8, 10, 20
        5=> -2, -3, -1, 3, 5, 7, 8, 10, 20
        6=> -3, -2, -1, 3, 5, 7, 8, 10, 20
        */
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 8, 10, 20];
        sort::<_, BubbleSort>(&mut to_sort);
        println!("{:?}", to_sort);
    }

    #[test]
    fn test_selection_sort() {
        let mut to_sort = vec![10, 9, 20, -1, 3, 5, 9, 8];
        let mut to_sort = vec![-1, 3, 5, 8, 10, -2, -3, 7, 20];
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 7, 8, 10, 20];
        /*
        0=> [-1, 3, 5, 8, 10, -2, -3, 7, 20]
        1=> [-3, 3, 5, 8, 10, -2, -1, 7, 20]
        2=> [-3, -2, 5, 8, 10, 3, -1, 7, 20]
        3=> [-3, -2, -1, 8, 10, 3, 5, 7, 20]
        4=> [-3, -2, -1, 3, 10, 8, 5, 7, 20]
        5=> [-3, -2, -1, 3, 5, 8, 10, 7, 20]
        6=> [-3, -2, -1, 3, 5, 7, 10, 8, 20]
        7=> [-3, -2, -1, 3, 5, 7, 8, 10, 20]
        */
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 8, 10, 20];
        sort::<_, SelectionSort>(&mut to_sort);
        println!("{:?}", to_sort);
    }

    #[test]
    fn test_insertion_sort() {
        let mut to_sort = vec![10, 9, 20, -1, 3, 5, 9, 8];
        let mut to_sort = vec![-1, 3, 5, 8, 10, -2, -3, 7, 20];
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 7, 8, 10, 20];
        /*
        0=> [-1, 3, 5, 8, 10, -2, -3, 7, 20]
        1=> [-3, 3, 5, 8, 10, -2, -1, 7, 20]
        2=> [-3, -2, 5, 8, 10, 3, -1, 7, 20]
        3=> [-3, -2, -1, 8, 10, 3, 5, 7, 20]
        4=> [-3, -2, -1, 3, 10, 8, 5, 7, 20]
        5=> [-3, -2, -1, 3, 5, 8, 10, 7, 20]
        6=> [-3, -2, -1, 3, 5, 7, 10, 8, 20]
        7=> [-3, -2, -1, 3, 5, 7, 8, 10, 20]
        */
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 8, 10, 20];
        sort::<_, InsertionSort>(&mut to_sort);
        println!("{:?}", to_sort);
    }

    #[test]
    fn test_merge_sort() {
        let mut to_sort = vec![10, 9, 20, -1, 3, 5, 9, 8];
        let mut to_sort = vec![-1, 3, 5, 8, 10, -2, -3, 7, 20];
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 7, 8, 10, 20];
        /*
        [-1, 3]
        [5, 8]
        [-1, 3, 5, 8]
        [-2, 10]
        [7, 20]
        [-3, 7, 20]
        [-3, -2, 7, 10, 20]
        [-3, -2, -1, 3, 5, 7, 8, 10, 20]
        */
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 8, 10, 20];
        sort::<_, MergeSort>(&mut to_sort);
        println!("{:?}", to_sort);
    }

    #[test]
    fn test_quick_sort() {
        let mut to_sort = vec![10, 9, 20, -1, 3, 5, 9, 8];
        let mut to_sort = vec![-1, 3, 5, 8, 10, -2, -3, 7, 20];
        let mut to_sort = Vec::<isize>::new();
        for i in 0..100 {
            let r = rand::thread_rng().gen_range(-100..=1000);
            to_sort.push(r);
        }
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 7, 8, 10, 20];
        /*
        now slice is : [20, 7, -2, 3, -1, 5, 10, -3, 8]
        now slice is : [7, 20, -2, 3, -1, 5, 10, -3, 8]
        now slice is : [7, -2, 20, 3, -1, 5, 10, -3, 8]
        now slice is : [7, -2, 3, 20, -1, 5, 10, -3, 8]
        now slice is : [7, -2, 3, -1, 20, 5, 10, -3, 8]
        now slice is : [7, -2, 3, -1, 5, 20, 10, -3, 8]
        now slice is : [7, -2, 3, -1, 5, 10, 20, -3, 8]
        now slice is : [7, -2, 3, -1, 5, 10, -3, 20, 8]
        now slice is : [7, -2, 3, -1, 5, 10, -3, 8, 20]
        now slice is : [7, -2, 3, -1, 5, 10, -3, 8, 20]
        now slice is : [-2, 7, 3, -1, 5, 10, -3, 8, 20]
        now slice is : [-2, 3, 7, -1, 5, 10, -3, 8, 20]
        now slice is : [-2, 3, -1, 7, 5, 10, -3, 8, 20]
        now slice is : [-2, 3, -1, 5, 7, 10, -3, 8, 20]
        now slice is : [-2, 3, -1, 5, -3, 10, 7, 8, 20]
        now slice is : [-2, 3, -1, 5, -3, 7, 10, 8, 20]
        now slice is : [-2, 3, -1, 5, -3, 7, 10, 8, 20]
        now slice is : [-3, 3, -1, 5, -2, 7, 10, 8, 20]
        now slice is : [-3, 3, -1, -2, 5, 7, 10, 8, 20]
        now slice is : [-3, 3, -2, -1, 5, 7, 10, 8, 20]
        now slice is : [-3, -2, 3, -1, 5, 7, 10, 8, 20]
        now slice is : [-3, -2, 3, -1, 5, 7, 10, 8, 20]
        now slice is : [-3, -2, -1, 3, 5, 7, 10, 8, 20]
        now slice is : [-3, -2, -1, 3, 5, 7, 10, 8, 20]
        now slice is : [-3, -2, -1, 3, 5, 7, 8, 10, 20]
         */
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 8, 10, 20];
        sort::<_, QuickSort>(&mut to_sort);
        println!("{:?}", to_sort);
    }

    #[test]
    fn test_quick_3way_sort() {
        let mut to_sort = vec![10, 9, 20, -1, 3, 5, 9, 8];
        let mut to_sort = vec![-1, 3, 5, 8, 10, -2, -3, 7, 20];
        let mut to_sort = Vec::<isize>::new();
        for i in 0..100 {
            let r = rand::thread_rng().gen_range(-100..=1000);
            to_sort.push(r);
        }
        let mut to_sort = vec!['A', 'A', 'A', 'A', 'A', 'A', 'A', 'A'];
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 7, 8, 10, 20];
        /*
         i = 1,
           -98, 834, 161, 531, 394, 633, 147, 17, 838, gt = 8
        -> -98, 838, 834, 161, 531, 394, 633, 147, 17, gt = 7
        -> -98, 17, 838, 834, 161, 531, 394, 633, 147, gt = 6
        -> -98, 147, 17, 838, 834, 161, 531, 394, 633, gt = 5
        -> -98, 633, 147, 17, 838, 834, 161, 531, 394, gt = 4
        -> -98, 394, 633, 147, 17, 838, 834, 161, 531, gt = 3
        -> -98, 531, 394, 633, 147, 17, 838, 834, 161, gt = 2
        -> -98, 161, 531, 394, 633, 147, 17, 838, 834, gt = 1
        -> -98, 834, 161, 531, 147, 17, 838, 834, 161, gt = 0

            */
        // let mut to_sort = vec![-3, -2, -1, 3, 5, 8, 10, 20];
        sort::<_, Quick3WaySort>(&mut to_sort);
        println!("{:?}", to_sort);
    }
}
