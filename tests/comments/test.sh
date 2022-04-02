#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/comments/ test.ju -o test_comments
