// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAccountAttributesOutput {
    /// <p>Information about the account attributes.</p>
    #[doc(hidden)]
    pub account_attributes: ::std::option::Option<::std::vec::Vec<crate::types::AccountAttribute>>,
    _request_id: Option<String>,
}
impl DescribeAccountAttributesOutput {
    /// <p>Information about the account attributes.</p>
    pub fn account_attributes(&self) -> ::std::option::Option<&[crate::types::AccountAttribute]> {
        self.account_attributes.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeAccountAttributesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAccountAttributesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAccountAttributesOutput`](crate::operation::describe_account_attributes::DescribeAccountAttributesOutput).
    pub fn builder() -> crate::operation::describe_account_attributes::builders::DescribeAccountAttributesOutputBuilder{
        crate::operation::describe_account_attributes::builders::DescribeAccountAttributesOutputBuilder::default()
    }
}

/// A builder for [`DescribeAccountAttributesOutput`](crate::operation::describe_account_attributes::DescribeAccountAttributesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAccountAttributesOutputBuilder {
    pub(crate) account_attributes:
        ::std::option::Option<::std::vec::Vec<crate::types::AccountAttribute>>,
    _request_id: Option<String>,
}
impl DescribeAccountAttributesOutputBuilder {
    /// Appends an item to `account_attributes`.
    ///
    /// To override the contents of this collection use [`set_account_attributes`](Self::set_account_attributes).
    ///
    /// <p>Information about the account attributes.</p>
    pub fn account_attributes(mut self, input: crate::types::AccountAttribute) -> Self {
        let mut v = self.account_attributes.unwrap_or_default();
        v.push(input);
        self.account_attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the account attributes.</p>
    pub fn set_account_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AccountAttribute>>,
    ) -> Self {
        self.account_attributes = input;
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
    /// Consumes the builder and constructs a [`DescribeAccountAttributesOutput`](crate::operation::describe_account_attributes::DescribeAccountAttributesOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_account_attributes::DescribeAccountAttributesOutput {
        crate::operation::describe_account_attributes::DescribeAccountAttributesOutput {
            account_attributes: self.account_attributes,
            _request_id: self._request_id,
        }
    }
}
