// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListConnectorDefinitionsOutput {
    /// Information about a definition.
    #[doc(hidden)]
    pub definitions: ::std::option::Option<::std::vec::Vec<crate::types::DefinitionInformation>>,
    /// The token for the next set of results, or ''null'' if there are no additional results.
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListConnectorDefinitionsOutput {
    /// Information about a definition.
    pub fn definitions(&self) -> ::std::option::Option<&[crate::types::DefinitionInformation]> {
        self.definitions.as_deref()
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListConnectorDefinitionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListConnectorDefinitionsOutput {
    /// Creates a new builder-style object to manufacture [`ListConnectorDefinitionsOutput`](crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput).
    pub fn builder(
    ) -> crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsOutputBuilder
    {
        crate::operation::list_connector_definitions::builders::ListConnectorDefinitionsOutputBuilder::default()
    }
}

/// A builder for [`ListConnectorDefinitionsOutput`](crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListConnectorDefinitionsOutputBuilder {
    pub(crate) definitions:
        ::std::option::Option<::std::vec::Vec<crate::types::DefinitionInformation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListConnectorDefinitionsOutputBuilder {
    /// Appends an item to `definitions`.
    ///
    /// To override the contents of this collection use [`set_definitions`](Self::set_definitions).
    ///
    /// Information about a definition.
    pub fn definitions(mut self, input: crate::types::DefinitionInformation) -> Self {
        let mut v = self.definitions.unwrap_or_default();
        v.push(input);
        self.definitions = ::std::option::Option::Some(v);
        self
    }
    /// Information about a definition.
    pub fn set_definitions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DefinitionInformation>>,
    ) -> Self {
        self.definitions = input;
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// The token for the next set of results, or ''null'' if there are no additional results.
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListConnectorDefinitionsOutput`](crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput {
        crate::operation::list_connector_definitions::ListConnectorDefinitionsOutput {
            definitions: self.definitions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
