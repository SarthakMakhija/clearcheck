use std::fmt::Debug;
use std::path::Path;

use crate::matchers::{Should, ShouldNot};
use crate::matchers::file::{
    be_a_directory, be_a_file, be_a_symbolic_link, be_absolute, be_readonly, be_relative,
    be_writable, be_zero_sized, contain_all_file_names, contain_any_file_names, contain_file_name,
    have_extension,
};

/// FileAssertion enables assertions about various properties of file or path.
///
///
/// It offers a fluent interface for chaining multiple assertions.
///
/// # Example
/// ```
/// use std::fs::File;
/// use tempdir::TempDir;
/// use clearcheck::assertions::file::FileAssertion;
///
/// let temporary_directory = TempDir::new(".").unwrap();
/// let file_path_junit = temporary_directory.path().join("junit.txt");
/// let file_path_clearcheck = temporary_directory.path().join("clearcheck.txt");
///
/// let _ = File::create(file_path_junit).unwrap();
/// let _ = File::create(file_path_clearcheck).unwrap();
///
/// let directory_path = temporary_directory.path();
/// directory_path
///     .should_be_a_directory()
///     .should_contain_any_of_file_names(vec!["junit.txt", "clearcheck.txt"]);
/// ```
pub trait FileAssertion {
    /// - Asserts that the path is a directory.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_a_directory(&self) -> &Self;

    /// - Asserts that the path is a file.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_a_file(&self) -> &Self;

    /// - Asserts that the path is a symbolic link.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_a_symbolic_link(&self) -> &Self;

    /// - Asserts that the path corresponds to a zero sized file.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_zero_sized(&self) -> &Self;

    /// - Asserts that the path corresponds to a non-zero sized file.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_not_be_zero_sized(&self) -> &Self;

    /// - Asserts that the path corresponds to a readonly file.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_readonly(&self) -> &Self;

    /// - Asserts that the path corresponds to a writable file.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_writable(&self) -> &Self;

    /// - Asserts that the path is absolute.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_absolute(&self) -> &Self;

    /// - Asserts that the path is relative.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_be_relative(&self) -> &Self;

    /// - Asserts that the path corresponds to a file with the specified extension.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_have_extension(&self, extension: &'static str) -> &Self;

    /// - Asserts that the path corresponds to a file that does not have the specified extension.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_not_have_extension(&self, extension: &'static str) -> &Self;

    /// - Asserts that the path corresponds to a directory that contains the specified file name.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_contain_file_name(&self, name: &'static str) -> &Self;

    /// - Asserts that the path corresponds to a directory that does not contain the specified file name.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_not_contain_file_name(&self, name: &'static str) -> &Self;

    /// - Asserts that the path corresponds to a directory that contains all the specified file names.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_contain_all_file_names(&self, names: Vec<&'static str>) -> &Self;

    /// - Asserts that the path corresponds to a directory that does not contain all the specified file names.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_not_contain_all_file_names(&self, names: Vec<&'static str>) -> &Self;

    /// - Asserts that the path corresponds to a directory that contains any of the specified file names.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_contain_any_of_file_names(&self, names: Vec<&'static str>) -> &Self;

    /// - Asserts that the path corresponds to a directory that does not contain any of the specified file names.
    /// - Returns a reference to self for fluent chaining.
    /// - Panics if the assertion fails.
    fn should_not_contain_any_of_file_names(&self, names: Vec<&'static str>) -> &Self;
}

impl<T: AsRef<Path> + Debug> FileAssertion for T {
    fn should_be_a_directory(&self) -> &Self {
        self.should(&be_a_directory());
        self
    }

    fn should_be_a_file(&self) -> &Self {
        self.should(&be_a_file());
        self
    }

    fn should_be_a_symbolic_link(&self) -> &Self {
        self.should(&be_a_symbolic_link());
        self
    }

    fn should_be_zero_sized(&self) -> &Self {
        self.should(&be_zero_sized());
        self
    }

    fn should_not_be_zero_sized(&self) -> &Self {
        self.should_not(&be_zero_sized());
        self
    }

    fn should_be_readonly(&self) -> &Self {
        self.should(&be_readonly());
        self
    }

    fn should_be_writable(&self) -> &Self {
        self.should(&be_writable());
        self
    }

    fn should_be_absolute(&self) -> &Self {
        self.should(&be_absolute());
        self
    }

    fn should_be_relative(&self) -> &Self {
        self.should(&be_relative());
        self
    }

    fn should_have_extension(&self, extension: &'static str) -> &Self {
        self.should(&have_extension(extension));
        self
    }

    fn should_not_have_extension(&self, extension: &'static str) -> &Self {
        self.should_not(&have_extension(extension));
        self
    }

    fn should_contain_file_name(&self, name: &'static str) -> &Self {
        self.should(&contain_file_name(name));
        self
    }

    fn should_not_contain_file_name(&self, name: &'static str) -> &Self {
        self.should_not(&contain_file_name(name));
        self
    }

    fn should_contain_all_file_names(&self, names: Vec<&'static str>) -> &Self {
        self.should(&contain_all_file_names(names));
        self
    }

    fn should_not_contain_all_file_names(&self, names: Vec<&'static str>) -> &Self {
        self.should_not(&contain_all_file_names(names));
        self
    }

    fn should_contain_any_of_file_names(&self, names: Vec<&'static str>) -> &Self {
        self.should(&contain_any_file_names(names));
        self
    }

    fn should_not_contain_any_of_file_names(&self, names: Vec<&'static str>) -> &Self {
        self.should_not(&contain_any_file_names(names));
        self
    }
}

#[cfg(all(test, feature = "file"))]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use tempdir::TempDir;

    use crate::assertions::file::FileAssertion;

    #[test]
    fn should_be_a_directory() {
        let temporary_directory = TempDir::new(".").unwrap();
        temporary_directory.should_be_a_directory();
    }

    #[test]
    fn should_be_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        path.should_be_a_file();
    }

    #[test]
    fn should_be_zero_sized() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        path.should_be_zero_sized();
    }

    #[test]
    #[should_panic]
    fn should_be_a_zero_sized_file_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let mut file = File::create(file_path.clone()).unwrap();
        writeln!(file, "test content").unwrap();

        path.should_be_zero_sized();
    }

    #[test]
    fn should_not_be_zero_sized() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let mut file = File::create(file_path.clone()).unwrap();
        writeln!(file, "test content").unwrap();

        path.should_not_be_zero_sized();
    }

    #[test]
    #[should_panic]
    fn should_not_be_a_zero_sized_file_but_was() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        path.should_not_be_zero_sized();
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

        path.should_be_readonly();
    }

    #[test]
    #[should_panic]
    fn should_be_readonly_but_was_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        path.should_be_readonly();
    }

    #[test]
    fn should_be_writable() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("temporary.txt");

        let path = file_path.as_path();
        let _ = File::create(file_path.clone()).unwrap();

        path.should_be_writable();
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

        path.should_be_writable();
    }

    #[test]
    fn should_be_absolute() {
        let path = Path::new("/etc/conf.d");
        path.should_be_absolute();
    }

    #[test]
    #[should_panic]
    fn should_be_absolute_but_was_not() {
        let path = Path::new(".");
        path.should_be_absolute();
    }

    #[test]
    fn should_be_relative() {
        let path = Path::new(".");
        path.should_be_relative();
    }

    #[test]
    #[should_panic]
    fn should_be_relative_but_was_not() {
        let path = Path::new("/etc/conf.d");
        path.should_be_relative();
    }

    #[test]
    fn should_have_extension() {
        let path = Path::new("/etc/sample.txt");
        path.should_have_extension("txt");
    }

    #[test]
    #[should_panic]
    fn should_have_extension_but_was_not() {
        let path = Path::new("/etc/sample.txt");
        path.should_have_extension("zip");
    }

    #[test]
    fn should_not_have_extension() {
        let path = Path::new("/etc/sample.txt");
        path.should_not_have_extension("zip");
    }

    #[test]
    #[should_panic]
    fn should_not_have_extension_but_was() {
        let path = Path::new("/etc/sample.zip");
        path.should_not_have_extension("zip");
    }

    #[test]
    fn should_contain_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_file_name("junit.txt");
    }

    #[test]
    #[should_panic]
    fn should_contain_a_file_but_it_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_file_name("clearcheck.txt");
    }

    #[test]
    fn should_not_contain_a_file() {
        let temporary_directory = TempDir::new(".").unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_file_name("junit.txt");
    }

    #[test]
    #[should_panic]
    fn should_not_contain_a_file_but_it_did() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_file_name("junit.txt");
    }

    #[test]
    fn should_contain_all_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_all_file_names(vec!["junit.txt", "clearcheck.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_files_but_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_all_file_names(vec!["junit.txt", "gotest.txt"]);
    }

    #[test]
    fn should_not_contain_all_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_all_file_names(vec!["scalaunit.txt", "gotest.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_files_but_did() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_all_file_names(vec!["junit.txt", "clearcheck.txt"]);
    }

    #[test]
    fn should_contain_any_of_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_any_of_file_names(vec!["junit.txt", "gotest.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_files_but_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_any_of_file_names(vec!["scalaunit.txt", "gotest.txt"]);
    }

    #[test]
    fn should_not_contain_any_of_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_any_of_file_names(vec!["scalaunit.txt", "gotest.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_of_files_but_did() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("clearcheck.txt");

        let _ = File::create(file_path1).unwrap();
        let _ = File::create(file_path2).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_any_of_file_names(vec!["junit.txt", "gotest.txt"]);
    }
}
