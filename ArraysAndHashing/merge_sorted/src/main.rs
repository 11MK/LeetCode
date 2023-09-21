fn merge(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    let (mut m, mut n) = (m as usize, n as usize);

    while n > 0 {
        if m > 0 && nums1[m-1] > nums2[n-1] {
            nums1[m+n-1] = nums1[m-1];
            m -= 1
        }
        else {
            nums1[m+n-1] = nums2[n-1];
            n -= 1
        }
    }
}

fn main() {
    let mut vec_a: Vec<i32> = vec![1,2,3,0,0,0];
    merge(&mut vec_a, 3, &mut [2,5,6], 3);
    assert_eq!(vec_a, vec![1,2,2,3,5,6]);
}

