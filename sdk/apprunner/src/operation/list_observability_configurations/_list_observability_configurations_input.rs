// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListObservabilityConfigurationsInput {
    /// <p>The name of the App Runner observability configuration that you want to list. If specified, App Runner lists revisions that share this name. If not specified, App Runner returns revisions of all active configurations.</p>
    #[doc(hidden)]
    pub observability_configuration_name: ::std::option::Option<::std::string::String>,
    /// <p>Set to <code>true</code> to list only the latest revision for each requested configuration name.</p>
    /// <p>Set to <code>false</code> to list all revisions for each requested configuration name.</p>
    /// <p>Default: <code>true</code> </p>
    #[doc(hidden)]
    pub latest_only: ::std::option::Option<bool>,
    /// <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>
    /// <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>A token from a previous result page. It's used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones that are specified in the initial request.</p>
    /// <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListObservabilityConfigurationsInput {
    /// <p>The name of the App Runner observability configuration that you want to list. If specified, App Runner lists revisions that share this name. If not specified, App Runner returns revisions of all active configurations.</p>
    pub fn observability_configuration_name(&self) -> ::std::option::Option<&str> {
        self.observability_configuration_name.as_deref()
    }
    /// <p>Set to <code>true</code> to list only the latest revision for each requested configuration name.</p>
    /// <p>Set to <code>false</code> to list all revisions for each requested configuration name.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn latest_only(&self) -> ::std::option::Option<bool> {
        self.latest_only
    }
    /// <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>
    /// <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>A token from a previous result page. It's used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones that are specified in the initial request.</p>
    /// <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListObservabilityConfigurationsInput {
    /// Creates a new builder-style object to manufacture [`ListObservabilityConfigurationsInput`](crate::operation::list_observability_configurations::ListObservabilityConfigurationsInput).
    pub fn builder() -> crate::operation::list_observability_configurations::builders::ListObservabilityConfigurationsInputBuilder{
        crate::operation::list_observability_configurations::builders::ListObservabilityConfigurationsInputBuilder::default()
    }
}

/// A builder for [`ListObservabilityConfigurationsInput`](crate::operation::list_observability_configurations::ListObservabilityConfigurationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListObservabilityConfigurationsInputBuilder {
    pub(crate) observability_configuration_name: ::std::option::Option<::std::string::String>,
    pub(crate) latest_only: ::std::option::Option<bool>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListObservabilityConfigurationsInputBuilder {
    /// <p>The name of the App Runner observability configuration that you want to list. If specified, App Runner lists revisions that share this name. If not specified, App Runner returns revisions of all active configurations.</p>
    pub fn observability_configuration_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.observability_configuration_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the App Runner observability configuration that you want to list. If specified, App Runner lists revisions that share this name. If not specified, App Runner returns revisions of all active configurations.</p>
    pub fn set_observability_configuration_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.observability_configuration_name = input;
        self
    }
    /// <p>Set to <code>true</code> to list only the latest revision for each requested configuration name.</p>
    /// <p>Set to <code>false</code> to list all revisions for each requested configuration name.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn latest_only(mut self, input: bool) -> Self {
        self.latest_only = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set to <code>true</code> to list only the latest revision for each requested configuration name.</p>
    /// <p>Set to <code>false</code> to list all revisions for each requested configuration name.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn set_latest_only(mut self, input: ::std::option::Option<bool>) -> Self {
        self.latest_only = input;
        self
    }
    /// <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>
    /// <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to include in each response (result page). It's used for a paginated request.</p>
    /// <p>If you don't specify <code>MaxResults</code>, the request retrieves all available results in a single response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>A token from a previous result page. It's used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones that are specified in the initial request.</p>
    /// <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token from a previous result page. It's used for a paginated request. The request retrieves the next result page. All other parameter values must be identical to the ones that are specified in the initial request.</p>
    /// <p>If you don't specify <code>NextToken</code>, the request retrieves the first result page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListObservabilityConfigurationsInput`](crate::operation::list_observability_configurations::ListObservabilityConfigurationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_observability_configurations::ListObservabilityConfigurationsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_observability_configurations::ListObservabilityConfigurationsInput {
                observability_configuration_name: self.observability_configuration_name
                ,
                latest_only: self.latest_only
                ,
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
