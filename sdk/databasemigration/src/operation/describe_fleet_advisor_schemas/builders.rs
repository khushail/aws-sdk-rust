// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_fleet_advisor_schemas::_describe_fleet_advisor_schemas_output::DescribeFleetAdvisorSchemasOutputBuilder;

pub use crate::operation::describe_fleet_advisor_schemas::_describe_fleet_advisor_schemas_input::DescribeFleetAdvisorSchemasInputBuilder;

/// Fluent builder constructing a request to `DescribeFleetAdvisorSchemas`.
///
/// <p>Returns a list of schemas detected by Fleet Advisor Collectors in your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeFleetAdvisorSchemasFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_fleet_advisor_schemas::builders::DescribeFleetAdvisorSchemasInputBuilder,
}
impl DescribeFleetAdvisorSchemasFluentBuilder {
    /// Creates a new `DescribeFleetAdvisorSchemas`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemas,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemasError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemasOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemasError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemasOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemasError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemas,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_fleet_advisor_schemas::DescribeFleetAdvisorSchemasError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_fleet_advisor_schemas::paginator::DescribeFleetAdvisorSchemasPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_fleet_advisor_schemas::paginator::DescribeFleetAdvisorSchemasPaginator{
        crate::operation::describe_fleet_advisor_schemas::paginator::DescribeFleetAdvisorSchemasPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p> If you specify any of the following filters, the output includes information for only those schemas that meet the filter criteria:</p>
    /// <ul>
    /// <li> <p> <code>complexity</code> – The schema's complexity, for example <code>Simple</code>.</p> </li>
    /// <li> <p> <code>database-id</code> – The ID of the schema's database.</p> </li>
    /// <li> <p> <code>database-ip-address</code> – The IP address of the schema's database.</p> </li>
    /// <li> <p> <code>database-name</code> – The name of the schema's database.</p> </li>
    /// <li> <p> <code>database-engine</code> – The name of the schema database's engine.</p> </li>
    /// <li> <p> <code>original-schema-name</code> – The name of the schema's database's main schema.</p> </li>
    /// <li> <p> <code>schema-id</code> – The ID of the schema, for example <code>15</code>.</p> </li>
    /// <li> <p> <code>schema-name</code> – The name of the schema.</p> </li>
    /// <li> <p> <code>server-ip-address</code> – The IP address of the schema database's server.</p> </li>
    /// </ul>
    /// <p>An example is: <code>describe-fleet-advisor-schemas --filter Name="schema-id",Values="50"</code> </p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p> If you specify any of the following filters, the output includes information for only those schemas that meet the filter criteria:</p>
    /// <ul>
    /// <li> <p> <code>complexity</code> – The schema's complexity, for example <code>Simple</code>.</p> </li>
    /// <li> <p> <code>database-id</code> – The ID of the schema's database.</p> </li>
    /// <li> <p> <code>database-ip-address</code> – The IP address of the schema's database.</p> </li>
    /// <li> <p> <code>database-name</code> – The name of the schema's database.</p> </li>
    /// <li> <p> <code>database-engine</code> – The name of the schema database's engine.</p> </li>
    /// <li> <p> <code>original-schema-name</code> – The name of the schema's database's main schema.</p> </li>
    /// <li> <p> <code>schema-id</code> – The ID of the schema, for example <code>15</code>.</p> </li>
    /// <li> <p> <code>schema-name</code> – The name of the schema.</p> </li>
    /// <li> <p> <code>server-ip-address</code> – The IP address of the schema database's server.</p> </li>
    /// </ul>
    /// <p>An example is: <code>describe-fleet-advisor-schemas --filter Name="schema-id",Values="50"</code> </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Sets the maximum number of records returned in the response.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>Sets the maximum number of records returned in the response.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>If <code>NextToken</code> is returned by a previous response, there are more results available. The value of <code>NextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If <code>NextToken</code> is returned by a previous response, there are more results available. The value of <code>NextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page. Keep all other arguments unchanged. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
