// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDataSourceOutput {
    /// <p>The identifier of the data source connector.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateDataSourceOutput {
    /// <p>The identifier of the data source connector.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateDataSourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateDataSourceOutput {
    /// Creates a new builder-style object to manufacture [`CreateDataSourceOutput`](crate::operation::create_data_source::CreateDataSourceOutput).
    pub fn builder() -> crate::operation::create_data_source::builders::CreateDataSourceOutputBuilder
    {
        crate::operation::create_data_source::builders::CreateDataSourceOutputBuilder::default()
    }
}

/// A builder for [`CreateDataSourceOutput`](crate::operation::create_data_source::CreateDataSourceOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDataSourceOutputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateDataSourceOutputBuilder {
    /// <p>The identifier of the data source connector.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the data source connector.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
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
    /// Consumes the builder and constructs a [`CreateDataSourceOutput`](crate::operation::create_data_source::CreateDataSourceOutput).
    pub fn build(self) -> crate::operation::create_data_source::CreateDataSourceOutput {
        crate::operation::create_data_source::CreateDataSourceOutput {
            id: self.id,
            _request_id: self._request_id,
        }
    }
}
