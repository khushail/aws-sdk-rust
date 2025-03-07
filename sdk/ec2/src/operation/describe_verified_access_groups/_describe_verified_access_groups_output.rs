// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVerifiedAccessGroupsOutput {
    /// <p>The ID of the Verified Access group.</p>
    #[doc(hidden)]
    pub verified_access_groups:
        ::std::option::Option<::std::vec::Vec<crate::types::VerifiedAccessGroup>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVerifiedAccessGroupsOutput {
    /// <p>The ID of the Verified Access group.</p>
    pub fn verified_access_groups(
        &self,
    ) -> ::std::option::Option<&[crate::types::VerifiedAccessGroup]> {
        self.verified_access_groups.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeVerifiedAccessGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVerifiedAccessGroupsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVerifiedAccessGroupsOutput`](crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput).
    pub fn builder() -> crate::operation::describe_verified_access_groups::builders::DescribeVerifiedAccessGroupsOutputBuilder{
        crate::operation::describe_verified_access_groups::builders::DescribeVerifiedAccessGroupsOutputBuilder::default()
    }
}

/// A builder for [`DescribeVerifiedAccessGroupsOutput`](crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVerifiedAccessGroupsOutputBuilder {
    pub(crate) verified_access_groups:
        ::std::option::Option<::std::vec::Vec<crate::types::VerifiedAccessGroup>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeVerifiedAccessGroupsOutputBuilder {
    /// Appends an item to `verified_access_groups`.
    ///
    /// To override the contents of this collection use [`set_verified_access_groups`](Self::set_verified_access_groups).
    ///
    /// <p>The ID of the Verified Access group.</p>
    pub fn verified_access_groups(mut self, input: crate::types::VerifiedAccessGroup) -> Self {
        let mut v = self.verified_access_groups.unwrap_or_default();
        v.push(input);
        self.verified_access_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ID of the Verified Access group.</p>
    pub fn set_verified_access_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VerifiedAccessGroup>>,
    ) -> Self {
        self.verified_access_groups = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeVerifiedAccessGroupsOutput`](crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput {
        crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput {
            verified_access_groups: self.verified_access_groups,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
