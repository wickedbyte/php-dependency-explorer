use std::path::PathBuf;
use std::process::Command;

pub trait VersionCommand {
    fn version(&self) -> Result<String, String>;
}

pub fn run(command: &mut Command) -> Result<String, String> {
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


pub struct Php {
    path: PathBuf,
}

impl Php {
    pub fn new(path: Option<String>) -> Php {
        let path = match path {
            Some(path) => PathBuf::from(path),
            _ => PathBuf::from("php"),
        };

        Php { path }
    }
}

impl VersionCommand for Php {
    fn version(&self) -> Result<String, String> {
        run(&mut Command::new(&self.path).arg("--version"))
    }
}

pub struct Composer {
    path: PathBuf,
}

impl Composer {
    pub fn new(path: Option<String>) -> Composer {
        let path = match path {
            Some(path) => PathBuf::from(path),
            _ => PathBuf::from("composer"),
        };

        Composer { path }
    }

    pub fn get_locked_dependencies(&self) -> String {
        let json = run(&mut Command::new(&self.path)
            .arg("show")
            .arg("--latest")
            .arg("--locked")
            .arg("--ignore-platform-reqs")
            .arg("--format=json")
        ).unwrap();

        println!("{}", json);

        json
    }
}

impl VersionCommand for Composer {
    fn version(&self) -> Result<String, String> {
        run(&mut Command::new(&self.path).arg("--version"))
    }
}

