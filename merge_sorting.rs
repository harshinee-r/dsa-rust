pub fn ms(arr: &mut [i32]) {
    let u = arr.len() - 1;

    ms_h(arr, 0, u);
}

fn ms_h(arr: &mut [i32], s: usize, e: usize) {
    if s >= e {
        return;
    } else {
        let k: usize = (e + s) / 2;
        ms_h(arr, s, k);
        ms_h(arr, k + 1, e);
        let mut a: usize = s;
        let mut b: usize = k + 1;
        let mut arrr_ = Vec::new();
        let mut i = 0;
        while i < e - s + 1 {
            if a <= k && b <= e {
                if arr[a] <= arr[b] {
                    arrr_.push(arr[a]);
                    a += 1;
                } else {
                    arrr_.push(arr[b]);
                    b += 1;
                }
            } else if b > e {
                arrr_.push(arr[a]);
                a += 1;
            } else if a > k {
                arrr_.push(arr[b]);
                b += 1;
            }
            i += 1;
        }

        let mut l = 0;
        while l + s <= e {
            arr[l + s] = arrr_[l];
            l += 1;
        }
        return;
    }
}
