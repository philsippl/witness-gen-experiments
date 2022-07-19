# Circom Rust Witness Generator

> ⚠️ This is all super experimental and WIP

This is a hacky approach of implementing a Rust native witness generator for circom. 
Currently, circom only supports wasm and c++. Wasm is fine for browser-native use cases, but 
lacks the performance of the c++ version. Also, the c++ version is not cross-compilation friendly
due to custom assembly code that implements most of the field math. 

Quite obviously, everybody would love to have a native rust generator instead. Unfortunately, the 
code generator in the circom codebase is very entangled with the current two backends and it seemed 
like a lot of effort and changes to implement Rust as a third one. 

This repository takes the short cut of reusing the c++ generator backend, and then "transpiling" the c++ 
into rust. All field math was implement as macros and in general the rust code was written a lot to look like c++ 
in order to have minimal changes. The "transpiling" is simple string replacement in python right now for the sake 
of the experiment.

## Run 
> TODO: there is still a lot of manual tweaking

- Generate files: `circom test.circom --r1cs --c`
- Run transpiler: `python3 cpp_rs.py`
- Read constants (TODO: run automatically and insert in generated code): `python3 read_constants.py`


