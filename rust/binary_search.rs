fn binary_search(items: &[usize], target: usize) -> Option<usize> {
    if items.is_empty() {
        return None;
    }
    if target < items[0] || target > items[items.len() - 1] {
        return None;
    }
    if items.len() == 1 {
        return if items[0] == target { Some(0) } else { None };
    }
    if items.len() == 2 {
        return if items[0] == target {
            Some(0)
        } else if items[1] == target {
            Some(1)
        } else {
            None
        };
    }

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
    let time = std::time::Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let mut size: Option<usize> = None;
    let mut target: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--size" | "-s" => {
                if i + 1 < args.len() {
                    size = Some(args[i + 1].parse().unwrap_or_else(|_| {
                        println!("The value for --size must be a valid number");
                        std::process::exit(1);
                    }));
                    i += 2;
                } else {
                    println!("--size requires a value");
                    std::process::exit(1);
                }
            }
            "--target" | "-t" => {
                if i + 1 < args.len() {
                    target = Some(args[i + 1].parse().unwrap_or_else(|_| {
                        println!("The value for --target must be a valid number");
                        std::process::exit(1);
                    }));
                    i += 2;
                } else {
                    println!("--target requires a value");
                    std::process::exit(1);
                }
            }
            arg if arg.starts_with("-") => {
                println!("Unknown option: {}", arg);
                std::process::exit(1);
            }
            _ => {
                i += 1;
            }
        }
    }

    let array_size = size.unwrap_or_else(|| {
        println!("--size argument is required");
        std::process::exit(1);
    });
    let target = target.unwrap_or_else(|| {
        println!("--target argument is required");
        std::process::exit(1);
    });

    let array: Vec<usize> = (0..(array_size + 1)).collect();

    let index = binary_search(&array, target).unwrap_or_else(|| {
        let message = format!("The number {} is not in the array", target);
        println!("{}", message);
        std::process::exit(1);
    });
    println!(
        "The index is: {:?} --- Item selected: {:?}",
        index, array[index]
    );

    println!("Elapsed time: {:.2?}", time.elapsed());
}
