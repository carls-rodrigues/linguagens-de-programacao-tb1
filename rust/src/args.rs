use clap::{Arg, Command};

pub fn get_args() -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let matches = Command::new("Binary Search")
        .version("1.0")
        .about("Performs binary search on a sorted range")
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_name("SIZE")
                .help("Size of the array (non-negative integer)")
                .required(true)
                .value_parser(clap::value_parser!(i64)),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET")
                .help("Target number to find")
                .required(true)
                .value_parser(clap::value_parser!(i64)),
        )
        .get_matches();

    let array_size: i64 = *matches.get_one("size").ok_or("Size is required")?;
    let target: i64 = *matches.get_one("target").ok_or("Target is required")?;
    Ok((array_size, target))
}
