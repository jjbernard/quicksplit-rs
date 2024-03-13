use clap::{Arg, ArgAction, Command};
use std::path::Path;
use std::fs;

#[derive(Debug)]
pub struct MyFiles {
    files: Vec<String>,
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

pub fn split_str(my_str: &str) -> Vec<&str> {
    my_str.split('-').collect()
}

pub fn verify_dir(dir: &String) -> bool {
    Path::new(dir.as_str()).is_dir()
}

pub fn new_dir_name(dir: &String) -> String {
    let dir_split = split_str(dir.as_str());
    let mut dir_string = String::new();
    for w in dir_split {
        dir_string.push_str(w);
        dir_string.push('/');
    }

    dir_string
}

pub fn mk_dir(dir: &String) -> std::io::Result<()> {
    let dir = new_dir_name(dir);
    fs::create_dir_all(dir)?;
    Ok(())
}

pub fn move_files(from: &String, to: &String) -> std::io::Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

pub fn rm_dir(dir: &String) -> std::io::Result<()> {
    fs::remove_dir(&dir.as_str())?;
    Ok(())
}

pub fn run() {
    let dirs = get_args();

    // There are far too many match here... need to rework this by using ? instead.
    for f in dirs.files {
        if !verify_dir(&f) {
            eprintln!("{:?} is not a directory", f);
            continue
        } else {
            let new_dir = new_dir_name(&f);
            let res_create = mk_dir(&new_dir);

            match res_create {
                Ok(()) => {
                    let res_move = move_files(&f, &new_dir);
                    match res_move {
                        Ok(()) => {
                            let res_rm = rm_dir(&f);
                            match res_rm {
                                Ok(()) => continue,
                                _ => { eprintln!("Error removing directory"); continue },
                            }},
                        _ => { eprintln!("Error moving file"); continue}
                    }
                },
                _ => { eprintln!("Error creating directory"); continue }
            }
        }
    }
}
