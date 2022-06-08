<img src="https://raw.githubusercontent.com/junon-corp/jur/main/assets/logo_circle.png" align="right" width="20%" alt="Junon logo" />

# juc
Compiler for the Junon language. Multi-platform and modern design. Only available on Linux for the moment

## Note
The compiler is still under development, and the main branch is not always stable. \
About the Rust code, it's often reviewed to optimize some parts and to better follow the Rust's guidelines. You can contribute on that checking by making a pull request with your changes.

## Simple example
```junon
fun main {
    let a: int = 3
    a = 5

    let b: int = { 4 + 5 }
    ret
}

fun foo: int {
    ret { 6 / 2 }
}
```
Need more things ? While you wait for the documentation, you can [contact me](mailto:antonherault@gmail.com) :)

Be careful, the source code is not yet checked by the compiler. Your code has to be perfect. 

## Documentation
The first Junon language's documentation pages are coming soon.

The [juc](https://github.com/junon-corp/juc) code comments for documentation could be wrong sometimes if a change forgot to update the comment with itself. If you see any documentation error, do not hesitate to open an issue. \
Before each stable version, the documentation will be checked again to avoid these kind of problems.

## Run tests
All tests are located in the "tests/" folder. \
To run one of them : `./tests/<test_name>/test.sh`, when you know *test_name*
