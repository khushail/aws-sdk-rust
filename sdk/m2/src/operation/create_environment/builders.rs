// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_environment::_create_environment_output::CreateEnvironmentOutputBuilder;

pub use crate::operation::create_environment::_create_environment_input::CreateEnvironmentInputBuilder;

/// Fluent builder constructing a request to `CreateEnvironment`.
///
/// <p>Creates a runtime environment for a given runtime engine.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateEnvironmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_environment::builders::CreateEnvironmentInputBuilder,
}
impl CreateEnvironmentFluentBuilder {
    /// Creates a new `CreateEnvironment`.
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
            crate::operation::create_environment::CreateEnvironment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_environment::CreateEnvironmentError,
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
        crate::operation::create_environment::CreateEnvironmentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_environment::CreateEnvironmentError,
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
        crate::operation::create_environment::CreateEnvironmentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_environment::CreateEnvironmentError,
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
            crate::operation::create_environment::CreateEnvironment,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_environment::CreateEnvironmentError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the runtime environment. Must be unique within the account.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the runtime environment. Must be unique within the account.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The type of instance for the runtime environment.</p>
    pub fn instance_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.instance_type(input.into());
        self
    }
    /// <p>The type of instance for the runtime environment.</p>
    pub fn set_instance_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>The description of the runtime environment.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the runtime environment.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The engine type for the runtime environment.</p>
    pub fn engine_type(mut self, input: crate::types::EngineType) -> Self {
        self.inner = self.inner.engine_type(input);
        self
    }
    /// <p>The engine type for the runtime environment.</p>
    pub fn set_engine_type(
        mut self,
        input: ::std::option::Option<crate::types::EngineType>,
    ) -> Self {
        self.inner = self.inner.set_engine_type(input);
        self
    }
    /// <p>The version of the engine type for the runtime environment.</p>
    pub fn engine_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The version of the engine type for the runtime environment.</p>
    pub fn set_engine_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// Appends an item to `subnetIds`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The list of subnets associated with the VPC for this runtime environment.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_ids(input.into());
        self
    }
    /// <p>The list of subnets associated with the VPC for this runtime environment.</p>
    pub fn set_subnet_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_subnet_ids(input);
        self
    }
    /// Appends an item to `securityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The list of security groups for the VPC associated with this runtime environment.</p>
    pub fn security_group_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>The list of security groups for the VPC associated with this runtime environment.</p>
    pub fn set_security_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// Appends an item to `storageConfigurations`.
    ///
    /// To override the contents of this collection use [`set_storage_configurations`](Self::set_storage_configurations).
    ///
    /// <p>Optional. The storage configurations for this runtime environment.</p>
    pub fn storage_configurations(mut self, input: crate::types::StorageConfiguration) -> Self {
        self.inner = self.inner.storage_configurations(input);
        self
    }
    /// <p>Optional. The storage configurations for this runtime environment.</p>
    pub fn set_storage_configurations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StorageConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_storage_configurations(input);
        self
    }
    /// <p>Specifies whether the runtime environment is publicly accessible.</p>
    pub fn publicly_accessible(mut self, input: bool) -> Self {
        self.inner = self.inner.publicly_accessible(input);
        self
    }
    /// <p>Specifies whether the runtime environment is publicly accessible.</p>
    pub fn set_publicly_accessible(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_publicly_accessible(input);
        self
    }
    /// <p>The details of a high availability configuration for this runtime environment.</p>
    pub fn high_availability_config(mut self, input: crate::types::HighAvailabilityConfig) -> Self {
        self.inner = self.inner.high_availability_config(input);
        self
    }
    /// <p>The details of a high availability configuration for this runtime environment.</p>
    pub fn set_high_availability_config(
        mut self,
        input: ::std::option::Option<crate::types::HighAvailabilityConfig>,
    ) -> Self {
        self.inner = self.inner.set_high_availability_config(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the runtime environment.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags for the runtime environment.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Configures the maintenance window you want for the runtime environment. If you do not provide a value, a random system-generated value will be assigned.</p>
    pub fn preferred_maintenance_window(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.preferred_maintenance_window(input.into());
        self
    }
    /// <p>Configures the maintenance window you want for the runtime environment. If you do not provide a value, a random system-generated value will be assigned.</p>
    pub fn set_preferred_maintenance_window(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_preferred_maintenance_window(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request to create an environment. The service generates the clientToken when the API call is triggered. The token expires after one hour, so if you retry the API within this timeframe with the same clientToken, you will get the same response. The service also handles deleting the clientToken after it expires. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request to create an environment. The service generates the clientToken when the API call is triggered. The token expires after one hour, so if you retry the API within this timeframe with the same clientToken, you will get the same response. The service also handles deleting the clientToken after it expires. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The identifier of a customer managed key.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The identifier of a customer managed key.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
}
