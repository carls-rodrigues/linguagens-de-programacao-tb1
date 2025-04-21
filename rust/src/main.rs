mod binary_search;
use binary_search::binary_search;

fn main() {
    let time = std::time::Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let mut size: Option<i64> = None;
    let mut target: Option<i64> = None;

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
                    target = Some(args[i + 1].parse::<i64>().unwrap_or_else(|e| {
                        println!("--target: {:?}", e);
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

    let array: Vec<i64> = (0..(array_size + 1)).collect();

    let index = binary_search(&array, target).unwrap_or_else(|| {
        let message = format!("The number {} is not in the array", target);
        println!("{}", message);
        std::process::exit(1);
    });
    println!(
        "The index is: {:?} --- Item selected: {:?}",
        index, array[index as usize]
    );

    println!("Elapsed time: {:.2?}", time.elapsed());
}
