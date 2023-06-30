//! The `ram` library provides a simple interpreter for a custom assembly language, which is
//! designed to work with a RAM (Random Access Memory) machine model. This library allows users to
//! parse, validate, and execute assembly code, and to perform various operations on the RAM machine.
//!
//! The library is organized into the following modules:
//!
//! - [`errors`] for error types related to parsing and interpretation.
//! - [`parser`] for parsing assembly code into an intermediate representation.
//! - [`program`] for representing and working with a program in memory.
//! - [`ram`] for the RAM machine implementation and its execution logic.
//! - [`registers`] for working with the RAM machine registers.
//! - [`stmt`] for representing and working with assembly statements.
//!
//! Additionally, the library will provide the following optional features:
//!
//! - `wasm`: Adds WebAssembly bindings for using the library in a WebAssembly environment.
//! - `serde`: Adds serialization and deserialization support for the RAM machine state.
//!
//! [`errors`]: errors/index.html
//! [`parser`]: parser/index.html
//! [`program`]: program/index.html
//! [`ram`]: ram/index.html
//! [`registers`]: registers/index.html
//! [`stmt`]: stmt/index.html

#![warn(missing_docs)]

pub mod errors {
    //! The [`errors`] module provides error types for various parsing and interpretation errors
    //! that may occur during the parsing, validation, and execution of a program.
    //!
    //! This module includes the following error types:
    //! - [`ParseError`] for parsing errors that may occur during parsing and validating input.
    //! - [`InterpretError`] for interpretation errors that may occur during program execution.
    //!
    //! It also includes error-related types:
    //! - [`InvalidArgument`] for representing various invalid argument errors.
    //!
    //! [`ParseError`]: enum.ParseError.html
    //! [`InterpretError`]: enum.InterpretError.html
    //! [`InvalidArgument`]: enum.InvalidArgument.html
    //! [`errors`]: errors/index.html
    mod parser;
    mod ram;

    pub use parser::*;
    pub use ram::*;
}
pub mod parser;
pub mod program;
pub mod ram;
pub mod registers;
pub mod stmt;

// TODO: Serde feature
