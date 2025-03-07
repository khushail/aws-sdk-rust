// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_event_details_for_organization::_describe_event_details_for_organization_output::DescribeEventDetailsForOrganizationOutputBuilder;

pub use crate::operation::describe_event_details_for_organization::_describe_event_details_for_organization_input::DescribeEventDetailsForOrganizationInputBuilder;

/// Fluent builder constructing a request to `DescribeEventDetailsForOrganization`.
///
/// <p>Returns detailed information about one or more specified events for one or more Amazon Web Services accounts in your organization. This information includes standard event data (such as the Amazon Web Services Region and service), an event description, and (depending on the event) possible metadata. This operation doesn't return affected entities, such as the resources related to the event. To return affected entities, use the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_DescribeAffectedEntitiesForOrganization.html">DescribeAffectedEntitiesForOrganization</a> operation.</p> <note>
/// <p>Before you can call this operation, you must first enable Health to work with Organizations. To do this, call the <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_EnableHealthServiceAccessForOrganization.html">EnableHealthServiceAccessForOrganization</a> operation from your organization's management account.</p>
/// </note>
/// <p>When you call the <code>DescribeEventDetailsForOrganization</code> operation, specify the <code>organizationEventDetailFilters</code> object in the request. Depending on the Health event type, note the following differences:</p>
/// <ul>
/// <li> <p>To return event details for a public event, you must specify a null value for the <code>awsAccountId</code> parameter. If you specify an account ID for a public event, Health returns an error message because public events aren't specific to an account.</p> </li>
/// <li> <p>To return event details for an event that is specific to an account in your organization, you must specify the <code>awsAccountId</code> parameter in the request. If you don't specify an account ID, Health returns an error message because the event is specific to an account in your organization. </p> </li>
/// </ul>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/health/latest/APIReference/API_Event.html">Event</a>.</p> <note>
/// <p>This operation doesn't support resource-level permissions. You can't use this operation to allow or deny access to specific Health events. For more information, see <a href="https://docs.aws.amazon.com/health/latest/ug/security_iam_id-based-policy-examples.html#resource-action-based-conditions">Resource- and action-based conditions</a> in the <i>Health User Guide</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeEventDetailsForOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::describe_event_details_for_organization::builders::DescribeEventDetailsForOrganizationInputBuilder,
}
impl DescribeEventDetailsForOrganizationFluentBuilder {
    /// Creates a new `DescribeEventDetailsForOrganization`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
                    pub async fn customize_middleware(self) -> ::std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganization, ::aws_http::retry::AwsResponseRetryClassifier,>,
                        ::aws_smithy_http::result::SdkError<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationError>
    >{
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
                    pub async fn send_middleware(self) -> ::std::result::Result<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationError>>
                     {
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
                        pub async fn send(self) -> ::std::result::Result<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationOutput, ::aws_smithy_http::result::SdkError<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationError>>
                         {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                        pub async fn customize(self) -> ::std::result::Result<
                            crate::client::customize::CustomizableOperation<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganization, ::aws_http::retry::AwsResponseRetryClassifier,>,
                            ::aws_smithy_http::result::SdkError<crate::operation::describe_event_details_for_organization::DescribeEventDetailsForOrganizationError>
    >{
        self.customize_middleware().await
    }
    /// Appends an item to `organizationEventDetailFilters`.
    ///
    /// To override the contents of this collection use [`set_organization_event_detail_filters`](Self::set_organization_event_detail_filters).
    ///
    /// <p>A set of JSON elements that includes the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    pub fn organization_event_detail_filters(
        mut self,
        input: crate::types::EventAccountFilter,
    ) -> Self {
        self.inner = self.inner.organization_event_detail_filters(input);
        self
    }
    /// <p>A set of JSON elements that includes the <code>awsAccountId</code> and the <code>eventArn</code>.</p>
    pub fn set_organization_event_detail_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EventAccountFilter>>,
    ) -> Self {
        self.inner = self.inner.set_organization_event_detail_filters(input);
        self
    }
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    pub fn locale(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale(input.into());
        self
    }
    /// <p>The locale (language) to return information in. English (en) is the default and the only supported value at this time.</p>
    pub fn set_locale(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale(input);
        self
    }
}
