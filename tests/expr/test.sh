#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/expr/ test.ju -o test_expr
./tests/expr/test_expr
print_test_end $?
