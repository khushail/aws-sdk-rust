// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_service_instance::_update_service_instance_output::UpdateServiceInstanceOutputBuilder;

pub use crate::operation::update_service_instance::_update_service_instance_input::UpdateServiceInstanceInputBuilder;

/// Fluent builder constructing a request to `UpdateServiceInstance`.
///
/// <p>Update a service instance.</p>
/// <p>There are a few modes for updating a service instance. The <code>deploymentType</code> field defines the mode.</p> <note>
/// <p>You can't update a service instance while its deployment status, or the deployment status of a component attached to it, is <code>IN_PROGRESS</code>.</p>
/// <p>For more information about components, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-components.html">Proton components</a> in the <i>Proton User Guide</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateServiceInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_service_instance::builders::UpdateServiceInstanceInputBuilder,
}
impl UpdateServiceInstanceFluentBuilder {
    /// Creates a new `UpdateServiceInstance`.
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
            crate::operation::update_service_instance::UpdateServiceInstance,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_instance::UpdateServiceInstanceError,
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
        crate::operation::update_service_instance::UpdateServiceInstanceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_instance::UpdateServiceInstanceError,
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
        crate::operation::update_service_instance::UpdateServiceInstanceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_instance::UpdateServiceInstanceError,
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
            crate::operation::update_service_instance::UpdateServiceInstance,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_service_instance::UpdateServiceInstanceError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the service instance to update.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the service instance to update.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the service that the service instance belongs to.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the service that the service instance belongs to.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// <p>The deployment type. It defines the mode for updating a service instance, as follows:</p>
    /// <dl>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>NONE</code> </p>
    /// <p>In this mode, a deployment <i>doesn't</i> occur. Only the requested metadata parameters are updated.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>CURRENT_VERSION</code> </p>
    /// <p>In this mode, the service instance is deployed and updated with the new spec that you provide. Only requested parameters are updated. <i>Don’t</i> include major or minor version parameters when you use this deployment type.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>MINOR_VERSION</code> </p>
    /// <p>In this mode, the service instance is deployed and updated with the published, recommended (latest) minor version of the current major version in use, by default. You can also specify a different minor version of the current major version in use.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>MAJOR_VERSION</code> </p>
    /// <p>In this mode, the service instance is deployed and updated with the published, recommended (latest) major and minor version of the current template, by default. You can specify a different major version that's higher than the major version in use and a minor version.</p>
    /// </dd>
    /// </dl>
    pub fn deployment_type(mut self, input: crate::types::DeploymentUpdateType) -> Self {
        self.inner = self.inner.deployment_type(input);
        self
    }
    /// <p>The deployment type. It defines the mode for updating a service instance, as follows:</p>
    /// <dl>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>NONE</code> </p>
    /// <p>In this mode, a deployment <i>doesn't</i> occur. Only the requested metadata parameters are updated.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>CURRENT_VERSION</code> </p>
    /// <p>In this mode, the service instance is deployed and updated with the new spec that you provide. Only requested parameters are updated. <i>Don’t</i> include major or minor version parameters when you use this deployment type.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>MINOR_VERSION</code> </p>
    /// <p>In this mode, the service instance is deployed and updated with the published, recommended (latest) minor version of the current major version in use, by default. You can also specify a different minor version of the current major version in use.</p>
    /// </dd>
    /// <dt></dt>
    /// <dd>
    /// <p> <code>MAJOR_VERSION</code> </p>
    /// <p>In this mode, the service instance is deployed and updated with the published, recommended (latest) major and minor version of the current template, by default. You can specify a different major version that's higher than the major version in use and a minor version.</p>
    /// </dd>
    /// </dl>
    pub fn set_deployment_type(
        mut self,
        input: ::std::option::Option<crate::types::DeploymentUpdateType>,
    ) -> Self {
        self.inner = self.inner.set_deployment_type(input);
        self
    }
    /// <p>The formatted specification that defines the service instance update.</p>
    pub fn spec(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.spec(input.into());
        self
    }
    /// <p>The formatted specification that defines the service instance update.</p>
    pub fn set_spec(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_spec(input);
        self
    }
    /// <p>The major version of the service template to update.</p>
    pub fn template_major_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.template_major_version(input.into());
        self
    }
    /// <p>The major version of the service template to update.</p>
    pub fn set_template_major_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_template_major_version(input);
        self
    }
    /// <p>The minor version of the service template to update.</p>
    pub fn template_minor_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.template_minor_version(input.into());
        self
    }
    /// <p>The minor version of the service template to update.</p>
    pub fn set_template_minor_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_template_minor_version(input);
        self
    }
    /// <p>The client token of the service instance to update.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client token of the service instance to update.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
