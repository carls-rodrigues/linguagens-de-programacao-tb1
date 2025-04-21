mod args;
mod binary_search;
use args::get_args;
use binary_search::binary_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (arg_array_size, arg_target, step) = get_args()?;

    if arg_array_size < 0 {
        return Err("Size must be non-negative".into());
    }

    let array: Vec<i64> = (0..)
        .step_by(step as usize)
        .take(arg_array_size as usize)
        .collect();
    let time = std::time::Instant::now();

    let index = binary_search(&array, arg_target).unwrap_or_else(|| {
        let message = format!("The number {} is not in the array", arg_target);
        println!("{}", message);
        std::process::exit(1);
    });
    println!(
        "Target value: {} | Found at index: {} | Value at index: {}",
        arg_target, index, array[index as usize]
    );

    println!("Elapsed time: {:.2?}", time.elapsed());
    Ok(())
}
