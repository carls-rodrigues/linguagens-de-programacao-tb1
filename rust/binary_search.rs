fn binary_search(items: &[usize], target: usize) -> Option<usize> {
    let mut high = items.len() - 1;
    let mut low = 0;

    while low <= high {
        let mid = (low + high) / 2;
        if items[mid] == target {
            return Some(mid);
        }
        if items[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return None;
}

fn main() {
    let array: Vec<usize> = (0..1000).collect();

    let search = binary_search(&array, 256);
    println!("{:?}", search)
}
