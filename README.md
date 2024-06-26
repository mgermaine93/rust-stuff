# Difference between Python and Rust

## Python

- Interpreted
- Needs an interpreter (e.g., CPython)
- Flexible (all types)
- Everything is an object

## Rust

- Compiled
- Needs a compiler
- Strongly typed
- Ownership, smart pointers

## Rust Terms:

- Crates = smallest amount of code that the Rust compiler considers at a time.
- "Macros" = code that writes other code (somewhat similar to packages, etc.). These are reusable code (functions, etc.).
- Print a failure message, unwind, clean up the stack, and quit.

## What is Maturin?

- It's the library we used to build these Rust modules.

## Adding functions to modules

- You don't need to create a new module for each function.
- You can just add functions to a module (provided that the module is one that you created).

```
pyenv virtualenv 3.12.2 pyo3
pyenv activate pyo3
pip install maturin
```
