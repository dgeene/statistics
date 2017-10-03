/*
 * Objectives
 *
 * 1. sort the values
 * 2. find the mean, median
 * 3. find the std dev
 * 4. find the iqr
 * 5. read from file
 *
 *
 * wishlist
 * - read from xml
 *
 */
fn main() {
    let input = [50, 61, 44, 68, 72, 75, 64,
        76, 84, 102, 86, 94];

    //let sorted = sort();
    sort(&input);
}

fn sort(unsorted: &[i32]) {
    let mut sorted: Vec<i32> = Vec::new();
    let mut val = unsorted[0];
    // find the smallest value
    for (i, elem) in unsorted.iter().enumerate() {
        //println!("elem: {}", elem);
        if val < elem {
            val = elem;
        }
    }
    println!("smallest elem: {}", val)
}
