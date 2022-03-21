#!/bin/bash

source tests/test.sh

print_test_name
$BIN

print_test_name
$BIN -h

print_test_name
$BIN -z

print_test_name
$BIN -d khazmfla/

print_test_name
$BIN -p AzR

# these commands should not fail
print_test_name
$BIN -p Windows

print_test_name
$BIN -d tests/args/

print_test_name
$BIN tests/args/file.ju
