#!/bin/bash

if ! cargo build
then
    exit 1
fi

BIN=build/debug/juc
I=1

function print_test_name() {
    echo "--- Test" $I "---"
    ((I++))
}

print_test_name
$BIN

print_test_name
$BIN tests/args/file.ju

print_test_name
$BIN -h

print_test_name
$BIN -z
