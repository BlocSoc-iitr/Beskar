## Beskar 

Beskar is a tool which uses `gambit`, a mutant generation tool from `Certora` to perform mutation testing on foundry projects. 

It allows developers to perform analysis of their tests and improve the test coverage by giving a smooth overview of results of testing on each mutant. 

### Mutation Testing 
Mutation Testing is a type of white box testing that is performed in order to evaluate the quality of existing software tests. Mutation testing is related to modification a program in small ways. The program files obtained after these modifications are called mutants. It focuses to help the tester develop effective tests or locate weaknesses in the test data used for the program. 

### Requirements 
1. Beskar is written in Rust. You'll need to [install Rust and
   Cargo](https://www.rust-lang.org/tools/install).
2. Gambit uses solc, the Solidity compiler, to generate mutants. You'll need to have a solc binary that is compatible with the project you are mutating.
3. lastly, install gambit by cloning [the Gambit repository](https://github.com/Certora/gambit) and run

```
cargo install --path .
```


### Installation

Simply run the following command in your terminal
```bash
cargo install beskar
```

## Usage 

Navigate to your foundry project root and run the following command

``` bash
beskar run .
```
The execution result will be logged to the terminal when it finishes the execution.

> Note: This repository is still under development.
