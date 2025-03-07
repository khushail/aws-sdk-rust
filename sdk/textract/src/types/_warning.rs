// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A warning about an issue that occurred during asynchronous text analysis (<code>StartDocumentAnalysis</code>) or asynchronous document text detection (<code>StartDocumentTextDetection</code>). </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Warning {
    /// <p>The error code for the warning.</p>
    #[doc(hidden)]
    pub error_code: ::std::option::Option<::std::string::String>,
    /// <p>A list of the pages that the warning applies to.</p>
    #[doc(hidden)]
    pub pages: ::std::option::Option<::std::vec::Vec<i32>>,
}
impl Warning {
    /// <p>The error code for the warning.</p>
    pub fn error_code(&self) -> ::std::option::Option<&str> {
        self.error_code.as_deref()
    }
    /// <p>A list of the pages that the warning applies to.</p>
    pub fn pages(&self) -> ::std::option::Option<&[i32]> {
        self.pages.as_deref()
    }
}
impl Warning {
    /// Creates a new builder-style object to manufacture [`Warning`](crate::types::Warning).
    pub fn builder() -> crate::types::builders::WarningBuilder {
        crate::types::builders::WarningBuilder::default()
    }
}

/// A builder for [`Warning`](crate::types::Warning).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct WarningBuilder {
    pub(crate) error_code: ::std::option::Option<::std::string::String>,
    pub(crate) pages: ::std::option::Option<::std::vec::Vec<i32>>,
}
impl WarningBuilder {
    /// <p>The error code for the warning.</p>
    pub fn error_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.error_code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The error code for the warning.</p>
    pub fn set_error_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.error_code = input;
        self
    }
    /// Appends an item to `pages`.
    ///
    /// To override the contents of this collection use [`set_pages`](Self::set_pages).
    ///
    /// <p>A list of the pages that the warning applies to.</p>
    pub fn pages(mut self, input: i32) -> Self {
        let mut v = self.pages.unwrap_or_default();
        v.push(input);
        self.pages = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the pages that the warning applies to.</p>
    pub fn set_pages(mut self, input: ::std::option::Option<::std::vec::Vec<i32>>) -> Self {
        self.pages = input;
        self
    }
    /// Consumes the builder and constructs a [`Warning`](crate::types::Warning).
    pub fn build(self) -> crate::types::Warning {
        crate::types::Warning {
            error_code: self.error_code,
            pages: self.pages,
        }
    }
}
