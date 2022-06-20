set -e

# Usage :
#   tests/test.sh <test name> ?asm
# 
# Options :
#   - asm : Print the generated assembly code

# When $1 is the test name
function run_test() {
    echo -e "\x1b[1mTest '$1' started.\x1b[0m\n---"
    build/debug/juc test.ju -o test_$1 -d tests/$1
    
    if [[ $2 = "asm" ]]
    then
        echo -e "---\n\x1b[1mTest '$1' asm output.\x1b[0m\n---"
        cat tests/$1/.junon/test.ju.asm
    fi

    ./tests/$1/test_$1
    echo -e "\x1b[1m---\nTest '$1' done.\x1b[0m"
}

# No input file given
if [[ $1 = "" ]]
then
    echo "Test name required in command line arguments"
    exit 1
fi

run_test $1 $2
