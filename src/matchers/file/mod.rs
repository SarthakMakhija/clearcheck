use std::collections::HashSet;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs;
use std::path::Path;

use walkdir::WalkDir;

use crate::matchers::{Matcher, MatcherResult};

pub enum FileTypeBased {
    File,
    Directory,
    SymbolicLink,
    ZeroSized,
    Readonly,
    Writable,
}

pub enum FilePathBased<'a> {
    Absolute,
    Relative,
    Extension(&'a str),
}

pub enum WalkTreeBased<'a> {
    Contain(&'a str),
    ContainAll(&'a [&'a str]),
    ContainAny(&'a [&'a str]),
}

impl<T: AsRef<Path> + Debug> Matcher<T> for FileTypeBased {
    fn test(&self, value: &T) -> MatcherResult {
        let metadata = fs::metadata(value);
        match &self {
            FileTypeBased::File => MatcherResult::formatted(
                value.as_ref().is_file(),
                format!("{:?} should be a file", value),
                format!("{:?} should not be a file", value),
            ),
            FileTypeBased::Directory => MatcherResult::formatted(
                value.as_ref().is_dir(),
                format!("{:?} should be a directory", value),
                format!("{:?} should not be a directory", value),
            ),
            FileTypeBased::SymbolicLink => MatcherResult::formatted(
                value.as_ref().is_symlink(),
                format!("{:?} should be a symbolic link", value),
                format!("{:?} should not be a symbolic link", value),
            ),
            FileTypeBased::ZeroSized => MatcherResult::formatted(
                metadata.is_ok_and(|metadata| metadata.len() == 0),
                format!("{:?} should be zero sized", value),
                format!("{:?} should not be zero sized", value),
            ),
            FileTypeBased::Readonly => MatcherResult::formatted(
                metadata.is_ok_and(|metadata| metadata.permissions().readonly()),
                format!("{:?} should be readonly", value),
                format!("{:?} should not be readonly", value),
            ),
            FileTypeBased::Writable => MatcherResult::formatted(
                metadata.is_ok_and(|metadata| !metadata.permissions().readonly()),
                format!("{:?} should be writable", value),
                format!("{:?} should not be writable", value),
            ),
        }
    }
}

impl<T: AsRef<Path> + Debug> Matcher<T> for FilePathBased<'_> {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            FilePathBased::Absolute => MatcherResult::formatted(
                value.as_ref().is_absolute(),
                format!("{:?} should be absolute", value),
                format!("{:?} should not be absolute", value),
            ),
            FilePathBased::Relative => MatcherResult::formatted(
                value.as_ref().is_relative(),
                format!("{:?} should be relative", value),
                format!("{:?} should not be relative", value),
            ),
            FilePathBased::Extension(extension) => MatcherResult::formatted(
                value
                    .as_ref()
                    .extension()
                    .filter(|source| source == extension)
                    .is_some(),
                format!("{:?} should have extension {:?}", value, extension),
                format!("{:?} should not be have extension {:?}", value, extension),
            ),
        }
    }
}

impl<T: AsRef<Path> + Debug> Matcher<T> for WalkTreeBased<'_> {
    fn test(&self, value: &T) -> MatcherResult {
        match self {
            WalkTreeBased::Contain(name) => {
                for directory_entry in WalkDir::new(value) {
                    if let Ok(entry) = directory_entry {
                        if &entry.file_name() == name {
                            return MatcherResult::formatted(
                                true,
                                format!("{:?} should contain a file name {:?}", value, name),
                                format!("{:?} should not contain a file name {:?}", value, name),
                            );
                        }
                    }
                }
                MatcherResult::formatted(
                    false,
                    format!("{:?} should contain a file name {:?}", value, name),
                    format!("{:?} should not contain a file name {:?}", value, name),
                )
            }
            WalkTreeBased::ContainAll(names) => {
                let mut unique_names = names
                    .iter()
                    .map(|name| OsStr::new(name))
                    .collect::<HashSet<_>>();

                for directory_entry in WalkDir::new(value) {
                    if let Ok(entry) = directory_entry {
                        if unique_names.contains(entry.file_name()) {
                            unique_names.remove(entry.file_name());
                        }
                    }
                }
                MatcherResult::formatted(
                    unique_names.len() == 0,
                    format!("{:?} should contain all file names {:?}", value, names),
                    format!("{:?} should not contain all file names {:?}", value, names),
                )
            }
            WalkTreeBased::ContainAny(names) => {
                let mut unique_names = names
                    .iter()
                    .map(|name| OsStr::new(name))
                    .collect::<HashSet<_>>();

                for directory_entry in WalkDir::new(value) {
                    if let Ok(entry) = directory_entry {
                        if unique_names.contains(entry.file_name()) {
                            unique_names.remove(entry.file_name());
                            break;
                        }
                    }
                }
                MatcherResult::formatted(
                    unique_names.len() != names.len(),
                    format!("{:?} should contain any of file names {:?}", value, names),
                    format!(
                        "{:?} should not contain any of file names {:?}",
                        value, names
                    ),
                )
            }
        }
    }
}

pub fn be_a_directory() -> FileTypeBased {
    FileTypeBased::Directory
}

pub fn be_a_file() -> FileTypeBased {
    FileTypeBased::File
}

pub fn be_a_symbolic_link() -> FileTypeBased {
    FileTypeBased::SymbolicLink
}

pub fn be_a_zero_sized_file() -> FileTypeBased {
    FileTypeBased::ZeroSized
}

pub fn be_readonly() -> FileTypeBased {
    FileTypeBased::Readonly
}

pub fn be_writable() -> FileTypeBased {
    FileTypeBased::Writable
}

pub fn be_absolute() -> FilePathBased<'static> {
    FilePathBased::Absolute
}

pub fn be_relative() -> FilePathBased<'static> {
    FilePathBased::Relative
}

pub fn have_extension(extension: &str) -> FilePathBased {
    FilePathBased::Extension(extension)
}

pub fn contain_file_name(name: &str) -> WalkTreeBased {
    WalkTreeBased::Contain(name)
}

pub fn contain_all_file_names<'a>(names: &'a [&'a str]) -> WalkTreeBased<'a> {
    WalkTreeBased::ContainAll(names)
}

pub fn contain_any_file_names<'a>(names: &'a [&'a str]) -> WalkTreeBased<'a> {
    WalkTreeBased::ContainAny(names)
}

#[cfg(test)]
mod file_type_tests {
    use std::fs::File;
    use std::io::Write;

    use tempdir::TempDir;

    use crate::assertions::bool::TrueFalse;
    use crate::matchers::file::{
        be_a_directory, be_a_file, be_a_zero_sized_file, be_readonly, be_writable,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_be_a_directory() {
        let temporary_directory = TempDir::new(".").unwrap();
        let matcher = be_a_directory();
        matcher.test(&temporary_directory).passed.should_be_true();
    }

    #[test]
    fn should_be_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_a_file();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_a_zero_sized_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_a_zero_sized_file();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_a_zero_sized_file_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let mut file = File::create(file_path.clone()).unwrap();
        writeln!(file, "test content").unwrap();

        let matcher = be_a_zero_sized_file();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_readonly() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let file = File::create(file_path.clone()).unwrap();

        let mut readonly_permission = file.metadata().unwrap().permissions();
        readonly_permission.set_readonly(true);

        file.set_permissions(readonly_permission).unwrap();

        let matcher = be_readonly();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_readonly_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_readonly();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_writable() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        let matcher = be_writable();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    #[should_panic]
    fn should_be_writable_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let file = File::create(file_path.clone()).unwrap();

        let mut readonly_permission = file.metadata().unwrap().permissions();
        readonly_permission.set_readonly(true);

        file.set_permissions(readonly_permission).unwrap();

        let matcher = be_writable();
        matcher.test(&path).passed.should_be_true();
    }
}

#[cfg(test)]
mod file_path_tests {
    use std::path::Path;

    use crate::assertions::bool::TrueFalse;
    use crate::matchers::file::{be_absolute, be_relative, have_extension};
    use crate::matchers::Matcher;

    #[test]
    fn should_be_absolute() {
        let path = Path::new("/etc/conf.d");
        let matcher = be_absolute();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_be_relative() {
        let path = Path::new(".");
        let matcher = be_relative();
        matcher.test(&path).passed.should_be_true();
    }

    #[test]
    fn should_have_extension() {
        let path = Path::new("/etc/sample.txt");
        let matcher = have_extension("txt");
        matcher.test(&path).passed.should_be_true();
    }
}

#[cfg(test)]
mod walk_tree_tests {
    use std::fs::File;

    use tempdir::TempDir;

    use crate::assertions::bool::TrueFalse;
    use crate::matchers::file::{
        contain_all_file_names, contain_any_file_names, contain_file_name,
    };
    use crate::matchers::Matcher;

    #[test]
    fn should_contain_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_file_name(&"junit.txt");
        matcher.test(&directory_path).passed.should_be_true();
    }

    #[test]
    fn should_contain_all_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_all_file_names(&["junit.txt", "assert4rs.txt"]);
        matcher.test(&directory_path).passed.should_be_true();
    }

    #[test]
    fn should_contain_any_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        let matcher = contain_any_file_names(&["junit.txt", "assert4rs.txt"]);
        matcher.test(&directory_path).passed.should_be_true();
    }
}
