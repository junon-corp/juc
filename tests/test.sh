if ! cargo build
then
    exit 1
fi

BIN=build/debug/juc
I=1

function print_test_name() {
    echo "---------------> Test" $I
    ((I++))
}
