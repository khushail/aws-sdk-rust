// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribePortfolioShareStatusOutput {
    /// <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p>
    #[doc(hidden)]
    pub portfolio_share_token: ::std::option::Option<::std::string::String>,
    /// <p>The portfolio identifier.</p>
    #[doc(hidden)]
    pub portfolio_id: ::std::option::Option<::std::string::String>,
    /// <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p>
    #[doc(hidden)]
    pub organization_node_value: ::std::option::Option<::std::string::String>,
    /// <p>Status of the portfolio share operation.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ShareStatus>,
    /// <p>Information about the portfolio share operation.</p>
    #[doc(hidden)]
    pub share_details: ::std::option::Option<crate::types::ShareDetails>,
    _request_id: Option<String>,
}
impl DescribePortfolioShareStatusOutput {
    /// <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p>
    pub fn portfolio_share_token(&self) -> ::std::option::Option<&str> {
        self.portfolio_share_token.as_deref()
    }
    /// <p>The portfolio identifier.</p>
    pub fn portfolio_id(&self) -> ::std::option::Option<&str> {
        self.portfolio_id.as_deref()
    }
    /// <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p>
    pub fn organization_node_value(&self) -> ::std::option::Option<&str> {
        self.organization_node_value.as_deref()
    }
    /// <p>Status of the portfolio share operation.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ShareStatus> {
        self.status.as_ref()
    }
    /// <p>Information about the portfolio share operation.</p>
    pub fn share_details(&self) -> ::std::option::Option<&crate::types::ShareDetails> {
        self.share_details.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribePortfolioShareStatusOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribePortfolioShareStatusOutput {
    /// Creates a new builder-style object to manufacture [`DescribePortfolioShareStatusOutput`](crate::operation::describe_portfolio_share_status::DescribePortfolioShareStatusOutput).
    pub fn builder() -> crate::operation::describe_portfolio_share_status::builders::DescribePortfolioShareStatusOutputBuilder{
        crate::operation::describe_portfolio_share_status::builders::DescribePortfolioShareStatusOutputBuilder::default()
    }
}

/// A builder for [`DescribePortfolioShareStatusOutput`](crate::operation::describe_portfolio_share_status::DescribePortfolioShareStatusOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribePortfolioShareStatusOutputBuilder {
    pub(crate) portfolio_share_token: ::std::option::Option<::std::string::String>,
    pub(crate) portfolio_id: ::std::option::Option<::std::string::String>,
    pub(crate) organization_node_value: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ShareStatus>,
    pub(crate) share_details: ::std::option::Option<crate::types::ShareDetails>,
    _request_id: Option<String>,
}
impl DescribePortfolioShareStatusOutputBuilder {
    /// <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p>
    pub fn portfolio_share_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.portfolio_share_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the portfolio share operation. For example, <code>share-6v24abcdefghi</code>.</p>
    pub fn set_portfolio_share_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.portfolio_share_token = input;
        self
    }
    /// <p>The portfolio identifier.</p>
    pub fn portfolio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.portfolio_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The portfolio identifier.</p>
    pub fn set_portfolio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.portfolio_id = input;
        self
    }
    /// <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p>
    pub fn organization_node_value(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.organization_node_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Organization node identifier. It can be either account id, organizational unit id or organization id.</p>
    pub fn set_organization_node_value(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.organization_node_value = input;
        self
    }
    /// <p>Status of the portfolio share operation.</p>
    pub fn status(mut self, input: crate::types::ShareStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Status of the portfolio share operation.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ShareStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Information about the portfolio share operation.</p>
    pub fn share_details(mut self, input: crate::types::ShareDetails) -> Self {
        self.share_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the portfolio share operation.</p>
    pub fn set_share_details(
        mut self,
        input: ::std::option::Option<crate::types::ShareDetails>,
    ) -> Self {
        self.share_details = input;
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
    /// Consumes the builder and constructs a [`DescribePortfolioShareStatusOutput`](crate::operation::describe_portfolio_share_status::DescribePortfolioShareStatusOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_portfolio_share_status::DescribePortfolioShareStatusOutput {
        crate::operation::describe_portfolio_share_status::DescribePortfolioShareStatusOutput {
            portfolio_share_token: self.portfolio_share_token,
            portfolio_id: self.portfolio_id,
            organization_node_value: self.organization_node_value,
            status: self.status,
            share_details: self.share_details,
            _request_id: self._request_id,
        }
    }
}
