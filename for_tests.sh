#!/usr/bin/env bash

echo "Creating the test structure"
TEST_DIR=tests/test_dir
TEST_DIR_COPY=tests/test_dir_copy
TEST_DIR_SPLIT=tests/test-dir-split
mkdir $TEST_DIR
mkdir $TEST_DIR_COPY
mkdir $TEST_DIR_SPLIT

FILES=("01.txt" "02.txt" "03.txt" "abc.txt")

echo "Creating files for tests"
for value in "${FILES[@]}"
do
  touch $TEST_DIR_COPY/"$value"
done

echo "Done"
