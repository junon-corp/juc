# juc
Compiler for the Junon language. Multi-platform and modern design. Only available on Linux for the moment

## Hello world
```junon
fun main {
    print 'hello world\n'
    ret 0
}
```
Need more things ? While you wait for the documentation, you can [contact me](mailto:antonherault@gmail.com) :)

## Usage
/!\ Currently under development, some argument parameters could be added/removed
often. 

Show current parameters by running : `cargo run -- -h`

## Run tests
All tests are located in the "tests/" folder.
```
./tests/<test_name>/test.sh
```
Where **test_name** is the thing you want to test (args, calls, funcs, ...)

## Linked projects
- jup | [repository](https://github.com/junon-corp/jup)
- rslog | [repository](https://github.com/antoninhrlt/rslog)
- x64asm | [repository](https://github.com/antoninhrlt/x64asm)
  