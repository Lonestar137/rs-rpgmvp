use std::error::Error;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{ArgGroup, Parser};
use glob::glob;
use hex;

const DEFAULT_HEADER_LEN: usize = 16;
const DEFAULT_SIGNATURE: &str = "5250474d56000000";
// Unused now, but may be relevant in the future.
// const PNG_HEADER: &str = "89504E470D0A1A0A0000000D49484452";
// const DEFAULT_VERSION: &str = "000301";
// const DEFAULT_REMAIN: &str = "0000000000";

/// CLI tool for decrypting RPGMVP files.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
#[command(group = ArgGroup::new("input").required(true).args(&["basepath", "files"]))]
struct CommandlineArgs {
    /// Directory to glob rpgmvp files from.
    #[arg(short, long, group = "input")]
    basepath: Option<PathBuf>,

    /// Takes a list of files to decrypt.
    #[arg(short, long, group = "input")]
    files: Option<Vec<PathBuf>>,

    /// File extension of files to decrypt.
    #[arg(short, long, default_value = "rpgmvp")]
    extension: String,

    /// Folder to output processed files into.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Key used to decrypt files. Note: Found in www/data/System.json
    #[arg(short, long)]
    decryption_key: String,
}

#[derive(Debug)]
struct FileSystemException(String);

impl fmt::Display for FileSystemException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for FileSystemException {}

struct Decryptor {
    decrypt_code: Vec<u8>,
    ignore_fake_header: bool,
}

impl Decryptor {
    fn new(decryption_code: Vec<u8>) -> Self {
        Self {
            decrypt_code: decryption_code,
            ignore_fake_header: false,
        }
    }

    fn get_decrypt_code(&self) -> &Vec<u8> {
        self.decrypt_code.as_ref()
    }

    fn get_header_len(&self) -> usize {
        DEFAULT_HEADER_LEN
    }

    fn is_ignore_fake_header(&self) -> bool {
        self.ignore_fake_header
    }

    fn check_fake_header(&self, content: &[u8]) -> bool {
        let signature = hex::decode(DEFAULT_SIGNATURE).unwrap();
        content.starts_with(&signature)
    }

    fn decrypt_file(&self, file_path: &Path, out_path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(file_path)?;

        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        if content.len() < self.get_header_len() * 2 {
            return Err(Box::new(FileSystemException(format!(
                "File is too short (< {} Bytes)",
                self.get_header_len() * 2
            ))));
        }

        if !self.is_ignore_fake_header() && !self.check_fake_header(&content) {
            return Err(Box::new(FileSystemException("Header is Invalid!".into())));
        }

        let mut content = content[self.get_header_len()..].to_vec();

        let decrypt_code = self.get_decrypt_code();
        if !content.is_empty() {
            for i in 0..self.get_header_len() {
                content[i] ^= decrypt_code[i];
            }
        }

        create_dir_all(out_path.parent().unwrap())?;
        let mut file = File::create(out_path)?;
        file.write_all(&content)?;

        Ok(())
    }
}

fn decrypt_files(decryption_key: String, files: Vec<PathBuf>, args: &CommandlineArgs) {
    let system_key = hex::decode(decryption_key).expect("Invalid decryption key.");
    let decryptor = Decryptor::new(system_key);

    for file in files.iter() {
        let mut outpath = file.clone();
        outpath.set_extension("png");
        if let Some(output_path) = &args.output {
            outpath = output_path.join(outpath.clone());
        }

        match decryptor.decrypt_file(&file, &outpath) {
            Ok(_) => println!("{:?}", outpath),
            Err(e) => eprintln!("Failed to decrypt file: {}", e),
        }
    }
}

fn get_files_with_extension(
    basepath: &PathBuf,
    extension: &str,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let pattern = format!("{}/**/*.{}", basepath.display(), extension);
    let mut files = Vec::new();

    for entry in glob(&pattern)? {
        match entry {
            Ok(path) => files.push(path),
            Err(e) => eprintln!("Error reading file: {:?}", e),
        }
    }

    Ok(files)
}

fn main() {
    let args = CommandlineArgs::parse();
    let cloned_args = args.clone(); // Cloning the arguments

    if let Some(files) = args.files {
        decrypt_files(args.decryption_key.clone(), files, &cloned_args);
    }

    if let Some(basepath) = args.basepath {
        let files_result = get_files_with_extension(&basepath, &args.extension);

        match files_result {
            Ok(files) => {
                decrypt_files(args.decryption_key.clone(), files, &cloned_args);
            }
            Err(error) => println!(
                "No files found at basepath: {:?}\n{:?}",
                cloned_args.basepath, error
            ),
        }
    }
}
