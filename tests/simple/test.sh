#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/simple/ test.ju -o test_simple
./tests/simple/test_simple
print_test_end $?
