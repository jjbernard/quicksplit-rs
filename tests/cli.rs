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

    let DIR_correct = String::from("./tests/test_dir");
    let DIR_incorrect = String::from("/test/non_existent_dir");

    assert_eq!(verify_dir(DIR_correct), true);
    assert_eq!(verify_dir(DIR_incorrect), false);
}

#[test]
fn test_copy_files() {
    use quicksplit_rs::copy_files;
    use std::io::Error;

    let DIR_SOURCE = String::from("./test/test_dir_copy");
    let DIR_DESTINATION = String::from("./test/test_dir");

    let res = copy_files(DIR_SOURCE, DIR_DESTINATION);
    match res {
        Ok(r) => assert_eq!(r, ()),
        _ => panic!("something wrong"),
    }
}

#[test]
fn test_split_dir() {
    use quicksplit_rs::new_dir;
    let TEST_DIR_SPLIT = "test-dir-split";
    let split_dir = new_dir(String::from(TEST_DIR_SPLIT));

    assert_eq!(split_dir, String::from("test/dir/split/"));
}

// What are the different things we need to do and therefore to test?
// todo: 2/ split the input dir along the "-" character into a vec of strings
// todo: 3/ create an output dir that will take the previous strings and turn them to a
// directory structure, i.e. 2024-03 into 2024/03
// todo: 3-bis/ ensure that we don't override an already partially existing structure, i.e.
// if 2024/02 exists, we are not going to delete it.
// todo: 4/ move all files from the given input dir into the newly created directory structure,
// which means that 2024-03/test.txt will be into 2024/03/test.txt
// todo: 5/ refactor to use crate:: instead of quicksplit.rs
