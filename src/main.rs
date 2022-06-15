use std::path::PathBuf;

use clap::{builder::TypedValueParser, Parser};
use glob::GlobError;

#[derive(Parser, Debug)]
#[clap(version, about = "command line helper to work with file extensions", long_about = None)]
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
        #[clap(value_parser = ExtensionParser)]
        extension: String,

        /// Glob patterns to filter files.
        #[clap(value_parser, required = true)]
        globs: Vec<String>,
    },
    /// Toggles between two extensions.
    ToggleBetween {
        /// Extension 1.
        #[clap(value_parser = ExtensionParser)]
        extension1: String,

        /// Extension 2.
        #[clap(value_parser = ExtensionParser)]
        extension2: String,

        /// Optional glob pattern to filter files.
        #[clap(value_parser)]
        globs: Vec<String>,
    },
    /// Replaces the extension with the given one.
    Set {
        /// Extension to be toggled.
        #[clap(value_parser = ExtensionParser)]
        extension: String,

        /// Glob patterns to filter files.
        #[clap(value_parser, required = true)]
        globs: Vec<String>,
    },
    /// Adds an extension to all found files.
    Add {
        /// The extension to add to a file.
        #[clap(value_parser = ExtensionParser)]
        extension: String,

        /// Glob pattern to search for files.
        #[clap(value_parser, required = true)]
        globs: Vec<String>,
    },
    /// Removes an extension from all found files.
    Remove {
        /// The extension to be removed from a file. Removes any extension if not set.
        #[clap(value_parser = ExtensionParser)]
        extension: Option<String>,

        /// Glob pattern to search for files.
        #[clap(value_parser)]
        globs: Vec<String>,
    },
}

fn is_file(e: &Result<PathBuf, GlobError>) -> bool {
    e.iter().all(|f| f.is_file())
}

fn get_files(globs: &[String]) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for glob in globs {
        let files = glob::glob(&glob)?.filter(is_file);
        for f in files {
            result.push(f?);
        }
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.action {
        Action::ToggleBetween {
            globs,
            extension1,
            extension2,
        } => {
            let globs = if globs.len() == 0 {
                vec!["*".to_owned()]
            } else {
                globs
            };
            let paths = get_files(&globs)?;
            for path in paths {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_string();
                    if extension1 == ext {
                        std::fs::rename(&path, path.with_extension(&extension2))?
                    } else if extension2 == ext {
                        std::fs::rename(&path, path.with_extension(&extension1))?
                    }
                }
            }
        }
        Action::Toggle { globs, extension } => {
            let mut new_globs = Vec::with_capacity(globs.len() * 2);
            for glob in &globs {
                new_globs.push(glob.clone());
                new_globs.push(format!("{}.{}", glob, extension));
            }
            let paths = get_files(&new_globs)?;
            for path in paths {
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
        }
        Action::Add { globs, extension } => {
            let paths = get_files(&globs)?;
            for path in paths {
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
        Action::Set { globs, extension } => {
            let paths = get_files(&globs)?;
            for path in paths {
                std::fs::rename(&path, path.with_extension(&extension))?;
            }
        }
        Action::Remove { globs, extension } => {
            let paths = get_files(&globs)?;
            for path in paths {
                match &extension {
                    Some(extension) => {
                        if let Some(ext) = path.extension() {
                            if extension == &ext.to_string_lossy().to_string() {
                                if let Some(new_path) = path.file_stem() {
                                    std::fs::rename(&path, path.with_file_name(new_path))?
                                }
                            }
                        }
                    }
                    None => {
                        if let Some(new_path) = path.file_stem() {
                            std::fs::rename(&path, path.with_file_name(new_path))?
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
