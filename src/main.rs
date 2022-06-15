use std::{
    ffi::OsString,
    path::PathBuf,
};

use clap::{builder::TypedValueParser, Parser};

#[derive(Parser, Debug)]
#[clap(version, about = "command line helper to work with file extensions", long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Clone, Debug)]
struct ExtensionParser;

impl TypedValueParser for ExtensionParser {
    type Value = OsString;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_string_lossy().to_string();
        if value.starts_with(".") {
            Ok(value.replacen(".", "", 1).into())
        } else {
            Ok(value.into())
        }
    }
}

#[derive(Parser, Debug)]
enum Action {
    /// Adds an extension when it's missing or removes it when it's present.
    Toggle {
        /// Extension to be toggled.
        #[clap(value_parser = ExtensionParser)]
        extension: OsString,

        /// List of files to change.
        #[clap(value_parser, required = true)]
        files: Vec<PathBuf>,
    },
    /// Toggles between two extensions.
    ToggleBetween {
        /// Extension 1.
        #[clap(value_parser = ExtensionParser)]
        extension1: OsString,

        /// Extension 2.
        #[clap(value_parser = ExtensionParser)]
        extension2: OsString,

        /// List of files to change.
        #[clap(value_parser)]
        files: Vec<PathBuf>,
    },
    /// Replaces the extension with the given one.
    Set {
        /// Extension to be toggled.
        #[clap(value_parser = ExtensionParser)]
        extension: OsString,

        /// List of files to change.
        #[clap(value_parser, required = true)]
        files: Vec<PathBuf>,
    },
    /// Adds an extension to all found files.
    Add {
        /// add extension even if the file already has the same extension.
        #[clap(short, long, action)]
        force: bool,

        /// The extension to add to a file.
        #[clap(value_parser = ExtensionParser)]
        extension: OsString,

        /// List of files to change.
        #[clap(value_parser, required = true)]
        files: Vec<PathBuf>,
    },
    /// Removes an extension from all found files.
    Remove {
        /// The extension to be removed from a file. Removes any extension if not set.
        #[clap(value_parser = ExtensionParser)]
        extension: OsString,

        /// List of files to change.
        #[clap(value_parser, required = true)]
        files: Vec<PathBuf>,
    },
}

fn is_file(pb: &&PathBuf) -> bool {
    pb.is_file()
}

fn get_files(files: &[PathBuf]) -> Vec<&PathBuf> {
    files.into_iter().filter(is_file).collect()
}

fn append_extension(path: &PathBuf, extension: &OsString, force: bool) -> PathBuf {
    let new_name = match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            if force || &ext != extension {
                ext.push(".");
                ext.push(&extension);
            }
            path.with_extension(ext)
        }
        None => path.with_extension(&extension),
    };
    new_name
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.action {
        Action::ToggleBetween {
            files,
            extension1,
            extension2,
        } => {
            let paths = get_files(&files);
            for path in paths {
                if let Some(ext) = path.extension() {
                    if extension1 == ext {
                        std::fs::rename(&path, path.with_extension(&extension2))?
                    } else if extension2 == ext {
                        std::fs::rename(&path, path.with_extension(&extension1))?
                    }
                }
            }
        }
        Action::Toggle { files, extension } => {
            let mut new_files = Vec::with_capacity(files.len() * 2);
            for file in files {
                new_files.push(file.clone());
                new_files.push(append_extension(&file, &extension, false));
            }
            let paths = get_files(&new_files);
            for path in paths {
                if let Some(ext) = path.extension() {
                    if extension == ext {
                        if let Some(new_path) = path.file_stem() {
                            std::fs::rename(&path, path.with_file_name(new_path))?
                        }
                    } else {
                        let new_name = append_extension(path, &extension, false);
                        std::fs::rename(&path, new_name)?
                    }
                }
            }
        }
        Action::Add {
            files,
            extension,
            force,
        } => {
            let paths = get_files(&files);
            for path in paths {
                let new_name = append_extension(path, &extension, force);
                std::fs::rename(&path, new_name)?
            }
        }
        Action::Set { files, extension } => {
            let paths = get_files(&files);
            for path in paths {
                std::fs::rename(&path, path.with_extension(&extension))?;
            }
        }
        Action::Remove { files, extension } => {
            let paths = get_files(&files);
            for path in paths {
                if extension.is_empty() {
                    if let Some(new_path) = path.file_stem() {
                        std::fs::rename(&path, path.with_file_name(new_path))?
                    }
                } else {
                    if let Some(ext) = path.extension() {
                        if extension == ext {
                            if let Some(new_path) = path.file_stem() {
                                std::fs::rename(&path, path.with_file_name(new_path))?
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
