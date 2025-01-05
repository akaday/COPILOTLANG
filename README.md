# CopilotLang

CopilotLang is a simple programming language and compiler implemented in Rust, designed to demonstrate the basic concepts of language design, lexing, and parsing. The project leverages Rust for the lexer and parser, with plans to incorporate MLIR and LLVM for intermediate representation and code generation.

## Features

- Basic types: `int`, `void`
- Variable declarations
- Function declarations
- Binary expressions
- Main function
- Error handling and type checking (in progress)

## Project Structure

- `src/lexer.rs`: Contains the lexer implementation that tokenizes the input source code.
- `src/parser.rs`: Contains the parser implementation that constructs the Abstract Syntax Tree (AST).
- `src/main.rs`: The main entry point of the compiler, demonstrating the use of the lexer and parser.
- `src/mlir/mod.rs`: The main module file for the MLIR integration.
- `src/mlir/ast_to_mlir/mod.rs`: The main module file for AST to MLIR conversion.
- `src/mlir/ast_to_mlir/conversion.rs`: Contains functions to traverse the AST and generate the corresponding MLIR code.
- `src/mlir/optimizations/mod.rs`: The main module file for MLIR optimizations.
- `src/mlir/optimizations/passes.rs`: Contains functions to perform various optimizations on the MLIR code.
- `src/mlir/mlir_to_llvm/mod.rs`: The main module file for MLIR to LLVM conversion.
- `src/mlir/mlir_to_llvm/conversion.rs`: Contains functions to convert the optimized MLIR code to LLVM IR.

## Getting Started

### Prerequisites

- Rust and Cargo installed. You can download them from [rust-lang.org](https://www.rust-lang.org/).
- (Optional) LLVM and MLIR for advanced compilation features. Follow the instructions on the [LLVM Project page](https://llvm.org/) to set them up.

### Installation

Clone the repository:

```bash
git clone https://github.com/akaday/COPILOT_LANG.git
cd copilot_lang
