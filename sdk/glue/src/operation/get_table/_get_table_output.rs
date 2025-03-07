// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTableOutput {
    /// <p>The <code>Table</code> object that defines the specified table.</p>
    #[doc(hidden)]
    pub table: ::std::option::Option<crate::types::Table>,
    _request_id: Option<String>,
}
impl GetTableOutput {
    /// <p>The <code>Table</code> object that defines the specified table.</p>
    pub fn table(&self) -> ::std::option::Option<&crate::types::Table> {
        self.table.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTableOutput {
    /// Creates a new builder-style object to manufacture [`GetTableOutput`](crate::operation::get_table::GetTableOutput).
    pub fn builder() -> crate::operation::get_table::builders::GetTableOutputBuilder {
        crate::operation::get_table::builders::GetTableOutputBuilder::default()
    }
}

/// A builder for [`GetTableOutput`](crate::operation::get_table::GetTableOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetTableOutputBuilder {
    pub(crate) table: ::std::option::Option<crate::types::Table>,
    _request_id: Option<String>,
}
impl GetTableOutputBuilder {
    /// <p>The <code>Table</code> object that defines the specified table.</p>
    pub fn table(mut self, input: crate::types::Table) -> Self {
        self.table = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>Table</code> object that defines the specified table.</p>
    pub fn set_table(mut self, input: ::std::option::Option<crate::types::Table>) -> Self {
        self.table = input;
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
    /// Consumes the builder and constructs a [`GetTableOutput`](crate::operation::get_table::GetTableOutput).
    pub fn build(self) -> crate::operation::get_table::GetTableOutput {
        crate::operation::get_table::GetTableOutput {
            table: self.table,
            _request_id: self._request_id,
        }
    }
}
