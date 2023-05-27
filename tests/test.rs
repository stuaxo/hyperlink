use hyperlink::{generate_file_path};
use rstest::rstest;
use std::path::Path;
use temp_env;

#[rstest(
    filepath,
    shortened,
    expected,
    case("/home/user/test.txt", true, "~/test.txt"),
    case("/home/user/test.txt", false, "/home/user/test.txt")
)]
fn test_generate_file_path_from_absolute(filepath: &str, shortened: bool, expected: &str) {
    let filepath = Path::new(filepath);

    // Since we are expanding tilde, HOME needs to be a known value
    temp_env::with_var("HOME", Some("/home/user"), || {
        let generated_path = generate_file_path(&filepath, None, shortened);

        assert_eq!(generated_path, expected);
    });
}

#[rstest(
    filepath,
    relative_to,
    shortened,
    expected,
    case("/home/user/test.txt", "/home/user", false, "test.txt"),
    case("/home/user/test.txt", "/home", false, "user/test.txt"),
    case("/home/user/test.txt", "/home", true, "user/test.txt")
)]
fn test_generate_file_path_from_relative(
    filepath: &str,
    relative_to: &str,
    shortened: bool,
    expected: &str,
) {
    // Verify that relative_to works - in this case shortened should not affect the output
    let filepath = Path::new(filepath);
    let relative_path = Some(Path::new(relative_to));
    // HOME needs to be a known value to test tilde expansion
    temp_env::with_var("HOME", Some("/home/user"), || {
        let generated_path = generate_file_path(filepath, relative_path, shortened);

        assert_eq!(generated_path, expected);
    });
}
