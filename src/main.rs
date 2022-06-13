use std::path::PathBuf;

use clap::{builder::TypedValueParser, Parser};
use glob::GlobError;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Clone, Debug)]
struct ExtensionParser;

impl TypedValueParser for ExtensionParser {
    type Value = String;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_string_lossy().to_string();
        if value.starts_with(".") {
            Ok(value.replacen(".", "", 1))
        } else {
            Ok(value)
        }
    }
}

#[derive(Parser, Debug)]
enum Action {
    /// Adds an extension when it's missing or removes it when it's present.
    Toggle {
        /// Extension to be toggled.
        #[clap(value_parser)]
        extension: String,

        /// Glob pattern to filter files.
        #[clap(value_parser = ExtensionParser)]
        glob: String,
    },
    /// Toggles between two extensions.
    ToggleBetween {
        /// Extension 1.
        #[clap(value_parser)]
        extension1: String,

        /// Extension 2.
        #[clap(value_parser)]
        extension2: String,

        /// Optional glob pattern to filter files.
        #[clap(value_parser = ExtensionParser)]
        glob: Option<String>,
    },
    /// Adds an extension to all found files.
    Add {
        /// The extension to add to a file.
        #[clap(value_parser = ExtensionParser)]
        extension: String,

        /// Glob pattern to search for files.
        #[clap(value_parser)]
        glob: String,
    },
    /// Removes an extension from all found files.
    Remove {
        /// The extension to be removed from a file. Removes any extension if not set.
        #[clap(value_parser = ExtensionParser)]
        extension: Option<String>,

        /// Glob pattern to search for files.
        #[clap(value_parser)]
        glob: String,
    },
}

fn is_file(e: &Result<PathBuf, GlobError>) -> bool {
    e.iter().all(|f| f.is_file())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.action {
        Action::ToggleBetween {
            glob,
            extension1,
            extension2,
        } => {
            let files = glob::glob(&glob.unwrap_or("*".to_string()))?.filter(is_file);
            for f in files {
                match f {
                    Ok(path) => {
                        if let Some(ext) = path.extension() {
                            if extension1 == ext.to_string_lossy().to_string() {
                                std::fs::rename(&path, path.with_extension(&extension2))?
                            } else if extension2 == ext.to_string_lossy().to_string() {
                                std::fs::rename(&path, path.with_extension(&extension1))?
                            }
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Action::Toggle { glob, extension } => {
            let files = glob::glob(&glob)?.collect::<Vec<_>>();
            let files2 = glob::glob(&format!("{}.{}", glob, extension))?.collect::<Vec<_>>();

            for f in files.into_iter().chain(files2.into_iter()).filter(is_file) {
                match f {
                    Ok(path) => {
                        if let Some(ext) = path.extension() {
                            if extension == ext.to_string_lossy().to_string() {
                                if let Some(new_path) = path.file_stem() {
                                    std::fs::rename(&path, path.with_file_name(new_path))?
                                }
                            } else {
                                let new_name = match path.extension() {
                                    Some(ext) => {
                                        let mut ext = ext.to_os_string();
                                        ext.push(".");
                                        ext.push(&extension);
                                        path.with_extension(ext)
                                    }
                                    None => path.with_extension(&extension),
                                };
                                std::fs::rename(&path, new_name)?
                            }
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Action::Add { glob, extension } => {
            let files = glob::glob(&glob)?.filter(is_file);
            for f in files {
                match f {
                    Ok(path) => std::fs::rename(&path, path.with_extension(&extension))?,
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Action::Remove { glob, extension } => {
            let files = glob::glob(&glob)?.filter(is_file);
            for f in files {
                match (f, &extension) {
                    (Ok(path), Some(extension)) => {
                        if let Some(ext) = path.extension() {
                            if extension == &ext.to_string_lossy().to_string() {
                                if let Some(new_path) = path.file_stem() {
                                    std::fs::rename(&path, path.with_file_name(new_path))?
                                }
                            }
                        }
                    }
                    (Ok(path), None) => {
                        if let Some(new_path) = path.file_stem() {
                            std::fs::rename(&path, path.with_file_name(new_path))?
                        }
                    }
                    (Err(e), _) => println!("{:?}", e),
                }
            }
        }
    }

    Ok(())
}
