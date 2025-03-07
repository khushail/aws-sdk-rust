// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSecurityConfigsInput {
    /// <p>The type of security configuration.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::SecurityConfigType>,
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListSecurityConfigsInput {
    /// <p>The type of security configuration.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::SecurityConfigType> {
        self.r#type.as_ref()
    }
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListSecurityConfigsInput {
    /// Creates a new builder-style object to manufacture [`ListSecurityConfigsInput`](crate::operation::list_security_configs::ListSecurityConfigsInput).
    pub fn builder(
    ) -> crate::operation::list_security_configs::builders::ListSecurityConfigsInputBuilder {
        crate::operation::list_security_configs::builders::ListSecurityConfigsInputBuilder::default(
        )
    }
}

/// A builder for [`ListSecurityConfigsInput`](crate::operation::list_security_configs::ListSecurityConfigsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSecurityConfigsInputBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::SecurityConfigType>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListSecurityConfigsInputBuilder {
    /// <p>The type of security configuration.</p>
    pub fn r#type(mut self, input: crate::types::SecurityConfigType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of security configuration.</p>
    pub fn set_type(
        mut self,
        input: ::std::option::Option<crate::types::SecurityConfigType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If your initial <code>ListSecurityConfigs</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>ListSecurityConfigs</code> operations, which returns results in the next page. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results. The default is 20.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListSecurityConfigsInput`](crate::operation::list_security_configs::ListSecurityConfigsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_security_configs::ListSecurityConfigsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_security_configs::ListSecurityConfigsInput {
                r#type: self.r#type,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
