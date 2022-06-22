set -e

# Usage : tests/test.sh <test name>
# Where <test name> is a Junon file or a folder containing Junon files

cd tests/

# Test name not given
if [[ $1 = "" ]]
then
    exit 1
fi

# Given test name matches with a folder, all Junon files in that folder will be
# compiled
if [ -d $1 ]
then
    for i in $(find $1 -name '*.ju')
    do
        if [[ $SRC = "" ]]
        then
            SRC=$i
        else
            SRC="$SRC $i"
        fi
    done
# It's not a folder, the given test name is simply a Junon file
else
    SRC=$1.ju
fi

# Build
echo -e "$0 : cargo run -- $SRC -o bin/test_$1"
export RUST_BACKTRACE=1
cargo run -- $SRC -o bin/test_$1

# Run
./bin/test_$1

