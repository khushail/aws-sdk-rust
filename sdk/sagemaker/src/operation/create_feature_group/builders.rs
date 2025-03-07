// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_feature_group::_create_feature_group_output::CreateFeatureGroupOutputBuilder;

pub use crate::operation::create_feature_group::_create_feature_group_input::CreateFeatureGroupInputBuilder;

/// Fluent builder constructing a request to `CreateFeatureGroup`.
///
/// <p>Create a new <code>FeatureGroup</code>. A <code>FeatureGroup</code> is a group of <code>Features</code> defined in the <code>FeatureStore</code> to describe a <code>Record</code>. </p>
/// <p>The <code>FeatureGroup</code> defines the schema and features contained in the FeatureGroup. A <code>FeatureGroup</code> definition is composed of a list of <code>Features</code>, a <code>RecordIdentifierFeatureName</code>, an <code>EventTimeFeatureName</code> and configurations for its <code>OnlineStore</code> and <code>OfflineStore</code>. Check <a href="https://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html">Amazon Web Services service quotas</a> to see the <code>FeatureGroup</code>s quota for your Amazon Web Services account.</p> <important>
/// <p>You must include at least one of <code>OnlineStoreConfig</code> and <code>OfflineStoreConfig</code> to create a <code>FeatureGroup</code>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFeatureGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_feature_group::builders::CreateFeatureGroupInputBuilder,
}
impl CreateFeatureGroupFluentBuilder {
    /// Creates a new `CreateFeatureGroup`.
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
            crate::operation::create_feature_group::CreateFeatureGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_feature_group::CreateFeatureGroupError,
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
        crate::operation::create_feature_group::CreateFeatureGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_feature_group::CreateFeatureGroupError,
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
        crate::operation::create_feature_group::CreateFeatureGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_feature_group::CreateFeatureGroupError,
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
            crate::operation::create_feature_group::CreateFeatureGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_feature_group::CreateFeatureGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the <code>FeatureGroup</code>. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account. The name:</p>
    /// <ul>
    /// <li> <p>Must start and end with an alphanumeric character.</p> </li>
    /// <li> <p>Can only contain alphanumeric character and hyphens. Spaces are not allowed. </p> </li>
    /// </ul>
    pub fn feature_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.feature_group_name(input.into());
        self
    }
    /// <p>The name of the <code>FeatureGroup</code>. The name must be unique within an Amazon Web Services Region in an Amazon Web Services account. The name:</p>
    /// <ul>
    /// <li> <p>Must start and end with an alphanumeric character.</p> </li>
    /// <li> <p>Can only contain alphanumeric character and hyphens. Spaces are not allowed. </p> </li>
    /// </ul>
    pub fn set_feature_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_feature_group_name(input);
        self
    }
    /// <p>The name of the <code>Feature</code> whose value uniquely identifies a <code>Record</code> defined in the <code>FeatureStore</code>. Only the latest record per identifier value will be stored in the <code>OnlineStore</code>. <code>RecordIdentifierFeatureName</code> must be one of feature definitions' names.</p>
    /// <p>You use the <code>RecordIdentifierFeatureName</code> to access data in a <code>FeatureStore</code>.</p>
    /// <p>This name:</p>
    /// <ul>
    /// <li> <p>Must start and end with an alphanumeric character.</p> </li>
    /// <li> <p>Can only contains alphanumeric characters, hyphens, underscores. Spaces are not allowed. </p> </li>
    /// </ul>
    pub fn record_identifier_feature_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.record_identifier_feature_name(input.into());
        self
    }
    /// <p>The name of the <code>Feature</code> whose value uniquely identifies a <code>Record</code> defined in the <code>FeatureStore</code>. Only the latest record per identifier value will be stored in the <code>OnlineStore</code>. <code>RecordIdentifierFeatureName</code> must be one of feature definitions' names.</p>
    /// <p>You use the <code>RecordIdentifierFeatureName</code> to access data in a <code>FeatureStore</code>.</p>
    /// <p>This name:</p>
    /// <ul>
    /// <li> <p>Must start and end with an alphanumeric character.</p> </li>
    /// <li> <p>Can only contains alphanumeric characters, hyphens, underscores. Spaces are not allowed. </p> </li>
    /// </ul>
    pub fn set_record_identifier_feature_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_record_identifier_feature_name(input);
        self
    }
    /// <p>The name of the feature that stores the <code>EventTime</code> of a <code>Record</code> in a <code>FeatureGroup</code>.</p>
    /// <p>An <code>EventTime</code> is a point in time when a new event occurs that corresponds to the creation or update of a <code>Record</code> in a <code>FeatureGroup</code>. All <code>Records</code> in the <code>FeatureGroup</code> must have a corresponding <code>EventTime</code>.</p>
    /// <p>An <code>EventTime</code> can be a <code>String</code> or <code>Fractional</code>. </p>
    /// <ul>
    /// <li> <p> <code>Fractional</code>: <code>EventTime</code> feature values must be a Unix timestamp in seconds.</p> </li>
    /// <li> <p> <code>String</code>: <code>EventTime</code> feature values must be an ISO-8601 string in the format. The following formats are supported <code>yyyy-MM-dd'T'HH:mm:ssZ</code> and <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code> where <code>yyyy</code>, <code>MM</code>, and <code>dd</code> represent the year, month, and day respectively and <code>HH</code>, <code>mm</code>, <code>ss</code>, and if applicable, <code>SSS</code> represent the hour, month, second and milliseconds respsectively. <code>'T'</code> and <code>Z</code> are constants.</p> </li>
    /// </ul>
    pub fn event_time_feature_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.event_time_feature_name(input.into());
        self
    }
    /// <p>The name of the feature that stores the <code>EventTime</code> of a <code>Record</code> in a <code>FeatureGroup</code>.</p>
    /// <p>An <code>EventTime</code> is a point in time when a new event occurs that corresponds to the creation or update of a <code>Record</code> in a <code>FeatureGroup</code>. All <code>Records</code> in the <code>FeatureGroup</code> must have a corresponding <code>EventTime</code>.</p>
    /// <p>An <code>EventTime</code> can be a <code>String</code> or <code>Fractional</code>. </p>
    /// <ul>
    /// <li> <p> <code>Fractional</code>: <code>EventTime</code> feature values must be a Unix timestamp in seconds.</p> </li>
    /// <li> <p> <code>String</code>: <code>EventTime</code> feature values must be an ISO-8601 string in the format. The following formats are supported <code>yyyy-MM-dd'T'HH:mm:ssZ</code> and <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code> where <code>yyyy</code>, <code>MM</code>, and <code>dd</code> represent the year, month, and day respectively and <code>HH</code>, <code>mm</code>, <code>ss</code>, and if applicable, <code>SSS</code> represent the hour, month, second and milliseconds respsectively. <code>'T'</code> and <code>Z</code> are constants.</p> </li>
    /// </ul>
    pub fn set_event_time_feature_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_event_time_feature_name(input);
        self
    }
    /// Appends an item to `FeatureDefinitions`.
    ///
    /// To override the contents of this collection use [`set_feature_definitions`](Self::set_feature_definitions).
    ///
    /// <p>A list of <code>Feature</code> names and types. <code>Name</code> and <code>Type</code> is compulsory per <code>Feature</code>. </p>
    /// <p>Valid feature <code>FeatureType</code>s are <code>Integral</code>, <code>Fractional</code> and <code>String</code>.</p>
    /// <p> <code>FeatureName</code>s cannot be any of the following: <code>is_deleted</code>, <code>write_time</code>, <code>api_invocation_time</code> </p>
    /// <p>You can create up to 2,500 <code>FeatureDefinition</code>s per <code>FeatureGroup</code>.</p>
    pub fn feature_definitions(mut self, input: crate::types::FeatureDefinition) -> Self {
        self.inner = self.inner.feature_definitions(input);
        self
    }
    /// <p>A list of <code>Feature</code> names and types. <code>Name</code> and <code>Type</code> is compulsory per <code>Feature</code>. </p>
    /// <p>Valid feature <code>FeatureType</code>s are <code>Integral</code>, <code>Fractional</code> and <code>String</code>.</p>
    /// <p> <code>FeatureName</code>s cannot be any of the following: <code>is_deleted</code>, <code>write_time</code>, <code>api_invocation_time</code> </p>
    /// <p>You can create up to 2,500 <code>FeatureDefinition</code>s per <code>FeatureGroup</code>.</p>
    pub fn set_feature_definitions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FeatureDefinition>>,
    ) -> Self {
        self.inner = self.inner.set_feature_definitions(input);
        self
    }
    /// <p>You can turn the <code>OnlineStore</code> on or off by specifying <code>True</code> for the <code>EnableOnlineStore</code> flag in <code>OnlineStoreConfig</code>.</p>
    /// <p>You can also include an Amazon Web Services KMS key ID (<code>KMSKeyId</code>) for at-rest encryption of the <code>OnlineStore</code>.</p>
    /// <p>The default value is <code>False</code>.</p>
    pub fn online_store_config(mut self, input: crate::types::OnlineStoreConfig) -> Self {
        self.inner = self.inner.online_store_config(input);
        self
    }
    /// <p>You can turn the <code>OnlineStore</code> on or off by specifying <code>True</code> for the <code>EnableOnlineStore</code> flag in <code>OnlineStoreConfig</code>.</p>
    /// <p>You can also include an Amazon Web Services KMS key ID (<code>KMSKeyId</code>) for at-rest encryption of the <code>OnlineStore</code>.</p>
    /// <p>The default value is <code>False</code>.</p>
    pub fn set_online_store_config(
        mut self,
        input: ::std::option::Option<crate::types::OnlineStoreConfig>,
    ) -> Self {
        self.inner = self.inner.set_online_store_config(input);
        self
    }
    /// <p>Use this to configure an <code>OfflineFeatureStore</code>. This parameter allows you to specify:</p>
    /// <ul>
    /// <li> <p>The Amazon Simple Storage Service (Amazon S3) location of an <code>OfflineStore</code>.</p> </li>
    /// <li> <p>A configuration for an Amazon Web Services Glue or Amazon Web Services Hive data catalog. </p> </li>
    /// <li> <p>An KMS encryption key to encrypt the Amazon S3 location used for <code>OfflineStore</code>. If KMS encryption key is not specified, by default we encrypt all data at rest using Amazon Web Services KMS key. By defining your <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucket-key.html">bucket-level key</a> for SSE, you can reduce Amazon Web Services KMS requests costs by up to 99 percent.</p> </li>
    /// <li> <p>Format for the offline store table. Supported formats are Glue (Default) and <a href="https://iceberg.apache.org/">Apache Iceberg</a>.</p> </li>
    /// </ul>
    /// <p>To learn more about this parameter, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OfflineStoreConfig.html">OfflineStoreConfig</a>.</p>
    pub fn offline_store_config(mut self, input: crate::types::OfflineStoreConfig) -> Self {
        self.inner = self.inner.offline_store_config(input);
        self
    }
    /// <p>Use this to configure an <code>OfflineFeatureStore</code>. This parameter allows you to specify:</p>
    /// <ul>
    /// <li> <p>The Amazon Simple Storage Service (Amazon S3) location of an <code>OfflineStore</code>.</p> </li>
    /// <li> <p>A configuration for an Amazon Web Services Glue or Amazon Web Services Hive data catalog. </p> </li>
    /// <li> <p>An KMS encryption key to encrypt the Amazon S3 location used for <code>OfflineStore</code>. If KMS encryption key is not specified, by default we encrypt all data at rest using Amazon Web Services KMS key. By defining your <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucket-key.html">bucket-level key</a> for SSE, you can reduce Amazon Web Services KMS requests costs by up to 99 percent.</p> </li>
    /// <li> <p>Format for the offline store table. Supported formats are Glue (Default) and <a href="https://iceberg.apache.org/">Apache Iceberg</a>.</p> </li>
    /// </ul>
    /// <p>To learn more about this parameter, see <a href="https://docs.aws.amazon.com/sagemaker/latest/APIReference/API_OfflineStoreConfig.html">OfflineStoreConfig</a>.</p>
    pub fn set_offline_store_config(
        mut self,
        input: ::std::option::Option<crate::types::OfflineStoreConfig>,
    ) -> Self {
        self.inner = self.inner.set_offline_store_config(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the <code>OfflineStore</code> if an <code>OfflineStoreConfig</code> is provided.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM execution role used to persist data into the <code>OfflineStore</code> if an <code>OfflineStoreConfig</code> is provided.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>A free-form description of a <code>FeatureGroup</code>.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A free-form description of a <code>FeatureGroup</code>.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags used to identify <code>Features</code> in each <code>FeatureGroup</code>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Tags used to identify <code>Features</code> in each <code>FeatureGroup</code>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
