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
        match items[mid].cmp(&target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
            std::cmp::Ordering::Equal => return Some(mid as i64),
        }

        // match items[mid] {
        //     _ if items[mid] < target => low = mid + 1,
        //     _ if items[mid] > target => high = mid - 1,
        //     _ => return Some(mid as i64),
        // }
    }

    None
}
