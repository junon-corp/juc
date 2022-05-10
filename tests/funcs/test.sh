#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/funcs/ test.ju -o test_funcs -l
./tests/funcs/test_funcs
print_test_end $?
