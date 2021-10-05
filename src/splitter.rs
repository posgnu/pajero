use crate::analyze::analyze;
use std::io;
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

const DEFAULT_BIN_PATH: &str = "./static/bin/PcapSplitter";

pub fn split_pcap(path: String, round: u8) -> Result<(), io::Error> {
    let pcap_path = path;
    // Create a directory inside of `std::env::temp_dir()`, named with the prefix "tmp_packets".
    let output_dir = TempDir::new("connections")?;
    let tmp_dir_path = match output_dir.path().to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => {
            // println!("Make tmp dir on {}", s);
            s
        }
    };

    // Execute PcapSplitter
    let output = Command::new(DEFAULT_BIN_PATH)
        .arg("-f")
        .arg(pcap_path)
        .arg("-o")
        .arg(tmp_dir_path)
        .arg("-m")
        .arg("connection")
        .output()
        .expect("failed to execute process");

    // Analyze each connection and store it separately
    analyze(tmp_dir_path.to_string(), round);

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `output_dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    output_dir.close()?;
    Ok(())
}
