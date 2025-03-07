// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetReadinessCheckStatusInput {
    /// <p>The number of objects that you want to return with this call.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token that identifies which batch of results you want to see.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Name of a readiness check.</p>
    #[doc(hidden)]
    pub readiness_check_name: ::std::option::Option<::std::string::String>,
}
impl GetReadinessCheckStatusInput {
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Name of a readiness check.</p>
    pub fn readiness_check_name(&self) -> ::std::option::Option<&str> {
        self.readiness_check_name.as_deref()
    }
}
impl GetReadinessCheckStatusInput {
    /// Creates a new builder-style object to manufacture [`GetReadinessCheckStatusInput`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusInput).
    pub fn builder(
    ) -> crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusInputBuilder
    {
        crate::operation::get_readiness_check_status::builders::GetReadinessCheckStatusInputBuilder::default()
    }
}

/// A builder for [`GetReadinessCheckStatusInput`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetReadinessCheckStatusInputBuilder {
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) readiness_check_name: ::std::option::Option<::std::string::String>,
}
impl GetReadinessCheckStatusInputBuilder {
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Name of a readiness check.</p>
    pub fn readiness_check_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.readiness_check_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of a readiness check.</p>
    pub fn set_readiness_check_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.readiness_check_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetReadinessCheckStatusInput`](crate::operation::get_readiness_check_status::GetReadinessCheckStatusInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_readiness_check_status::GetReadinessCheckStatusInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_readiness_check_status::GetReadinessCheckStatusInput {
                max_results: self.max_results,
                next_token: self.next_token,
                readiness_check_name: self.readiness_check_name,
            },
        )
    }
}
