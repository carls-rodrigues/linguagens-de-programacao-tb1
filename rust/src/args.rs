use clap::{Arg, Command};

pub fn get_args() -> Result<i64, Box<dyn std::error::Error>> {
    let matches = Command::new("Binary Search")
        .version("1.0")
        .about("Performs binary search on a sorted range")
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .value_name("ROOT")
                .help("Root of the binary tree")
                .required(true)
                .value_parser(clap::value_parser!(i64)),
        )
        .get_matches();

    let root: i64 = *matches
        .get_one("root")
        .ok_or("The binary tree Root is needed")?;
    Ok(root)
}
