use clap::{Arg, Command};

fn main() {
    let matches = Command::new("My Test Program")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(
            Arg::new("file")
                .short('d')
                .long("file")
                .required(true)
                .help("A cool file"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .required(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();

    if let Some(myfile) = matches.get_one::<String>("file") {
        println!("The file passed is: {}", myfile);
    }

    if let Some(num) = matches.get_one::<String>("num") {
        match num.parse::<i32>() {
            Ok(n) => println!("The number passed is: {}", n),
            Err(_) => println!("The number passed is not a valid integer."),
        }
    } else {
        println!("No idea what your favorite number is.");
    }
}

// cargo run --bin=cli-arguments -- -d src/main.rs -n 45
