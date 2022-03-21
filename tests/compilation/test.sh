#!/bin/bash

source tests/test.sh

print_test_name
$BIN -d tests/compilation main.ju

