//! Contains the parser and AST format used by the Koto language

#![warn(missing_docs)]

mod ast;
mod error;
mod node;
mod parser;
mod string;
mod string_format_options;
mod string_slice;

#[cfg(feature = "num64")]
mod constant_pool64;
#[cfg(feature = "num64")]
pub use crate::constant_pool64::{Constant, ConstantIndex, ConstantPool};

#[cfg(feature = "num32")]
mod constant_pool32;
#[cfg(feature = "num32")]
pub use crate::constant_pool32::{Constant, ConstantIndex, ConstantPool};

pub use crate::{
    ast::*,
    error::{Error, Result, format_source_excerpt},
    node::*,
    parser::Parser,
    string::KString,
    string_format_options::{StringAlignment, StringFormatOptions, StringFormatRepresentation},
    string_slice::StringSlice,
};
pub use koto_lexer::{Position, RawStringDelimiter, Span, StringQuote, StringType};

#[cfg(feature = "num64")]
type KInt = i64;
#[cfg(feature = "num64")]
type KFloat = f64;

#[cfg(feature = "num32")]
type KInt = i32;
#[cfg(feature = "num32")]
type KFloat = f32;
