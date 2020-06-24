use std::cmp::Ordering;
use rand::Rng;

fn bitonic_search(a: &[i32], find: i32, start: usize, end: usize) -> Option<usize> {
    
    let mid = start + (end - start)/ 2;
    let mid_lo = start + (mid - start) / 2;
    let mid_hi = mid + (end - mid) / 2;

    if start > end || end > a.len() - 1 {
        return None
    }
    
    if start == mid {
        return if a[mid] == find { Some(mid) } else if a[end] == find { Some(end) } else { None };
    }

    let found: Option<usize>;

    found = match recursive_binary(&a, find, start, mid_lo, true) {
            // recursive binary will return None if:
            //      + &a[start..mid_lo] is not sorted; or
            //      + term is not within slice
            Some(f) => Some(f),
            None => match recursive_binary(&a, find, mid_hi, end, false) {
                Some(f) => Some(f),
                None => bitonic_search(&a, find, mid_lo, mid_hi)
            }
    };

    found
}

fn recursive_binary(a: &[i32], find: i32, start: usize, end: usize, asc: bool) -> Option<usize> {

    if start > end || end > a.len() - 1 {
        return None
    }

    let mid = start + (end - start)/ 2;
    let (signed_find, signed_startvalue, signed_midvalue, signed_endvalue) =
        if asc { (find, a[start], a[mid], a[end]) } else { (-1*find, -1 * (a[start]), -1 * (a[mid]), -1 * (a[end]))};
    
    if  signed_startvalue > signed_find || signed_find > signed_endvalue ||
        signed_startvalue > signed_midvalue || signed_midvalue > signed_endvalue ||
        signed_startvalue > signed_endvalue {
        return bitonic_search(a, find, start, end)
    }

    if start == mid {
        return if a[mid] == find { Some(mid) } else if a[end] == find { Some(end) } else { None };
    }

    match signed_find.cmp(&signed_midvalue) {
        Ordering::Less => recursive_binary(&a, find, start, mid, asc),
        Ordering::Greater => recursive_binary(&a, find, mid, end, asc),
        _ => Some(mid)
    }

}

fn rb_search(a: &[i32], find: i32) -> Option<usize> {
    recursive_binary(a, find, 0, a.len()-1, a[0] < a[a.len() - 1])
}

fn rbt_search(a: &[i32], find: i32) -> Option<usize> {
    bitonic_search(a, find, 0, a.len()-1)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut asc_numbers: Vec<i32> = (0..150).map(|_| {
        // 1 (inclusive) to 21 (exclusive)
        rng.gen_range(-500, 501)
    }).collect();
    let mut desc_numbers: Vec<i32> = (0..150).map(|_| {
        // 1 (inclusive) to 21 (exclusive)
        rng.gen_range(-500, 501)
    }).collect();

    asc_numbers.sort();
    desc_numbers.sort();
    desc_numbers.reverse();

    let mut test = Vec::new();
    test.append(&mut asc_numbers);
    test.append(&mut desc_numbers);

    let find: i32 = rng.gen_range(-500, 501);

    match rb_search(&test[..150], find) {
        Some(r) => println!("I found something with reverse binary search at index: {}", r),
        None => println!("This rb_search didn't find {} in : {:?}", find, &test[..150])
    };
    match rb_search(&test[150..], find) {
        Some(r) => println!("I found something with normal binary search at index: {}", r),
        None => println!("This rb_search didn't find {} in : {:?}", find, &test[150..])
    };
    match rbt_search(&test, find) {
        Some(r) => println!("I found something at index: {}", r ),
        None => println!("That bitonic search didn't find {} in {:?}.", find, &test)
    }
    println!("Hello, world!");
}
