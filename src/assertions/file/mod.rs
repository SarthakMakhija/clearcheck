use std::fmt::Debug;
use std::path::Path;

use crate::matchers::file::{
    be_a_directory, be_a_file, be_a_symbolic_link, be_absolute, be_readonly, be_relative,
    be_writable, be_zero_sized, contain_all_file_names, contain_any_file_names, contain_file_name,
    have_extension,
};
use crate::matchers::{Should, ShouldNot};

pub trait FileAssertions {
    fn should_be_a_directory(&self) -> &Self;
    fn should_be_a_file(&self) -> &Self;
    fn should_be_a_symbolic_link(&self) -> &Self;

    fn should_be_zero_sized(&self) -> &Self;
    fn should_not_be_zero_sized(&self) -> &Self;

    fn should_be_readonly(&self) -> &Self;
    fn should_be_writable(&self) -> &Self;

    fn should_be_absolute(&self) -> &Self;
    fn should_be_relative(&self) -> &Self;

    fn should_have_extension(&self, extension: &str) -> &Self;
    fn should_not_have_extension(&self, extension: &str) -> &Self;

    fn should_contain_file_name(&self, name: &str) -> &Self;
    fn should_not_contain_file_name(&self, name: &str) -> &Self;

    fn should_contain_all_file_names(&self, names: &[&str]) -> &Self;
    fn should_not_contain_all_file_names(&self, names: &[&str]) -> &Self;

    fn should_contain_any_of_file_names(&self, names: &[&str]) -> &Self;
    fn should_not_contain_any_of_file_names(&self, names: &[&str]) -> &Self;
}

impl<T: AsRef<Path> + Debug> FileAssertions for T {
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

    fn should_have_extension(&self, extension: &str) -> &Self {
        self.should(&have_extension(extension));
        self
    }

    fn should_not_have_extension(&self, extension: &str) -> &Self {
        self.should_not(&have_extension(extension));
        self
    }

    fn should_contain_file_name(&self, name: &str) -> &Self {
        self.should(&contain_file_name(name));
        self
    }

    fn should_not_contain_file_name(&self, name: &str) -> &Self {
        self.should_not(&contain_file_name(name));
        self
    }

    fn should_contain_all_file_names(&self, names: &[&str]) -> &Self {
        self.should(&contain_all_file_names(names));
        self
    }

    fn should_not_contain_all_file_names(&self, names: &[&str]) -> &Self {
        self.should_not(&contain_all_file_names(names));
        self
    }

    fn should_contain_any_of_file_names(&self, names: &[&str]) -> &Self {
        self.should(&contain_any_file_names(names));
        self
    }

    fn should_not_contain_any_of_file_names(&self, names: &[&str]) -> &Self {
        self.should_not(&contain_any_file_names(names));
        self
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use tempdir::TempDir;

    use crate::assertions::file::FileAssertions;

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

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_file_name("junit.txt");
    }

    #[test]
    #[should_panic]
    fn should_contain_a_file_but_it_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path = temporary_directory.path().join("junit.txt");

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_file_name("assert4rs.txt");
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

        let _ = File::create(file_path.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_file_name("junit.txt");
    }

    #[test]
    fn should_contain_all_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_all_file_names(&["junit.txt", "assert4rs.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_all_files_but_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_all_file_names(&["junit.txt", "gotest.txt"]);
    }

    #[test]
    fn should_not_contain_all_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_all_file_names(&["scalaunit.txt", "gotest.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_all_files_but_did() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_all_file_names(&["junit.txt", "assert4rs.txt"]);
    }

    #[test]
    fn should_contain_any_of_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_any_of_file_names(&["junit.txt", "gotest.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_contain_any_of_files_but_did_not() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_contain_any_of_file_names(&["scalaunit.txt", "gotest.txt"]);
    }

    #[test]
    fn should_not_contain_any_of_files() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_any_of_file_names(&["scalaunit.txt", "gotest.txt"]);
    }

    #[test]
    #[should_panic]
    fn should_not_contain_any_of_files_but_did() {
        let temporary_directory = TempDir::new(".").unwrap();
        let file_path1 = temporary_directory.path().join("junit.txt");
        let file_path2 = temporary_directory.path().join("assert4rs.txt");

        let _ = File::create(file_path1.clone()).unwrap();
        let _ = File::create(file_path2.clone()).unwrap();

        let directory_path = temporary_directory.path();
        directory_path.should_not_contain_any_of_file_names(&["junit.txt", "gotest.txt"]);
    }
}
