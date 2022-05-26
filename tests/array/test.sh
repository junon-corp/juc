#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/array/ test.ju -o test_array
./tests/array/test_array
print_test_end $?
