extern crate git2;

use git2::{BlameOptions, Repository};
use std::path::Path;


fn blame() -> Result<(), git2::Error> {
    let repo = match Repository::open("/home/randy/chewse") {
        Ok(value) => {value},
        Err(_) => panic!("Couldn't open repo.")
    };
    let path = Path::new("chewse/media/js/dispatcher/components/order.js");

    let commit_range = "ce3e3044f76aa5c369cafb8d4aedc45a8d7b0102..6f7198a9ae78584c4d127a5d22c6d27b22208ff8";
    let revspec = repo.revparse(&commit_range)?;
    println!("{:?}", revspec.mode());

    let mut options = BlameOptions::new();
    if let (Some(oldest_commit), Some(newest_commit)) = (revspec.from(), revspec.to()) {
         options
            .oldest_commit(oldest_commit.id())
            .newest_commit(newest_commit.id());
    }
    let blame = match repo.blame_file(&path, None) {
        Ok(value) => value,
        Err(_) => panic!("Unable to blame path")
    };
    let num_lines = blame.len();
    println!("num lines: {}", num_lines);
    for index in 0..num_lines {
        if let Some(hunk) = blame.get_index(index) {
            println!(
                "{} - {} - {}", 
                index + 1,
                hunk.orig_commit_id(),
                hunk.orig_signature()
            );
        }
    } 
    Ok(())
}


fn main() {
    match blame() {
        Ok(v) => {},
        Err(r) => println!("error {}", r)
    }
}

