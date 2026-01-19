//  I believe all methods in this crate have worst case O(m * n) search complexity; m is size of regex, n
//  is size of string to search. Not ideal?
use regex::Regex;

use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust_find")]
struct Opt {
    /// one or more directories to search in
    #[structopt(short, long, parse(from_os_str))]
    dirs: Vec<PathBuf>,

    /// one or more regex patterns to match against
    #[structopt(short, long)]
    patterns: Vec<String>,

    /// write output to a file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// maximum return size of files
    #[structopt(short, long)]
    size: Option<u64>,

    /// specify whether only files should be returned
    #[structopt(short = "f", long)]
    only_files: bool,
}

struct MyFile {
    path: PathBuf,
    size: u64,
    is_file: bool,
}

impl MyFile {
    fn from_path(path: PathBuf) -> Result<Self, std::io::Error> {
        let meta = fs::metadata(&path)?;
        Ok(MyFile {
            path,
            size: meta.len(),
            is_file: meta.is_file(),
        })
    }
}

fn get_files(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .flat_map(|e| {
                    let path = e.path();
                    if path.is_dir() {
                        get_files(&path)
                    } else {
                        vec![path]
                    }
                })
                .collect()
        })
        // better alternatives to this try-catch?
        .unwrap_or_else(|_| {
            eprintln!("Err: {:?} is not readable", dir);
            vec![]
        })
}

fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);
    let regexes: Vec<Regex> = opt
        .patterns
        .iter()
        .filter_map(|p| {
            Regex::new(p)
                .map_err(|e| eprintln!("invalid regex '{}': {}", p, e))
                .ok()
        })
        .collect();

    let results: String = opt
        .dirs
        .iter()
        .flat_map(|d| get_files(d))
        .filter_map(|p| MyFile::from_path(p).ok())
        .filter(|f| {
            let name = f.path.to_string_lossy();
            let match_re = regexes.is_empty() || regexes.iter().any(|re| re.is_match(&name));
            let match_size = opt.size.map_or(true, |s| f.size >= s);
            let match_type = !opt.only_files || f.is_file;
            match_re && match_size && match_type
        })
        // redundant string_lossy call? not sure if there is a better way
        .map(|f| f.path.to_string_lossy().into_owned())
        // use a fold instead?
        // concise,"idiomatic Vec<_>", or just Vec<String> for clarity?
        .collect::<Vec<String>>()
        .join("\n");

    match opt.output {
        // use of expect?
        Some(path) => fs::write(path, results).expect("Unable to write output"),
        None => println!("{}", results),
    }
}
