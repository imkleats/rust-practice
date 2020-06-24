mod quadratic {
    pub fn three_sum(array: &Vec<i32>, sum: i32) -> i32 {
        let mut total = 0;
        for i in 0..array.len() {
            for j in (i+1)..array.len() {
                for k in (j+1)..array.len() {
                    if array[i] + array[j] + array[k] == sum {
                        total += 1;
                    }
                }
            }
        }
        total
    }
}

mod linearithmic {
    fn binary_search(array: &[i32], find: i32) -> Option<usize> {
        let mut lo = 0;
        let mut hi = array.len() - 1;
        // let mut present = false;
        while lo <= hi {
            let mid = lo + (hi.saturating_sub(lo) / 2);
            println!("Lo: {}; Hi: {}; Mid: {}", lo, hi, mid);
            if find < array[mid] {
                hi = mid.saturating_sub(1);
            } else if find > array[mid] {
                lo = mid.saturating_add(1);
            } else {
                return Some(mid);
            }
            if lo == mid && mid == hi {
                break
            }
        }
        None
    }
    pub fn three_sum(array: &mut Vec<i32>, sum: i32) -> i32 {
        array.sort();
        println!("The array to be summed is {:?}", array);
        let mut total = 0;
        for i in 0..array.len() {
            println!("Outer loop: i = {}", i);
            for j in (i+1)..array.len() {
                println!("Inner loop: j = {}", j);
                let check = sum - array[i] - array[j];
                println!("{}",check);
                if let Some(key) = binary_search(array, check) {
                    if i < j && j < key {
                        total = total + 1;
                    }
                }
            }
        }
        total
    }
}

fn main() {
    let mut my_list = vec![30, -40, -20, -10, 40, 0, 10, 5];
    let slow_matches = quadratic::three_sum(&my_list, 0);
    let faster_matches = linearithmic::three_sum(&mut my_list, 0);
    println!("My slow algorithm for {} sets of three numbers adding to 0.", slow_matches);
    println!("My fast algorithm for {} sets of three numbers adding to 0.", faster_matches);
    println!("Hello, world!");
}
