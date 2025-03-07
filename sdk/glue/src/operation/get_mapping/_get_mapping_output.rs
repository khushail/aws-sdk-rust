// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetMappingOutput {
    /// <p>A list of mappings to the specified targets.</p>
    #[doc(hidden)]
    pub mapping: ::std::option::Option<::std::vec::Vec<crate::types::MappingEntry>>,
    _request_id: Option<String>,
}
impl GetMappingOutput {
    /// <p>A list of mappings to the specified targets.</p>
    pub fn mapping(&self) -> ::std::option::Option<&[crate::types::MappingEntry]> {
        self.mapping.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetMappingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetMappingOutput {
    /// Creates a new builder-style object to manufacture [`GetMappingOutput`](crate::operation::get_mapping::GetMappingOutput).
    pub fn builder() -> crate::operation::get_mapping::builders::GetMappingOutputBuilder {
        crate::operation::get_mapping::builders::GetMappingOutputBuilder::default()
    }
}

/// A builder for [`GetMappingOutput`](crate::operation::get_mapping::GetMappingOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetMappingOutputBuilder {
    pub(crate) mapping: ::std::option::Option<::std::vec::Vec<crate::types::MappingEntry>>,
    _request_id: Option<String>,
}
impl GetMappingOutputBuilder {
    /// Appends an item to `mapping`.
    ///
    /// To override the contents of this collection use [`set_mapping`](Self::set_mapping).
    ///
    /// <p>A list of mappings to the specified targets.</p>
    pub fn mapping(mut self, input: crate::types::MappingEntry) -> Self {
        let mut v = self.mapping.unwrap_or_default();
        v.push(input);
        self.mapping = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of mappings to the specified targets.</p>
    pub fn set_mapping(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MappingEntry>>,
    ) -> Self {
        self.mapping = input;
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
    /// Consumes the builder and constructs a [`GetMappingOutput`](crate::operation::get_mapping::GetMappingOutput).
    pub fn build(self) -> crate::operation::get_mapping::GetMappingOutput {
        crate::operation::get_mapping::GetMappingOutput {
            mapping: self.mapping,
            _request_id: self._request_id,
        }
    }
}
