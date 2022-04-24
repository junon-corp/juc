#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/vars/ test.ju -o test_vars
./tests/vars/test_vars
print_test_end $?
