// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResiliencyPoliciesInput {
    /// <p>The name of the policy</p>
    #[doc(hidden)]
    pub policy_name: ::std::option::Option<::std::string::String>,
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListResiliencyPoliciesInput {
    /// <p>The name of the policy</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListResiliencyPoliciesInput {
    /// Creates a new builder-style object to manufacture [`ListResiliencyPoliciesInput`](crate::operation::list_resiliency_policies::ListResiliencyPoliciesInput).
    pub fn builder(
    ) -> crate::operation::list_resiliency_policies::builders::ListResiliencyPoliciesInputBuilder
    {
        crate::operation::list_resiliency_policies::builders::ListResiliencyPoliciesInputBuilder::default()
    }
}

/// A builder for [`ListResiliencyPoliciesInput`](crate::operation::list_resiliency_policies::ListResiliencyPoliciesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListResiliencyPoliciesInputBuilder {
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListResiliencyPoliciesInputBuilder {
    /// <p>The name of the policy</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the policy</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListResiliencyPoliciesInput`](crate::operation::list_resiliency_policies::ListResiliencyPoliciesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_resiliency_policies::ListResiliencyPoliciesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_resiliency_policies::ListResiliencyPoliciesInput {
                policy_name: self.policy_name,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
