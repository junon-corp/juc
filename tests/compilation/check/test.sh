#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/compilation/check/ test.ju test2.ju -o test_check
print_test_end $?

print_test_name
$BIN -d tests/compilation/check/ test2.ju  -o test_check2
print_test_end $?
