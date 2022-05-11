#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/main_func/ test.ju -o test_main
./tests/compilation/main_func/test_main
print_test_end $?
