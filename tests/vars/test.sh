#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/compilation/vars/ test.ju -o test_vars
./tests/compilation/vars/test_vars
print_test_end $?
