<img src="https://raw.githubusercontent.com/junon-corp/jur/main/assets/logo_circle.png" align="right" width="20%" alt="Junon logo" />

# juc
Compiler for the Junon language. Multi-platform and modern design. Currently only available for Linux

## Example
This example shows the different things you can actually do with the current version.
```junon
fun main {
    let a: int = 1
    a = 2

    let b: int = { 3 + 4 }
    let c: int[5] = [1, 2, 3, 4, 5]

    ret
}

// Divides by 2 a number `a`
fun foo(a: int): int {
    ret { a / 2 }
}
```

## Note
The compiler is still under development, and the main branch is not stable. If you encounter a bug, please open an issue.

## Documentation
The language's documentation will be there soon. If you have any question, open an issue or [contact me](mailto:antonherault@gmail.com).

The compiler's code is documented. To generate the documentation pages, run `cargo doc --open` then your web browser will be opened on it. More comments are written in the code to help contributors understanding how it works but they are not showing up on the generated pages.

## Contributions
Run a test : `tests/run.sh <test name>`. Some tests do not work with the current compiler's version, because the language is evolving. \
The `<test name>` is a test file's name (without the extension) or a folder located in the "tests/" folder.

To purpose some changes on the compiler, fork this repository and open a pull request with your changes. Please, work on your own another branch.

Check [jup](https://github.com/junon-corp/jup) for the code the tokenizing and parsing parts.

## Requirements
The compiler requires some programs at runtime : 

- **Linux** : [nasm](https://www.nasm.us/), ld

## Syntax highlighter
Create projects with Junon to make the language's integration possible on [github/linguist](https://github.com/github/linguist)

> We try only to add languages once they have some usage on GitHub. In most cases we prefer that each new file extension be in use in at least 200 unique `:user/:repo` repositories before supporting them in Linguist. \
*from [linguist/contributing.md](https://github.com/github/linguist/blob/master/CONTRIBUTING.md#Adding-a-language)*

For common editors, may extensions will be created once the language style rules will be finally set. 
