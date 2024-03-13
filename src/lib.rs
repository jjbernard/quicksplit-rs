use clap::{Arg, ArgAction, Command};
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct MyFiles {
    files: Vec<String>,
}

pub fn split_str(my_str: &str) -> Vec<&str> {
    my_str.split('-').collect()
}

pub fn get_args() -> MyFiles {
    let matches = Command::new("qsplit")
        .version("0.1")
        .about(
            "Split a directory name along the '-' symbol \
        and convert it into a directory structure",
        )
        .arg(Arg::new("DIRs").required(true).action(ArgAction::Append))
        .get_matches();

    let args = matches
        .get_many::<String>("DIRs")
        .unwrap_or_default()
        .map(|v| v.to_owned())
        .collect::<Vec<String>>();

    MyFiles { files: args }
}

pub fn verify_dir(dir: String) -> bool {
    Path::new(dir.as_str()).is_dir()
}

pub fn new_dir_name(dir: String) -> String {
    let dir_split = split_str(dir.as_str());
    let mut dir_string = String::new();
    for w in dir_split {
        dir_string.push_str(w);
        dir_string.push('/');
    }

    dir_string
}

pub fn mk_dir(dir: String) -> std::io::Result<()> {
    let dir = new_dir_name(dir);
    fs::create_dir_all(dir)?;
    Ok(())
}

pub fn move_files(from: String, to: String) -> std::io::Result<()> {
    let path_from = Path::new(from.as_str());
    let path_to = Path::new(to.as_str());

    fs::rename(from, to)?;
    Ok(())
}

pub fn run() {
    todo!()
}
