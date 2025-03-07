// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListEntitlementsOutput {
    /// A list of entitlements that have been granted to you from other AWS accounts.
    #[doc(hidden)]
    pub entitlements: ::std::option::Option<::std::vec::Vec<crate::types::ListedEntitlement>>,
    /// The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListEntitlementsOutput {
    /// A list of entitlements that have been granted to you from other AWS accounts.
    pub fn entitlements(&self) -> ::std::option::Option<&[crate::types::ListedEntitlement]> {
        self.entitlements.as_deref()
    }
    /// The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListEntitlementsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListEntitlementsOutput {
    /// Creates a new builder-style object to manufacture [`ListEntitlementsOutput`](crate::operation::list_entitlements::ListEntitlementsOutput).
    pub fn builder() -> crate::operation::list_entitlements::builders::ListEntitlementsOutputBuilder
    {
        crate::operation::list_entitlements::builders::ListEntitlementsOutputBuilder::default()
    }
}

/// A builder for [`ListEntitlementsOutput`](crate::operation::list_entitlements::ListEntitlementsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListEntitlementsOutputBuilder {
    pub(crate) entitlements:
        ::std::option::Option<::std::vec::Vec<crate::types::ListedEntitlement>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListEntitlementsOutputBuilder {
    /// Appends an item to `entitlements`.
    ///
    /// To override the contents of this collection use [`set_entitlements`](Self::set_entitlements).
    ///
    /// A list of entitlements that have been granted to you from other AWS accounts.
    pub fn entitlements(mut self, input: crate::types::ListedEntitlement) -> Self {
        let mut v = self.entitlements.unwrap_or_default();
        v.push(input);
        self.entitlements = ::std::option::Option::Some(v);
        self
    }
    /// A list of entitlements that have been granted to you from other AWS accounts.
    pub fn set_entitlements(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ListedEntitlement>>,
    ) -> Self {
        self.entitlements = input;
        self
    }
    /// The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// The token that identifies which batch of results that you want to see. For example, you submit a ListEntitlements request with MaxResults set at 5. The service returns the first batch of results (up to 5) and a NextToken value. To see the next batch of results, you can submit the ListEntitlements request a second time and specify the NextToken value.
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
    /// Consumes the builder and constructs a [`ListEntitlementsOutput`](crate::operation::list_entitlements::ListEntitlementsOutput).
    pub fn build(self) -> crate::operation::list_entitlements::ListEntitlementsOutput {
        crate::operation::list_entitlements::ListEntitlementsOutput {
            entitlements: self.entitlements,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
