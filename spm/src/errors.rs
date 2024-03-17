use crossterm::style::Stylize;
use itertools::Itertools;
use miette::Diagnostic;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Too many profiles matching `{}` were found", .query)]
pub struct TooManyErr {
    pub query: String,
    pub options: Vec<String>,
}

impl Diagnostic for TooManyErr {
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!(
            "Consider using more specific query. Matching items: {}",
            self.options.iter().map(|i| i.clone().blue()).join(", ")
        )))
    }
}
