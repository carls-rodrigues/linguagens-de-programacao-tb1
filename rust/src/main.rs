mod args;
mod binary_search;
use args::get_args;
use binary_search::binary_search;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (arg_array_size, arg_target) = get_args()?;

    if arg_array_size < 0 {
        return Err("Size must be non-negative".into());
    }

    let array: Vec<i64> = (0..(arg_array_size + 1)).collect();

    let time = std::time::Instant::now();

    let index = binary_search(&array, arg_target).unwrap_or_else(|| {
        let message = format!("The number {} is not in the array", arg_target);
        println!("{}", message);
        std::process::exit(1);
    });
    println!(
        "The index is: {:?} --- Item selected: {:?}",
        index, array[index as usize]
    );

    println!("Elapsed time: {:.2?}", time.elapsed());
    Ok(())
}
