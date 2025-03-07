// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a source file that is part of changes made in a commit.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourceFileSpecifier {
    /// <p>The full path to the file, including the name of the file.</p>
    #[doc(hidden)]
    pub file_path: ::std::option::Option<::std::string::String>,
    /// <p>Whether to remove the source file from the parent commit.</p>
    #[doc(hidden)]
    pub is_move: bool,
}
impl SourceFileSpecifier {
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn file_path(&self) -> ::std::option::Option<&str> {
        self.file_path.as_deref()
    }
    /// <p>Whether to remove the source file from the parent commit.</p>
    pub fn is_move(&self) -> bool {
        self.is_move
    }
}
impl SourceFileSpecifier {
    /// Creates a new builder-style object to manufacture [`SourceFileSpecifier`](crate::types::SourceFileSpecifier).
    pub fn builder() -> crate::types::builders::SourceFileSpecifierBuilder {
        crate::types::builders::SourceFileSpecifierBuilder::default()
    }
}

/// A builder for [`SourceFileSpecifier`](crate::types::SourceFileSpecifier).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourceFileSpecifierBuilder {
    pub(crate) file_path: ::std::option::Option<::std::string::String>,
    pub(crate) is_move: ::std::option::Option<bool>,
}
impl SourceFileSpecifierBuilder {
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn file_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.file_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn set_file_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.file_path = input;
        self
    }
    /// <p>Whether to remove the source file from the parent commit.</p>
    pub fn is_move(mut self, input: bool) -> Self {
        self.is_move = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether to remove the source file from the parent commit.</p>
    pub fn set_is_move(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_move = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceFileSpecifier`](crate::types::SourceFileSpecifier).
    pub fn build(self) -> crate::types::SourceFileSpecifier {
        crate::types::SourceFileSpecifier {
            file_path: self.file_path,
            is_move: self.is_move.unwrap_or_default(),
        }
    }
}
