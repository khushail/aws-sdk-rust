// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A collection of the imported DocumentationPart identifiers.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportDocumentationPartsOutput {
    /// <p>A list of the returned documentation part identifiers.</p>
    #[doc(hidden)]
    pub ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of warning messages reported during import of documentation parts.</p>
    #[doc(hidden)]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl ImportDocumentationPartsOutput {
    /// <p>A list of the returned documentation part identifiers.</p>
    pub fn ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.ids.as_deref()
    }
    /// <p>A list of warning messages reported during import of documentation parts.</p>
    pub fn warnings(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.warnings.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ImportDocumentationPartsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ImportDocumentationPartsOutput {
    /// Creates a new builder-style object to manufacture [`ImportDocumentationPartsOutput`](crate::operation::import_documentation_parts::ImportDocumentationPartsOutput).
    pub fn builder(
    ) -> crate::operation::import_documentation_parts::builders::ImportDocumentationPartsOutputBuilder
    {
        crate::operation::import_documentation_parts::builders::ImportDocumentationPartsOutputBuilder::default()
    }
}

/// A builder for [`ImportDocumentationPartsOutput`](crate::operation::import_documentation_parts::ImportDocumentationPartsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportDocumentationPartsOutputBuilder {
    pub(crate) ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) warnings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    _request_id: Option<String>,
}
impl ImportDocumentationPartsOutputBuilder {
    /// Appends an item to `ids`.
    ///
    /// To override the contents of this collection use [`set_ids`](Self::set_ids).
    ///
    /// <p>A list of the returned documentation part identifiers.</p>
    pub fn ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.ids.unwrap_or_default();
        v.push(input.into());
        self.ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the returned documentation part identifiers.</p>
    pub fn set_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.ids = input;
        self
    }
    /// Appends an item to `warnings`.
    ///
    /// To override the contents of this collection use [`set_warnings`](Self::set_warnings).
    ///
    /// <p>A list of warning messages reported during import of documentation parts.</p>
    pub fn warnings(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.warnings.unwrap_or_default();
        v.push(input.into());
        self.warnings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of warning messages reported during import of documentation parts.</p>
    pub fn set_warnings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.warnings = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ImportDocumentationPartsOutput`](crate::operation::import_documentation_parts::ImportDocumentationPartsOutput).
    pub fn build(
        self,
    ) -> crate::operation::import_documentation_parts::ImportDocumentationPartsOutput {
        crate::operation::import_documentation_parts::ImportDocumentationPartsOutput {
            ids: self.ids,
            warnings: self.warnings,
            _request_id: self._request_id,
        }
    }
}
