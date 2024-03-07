use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!(<DIR> "Directory to transform").required(true))
        .get_matches();

    if let Some(name) = matches.get_one::<String>("DIR") {
        println!("Value for name: {:?}", name);
    }
}

