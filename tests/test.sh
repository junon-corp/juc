echo -e "\n<--------------->\n"
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

function print_test_end() {
    RETURNED_VALUE=$1
    if [[ $RETURNED_VALUE == "" ]]
    then
        RETURNED_VALUE="nothing"
    fi

    echo "Test" $(expr $I - 1) "returned" $RETURNED_VALUE "<---------------" 
}