pub fn binary_search(items: &[i64], target: i64) -> Option<i64> {
    if items.is_empty() {
        return None;
    }
    if target < items[0] || target > items[items.len() - 1] {
        return None;
    }
    if items.len() == 1 {
        return if items[0] == target { Some(0) } else { None };
    }

    let mut high = items.len() - 1;
    let mut low = 0;

    while low <= high {
        let mid = (low + high) / 2;
        if items[mid] == target {
            return Some(mid as i64);
        }
        if items[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return None;
}
