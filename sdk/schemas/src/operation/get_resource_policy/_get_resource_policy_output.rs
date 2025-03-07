// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetResourcePolicyOutput {
    /// <p>The resource-based policy.</p>
    #[doc(hidden)]
    pub policy: ::std::option::Option<::std::string::String>,
    /// <p>The revision ID.</p>
    #[doc(hidden)]
    pub revision_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetResourcePolicyOutput {
    /// <p>The resource-based policy.</p>
    pub fn policy(&self) -> ::std::option::Option<&str> {
        self.policy.as_deref()
    }
    /// <p>The revision ID.</p>
    pub fn revision_id(&self) -> ::std::option::Option<&str> {
        self.revision_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetResourcePolicyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetResourcePolicyOutput {
    /// Creates a new builder-style object to manufacture [`GetResourcePolicyOutput`](crate::operation::get_resource_policy::GetResourcePolicyOutput).
    pub fn builder(
    ) -> crate::operation::get_resource_policy::builders::GetResourcePolicyOutputBuilder {
        crate::operation::get_resource_policy::builders::GetResourcePolicyOutputBuilder::default()
    }
}

/// A builder for [`GetResourcePolicyOutput`](crate::operation::get_resource_policy::GetResourcePolicyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetResourcePolicyOutputBuilder {
    pub(crate) policy: ::std::option::Option<::std::string::String>,
    pub(crate) revision_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetResourcePolicyOutputBuilder {
    /// <p>The resource-based policy.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The resource-based policy.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// <p>The revision ID.</p>
    pub fn revision_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.revision_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The revision ID.</p>
    pub fn set_revision_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.revision_id = input;
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
    /// Consumes the builder and constructs a [`GetResourcePolicyOutput`](crate::operation::get_resource_policy::GetResourcePolicyOutput).
    pub fn build(self) -> crate::operation::get_resource_policy::GetResourcePolicyOutput {
        crate::operation::get_resource_policy::GetResourcePolicyOutput {
            policy: self.policy,
            revision_id: self.revision_id,
            _request_id: self._request_id,
        }
    }
}
