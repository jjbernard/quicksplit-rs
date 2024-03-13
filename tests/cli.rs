use std::path::Path;

#[cfg(test)]
#[test]
fn split_test() {
    // Check that a given &str such as "abc-def-ghi" will be split into
    // "abc", "def", "ghi"
    use quicksplit_rs::split_str;
    let test_str1 = "abc-def-ghi-89012";
    let test_str2 = "gha-45er-%UI-$42";
    let test_str3 = "abcdefghi";

    let res1: Vec<&str> = split_str(test_str1);
    let res2: Vec<&str> = split_str(test_str2);
    let res3: Vec<&str> = split_str(test_str3);

    assert_eq!(res1, ["abc", "def", "ghi", "89012"]);
    assert_eq!(res2, ["gha", "45er", "%UI", "$42"]);
    assert_eq!(res3, ["abcdefghi"]);
}

#[test]
fn test_dir_exists() {
    use quicksplit_rs::verify_dir;

    let dir_correct = String::from("./tests/test_dir");
    let dir_incorrect = String::from("/test/non_existent_dir");

    assert_eq!(verify_dir(&dir_correct), true);
    assert_eq!(verify_dir(&dir_incorrect), false);
}

#[test]
fn test_move_files() {
    use quicksplit_rs::move_files;

    let dir_source = String::from("./tests/test_dir_copy");
    let dir_destination = String::from("./tests/test_dir");

    let res = move_files(&dir_source, &dir_destination);

    assert!(res.is_ok());
}

#[test]
fn test_split_dir() {
    use quicksplit_rs::new_dir_name;
    let test_dir_split = "test-dir-split";
    let split_dir = new_dir_name(&String::from(test_dir_split));

    assert_eq!(split_dir, String::from("test/dir/split/"));
}

#[test]
fn test_mk_dir() {
    use quicksplit_rs::mk_dir;

    let dir = String::from("./tests/test_new_dir");
    let res = mk_dir(&dir);

    assert!(res.is_ok());
    let path = Path::new(dir.as_str());
    assert!(path.exists());
}

#[test]
fn test_rm_dir() {
    use quicksplit_rs::rm_dir;

    let dir = String::from("./tests/test_delete_dir");

    let res = rm_dir(&dir);
    assert!(res.is_ok());
    assert!(!Path::new(dir.as_str()).exists());
}
