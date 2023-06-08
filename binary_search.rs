use std::cmp::Ordering;
pub fn b_search(item: i32, arr: &[i32; 5]) {
    let mut s: usize = 0;
    let mut e: usize = arr.len() - 1;
    let k = bs(item, &mut s, &mut e, &arr);
    if k != usize::MAX {
        println!("the element {} is found in index:{} in the array ", item, k);
    } else {
        println!("the element is not found in the array");
    }
}
fn bs(item: i32, s: &mut usize, e: &mut usize, arr: &[i32; 5]) -> usize {
    if s < e {
        let mid: usize = (*s + *e) / 2;
        match item.cmp(&arr[mid]) {
            Ordering::Less => *e = mid,
            Ordering::Equal => return mid,
            Ordering::Greater => *s = mid + 1,
        }
        return bs(item, s, e, &arr);
    } else {
        return usize::MAX;
    }
}
