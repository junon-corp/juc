<img src="https://raw.githubusercontent.com/junon-corp/jur/main/assets/logo_circle.png" align="right" width="20%" alt="Junon logo" />

# juc
Compiler for the Junon language. Multi-platform and modern design. Only available on Linux for the moment

## First example
```junon
fun main {
    let a: int = 3
    let b: int = { 4 + 5 }
    ret
}

fun foo: int {
    ret { 6 / 2 }
}
```
Need more things ? While you wait for the documentation, you can [contact me](mailto:antonherault@gmail.com) :)

## Run tests
All tests are located in the "tests/" folder. \
To run one of them : `./tests/<test_name>/test.sh`, when you know *test_name*
