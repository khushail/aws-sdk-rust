// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAppVersionsInput {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    #[doc(hidden)]
    pub app_arn: ::std::option::Option<::std::string::String>,
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListAppVersionsInput {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(&self) -> ::std::option::Option<&str> {
        self.app_arn.as_deref()
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
impl ListAppVersionsInput {
    /// Creates a new builder-style object to manufacture [`ListAppVersionsInput`](crate::operation::list_app_versions::ListAppVersionsInput).
    pub fn builder() -> crate::operation::list_app_versions::builders::ListAppVersionsInputBuilder {
        crate::operation::list_app_versions::builders::ListAppVersionsInputBuilder::default()
    }
}

/// A builder for [`ListAppVersionsInput`](crate::operation::list_app_versions::ListAppVersionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAppVersionsInputBuilder {
    pub(crate) app_arn: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListAppVersionsInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn set_app_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_arn = input;
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
    /// Consumes the builder and constructs a [`ListAppVersionsInput`](crate::operation::list_app_versions::ListAppVersionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_app_versions::ListAppVersionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_app_versions::ListAppVersionsInput {
            app_arn: self.app_arn,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
