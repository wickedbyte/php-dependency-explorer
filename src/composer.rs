use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::ErrorKind::NotFound;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

pub trait VersionCommand {
    fn version(&self) -> Result<String, String>;
}

pub struct Composer {
    command: String,
}

impl Composer {
    pub fn new(command: Option<String>) -> Composer {
        Composer {
            command: command.unwrap_or(String::from("composer")),
        }
    }

    pub fn get_locked_dependencies(&self) -> String {
        let json = self
            .run(vec![
                "show",
                "--latest",
                "--locked",
                "--ignore-platform-reqs",
                "--format=json",
            ])
            .unwrap();

        println!("{}", json);

        json
    }

    fn run(&self, args: Vec<&str>) -> Result<String, String> {
        let mut command = Command::new("/bin/sh");
        command.arg("-c");
        command.arg(self.command.clone() + " " + &*args.join(" "));

        println!("{:?}", command);

        match command.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout).unwrap())
                } else {
                    Err(String::from_utf8(output.stderr).unwrap())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

impl VersionCommand for Composer {
    fn version(&self) -> Result<String, String> {
        self.run(vec!["--version"])
    }
}

#[derive(Debug)]
pub struct ComposerFile {
    pub path: PathBuf,
    pub hash: [u8; 32],
}

impl ComposerFile {
    pub fn from_path(path: &Path) -> Result<ComposerFile, io::Error> {
        if !path.exists() {
            return Err(io::Error::new(
                NotFound,
                format!("File '{}' does not exist.", path.display()),
            ));
        }

        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 4096];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // End of file
            }
            hasher.update(&buffer[..bytes_read]);
        }

        let hash: [u8; 32] = hasher.finalize().into();

        Ok(ComposerFile {
            path: path.to_path_buf(),
            hash,
        })
    }
}

pub struct ComposerFiles {
    pub json: ComposerFile,
    pub lock: ComposerFile,
}

impl ComposerFiles {
    pub fn from_path(path: &Path) -> Result<ComposerFiles, io::Error> {
        let json_path = path.join("composer.json");
        let lock_path = path.join("composer.lock");
        println!("JSON: {:?} LOCK: {:?}", json_path, lock_path);

        Ok(ComposerFiles {
            json: ComposerFile::from_path(json_path.as_path())?,
            lock: ComposerFile::from_path(lock_path.as_path())?,
        })
    }
}
