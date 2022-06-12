#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/operators test.ju -o test_operators
./tests/operators/test_operators
print_test_end $?
