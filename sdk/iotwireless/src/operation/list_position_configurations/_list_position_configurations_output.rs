// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[deprecated(note = "This operation is no longer supported.")]
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPositionConfigurationsOutput {
    /// <p>A list of position configurations.</p>
    #[doc(hidden)]
    pub position_configuration_list:
        ::std::option::Option<::std::vec::Vec<crate::types::PositionConfigurationItem>>,
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListPositionConfigurationsOutput {
    /// <p>A list of position configurations.</p>
    pub fn position_configuration_list(
        &self,
    ) -> ::std::option::Option<&[crate::types::PositionConfigurationItem]> {
        self.position_configuration_list.as_deref()
    }
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListPositionConfigurationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListPositionConfigurationsOutput {
    /// Creates a new builder-style object to manufacture [`ListPositionConfigurationsOutput`](crate::operation::list_position_configurations::ListPositionConfigurationsOutput).
    pub fn builder() -> crate::operation::list_position_configurations::builders::ListPositionConfigurationsOutputBuilder{
        crate::operation::list_position_configurations::builders::ListPositionConfigurationsOutputBuilder::default()
    }
}

/// A builder for [`ListPositionConfigurationsOutput`](crate::operation::list_position_configurations::ListPositionConfigurationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListPositionConfigurationsOutputBuilder {
    pub(crate) position_configuration_list:
        ::std::option::Option<::std::vec::Vec<crate::types::PositionConfigurationItem>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListPositionConfigurationsOutputBuilder {
    /// Appends an item to `position_configuration_list`.
    ///
    /// To override the contents of this collection use [`set_position_configuration_list`](Self::set_position_configuration_list).
    ///
    /// <p>A list of position configurations.</p>
    pub fn position_configuration_list(
        mut self,
        input: crate::types::PositionConfigurationItem,
    ) -> Self {
        let mut v = self.position_configuration_list.unwrap_or_default();
        v.push(input);
        self.position_configuration_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of position configurations.</p>
    pub fn set_position_configuration_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PositionConfigurationItem>>,
    ) -> Self {
        self.position_configuration_list = input;
        self
    }
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to get the next set of results, or <b>null</b> if there are no additional results.</p>
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
    /// Consumes the builder and constructs a [`ListPositionConfigurationsOutput`](crate::operation::list_position_configurations::ListPositionConfigurationsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_position_configurations::ListPositionConfigurationsOutput {
        crate::operation::list_position_configurations::ListPositionConfigurationsOutput {
            position_configuration_list: self.position_configuration_list,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
