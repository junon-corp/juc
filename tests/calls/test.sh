#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/calls/ test.ju -o test_calls
./tests/calls/test_calls
print_test_end $?
