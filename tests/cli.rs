#[test]
fn split_test() {
    // Check that a given &str such as "abc-def-ghi" will be split into
    // "abc", "def", "ghi"
    use quicksplit_rs::split_str;
    let test_str1 = "abc-def-ghi-89012";
    let test_str2 = "gha-45er-%UI-$42";

    let res1: Vec<&str> = split_str(test_str1);
    let res2: Vec<&str> = split_str(test_str2);

    assert_eq!(res1, ["abc", "def", "ghi", "89012"]);
    assert_eq!(res2, ["gha", "45er", "%UI", "$42"]);
}

#[test]
fn test_dir_exists() {
    use std::path::Path;
    todo!()
}

// What are the different things we need to do and therefore to test?
// todo: 1/ read an input dir and check that it exists
// todo: 2/ split the input dir along the "-" character into a vec of strings
// todo: 3/ create an output dir that will take the previous strings and turn them to a
// directory structure, i.e. 2024-03 into 2024/03
// todo: 3-bis/ ensure that we don't override an already partially existing structure, i.e.
// if 2024/02 exists, we are not going to delete it.
// todo: 4/ move all files from the given input dir into the newly created directory structure,
// which means that 2024-03/test.txt will be into 2024/03/test.txt
// todo: 5/ refactor to use crate:: instead of quicksplit.rs