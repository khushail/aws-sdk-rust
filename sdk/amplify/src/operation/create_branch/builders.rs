// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_branch::_create_branch_output::CreateBranchOutputBuilder;

pub use crate::operation::create_branch::_create_branch_input::CreateBranchInputBuilder;

/// Fluent builder constructing a request to `CreateBranch`.
///
/// <p> Creates a new branch for an Amplify app. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateBranchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_branch::builders::CreateBranchInputBuilder,
}
impl CreateBranchFluentBuilder {
    /// Creates a new `CreateBranch`.
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
            crate::operation::create_branch::CreateBranch,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_branch::CreateBranchError>,
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
        crate::operation::create_branch::CreateBranchOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_branch::CreateBranchError>,
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
        crate::operation::create_branch::CreateBranchOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::create_branch::CreateBranchError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_branch::CreateBranch,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_branch::CreateBranchError>,
    > {
        self.customize_middleware().await
    }
    /// <p> The unique ID for an Amplify app. </p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p> The unique ID for an Amplify app. </p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p> The name for the branch. </p>
    pub fn branch_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.branch_name(input.into());
        self
    }
    /// <p> The name for the branch. </p>
    pub fn set_branch_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_branch_name(input);
        self
    }
    /// <p> The description for the branch. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p> The description for the branch. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p> Describes the current stage for the branch. </p>
    pub fn stage(mut self, input: crate::types::Stage) -> Self {
        self.inner = self.inner.stage(input);
        self
    }
    /// <p> Describes the current stage for the branch. </p>
    pub fn set_stage(mut self, input: ::std::option::Option<crate::types::Stage>) -> Self {
        self.inner = self.inner.set_stage(input);
        self
    }
    /// <p> The framework for the branch. </p>
    pub fn framework(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.framework(input.into());
        self
    }
    /// <p> The framework for the branch. </p>
    pub fn set_framework(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_framework(input);
        self
    }
    /// <p> Enables notifications for the branch. </p>
    pub fn enable_notification(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_notification(input);
        self
    }
    /// <p> Enables notifications for the branch. </p>
    pub fn set_enable_notification(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_notification(input);
        self
    }
    /// <p> Enables auto building for the branch. </p>
    pub fn enable_auto_build(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_auto_build(input);
        self
    }
    /// <p> Enables auto building for the branch. </p>
    pub fn set_enable_auto_build(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_auto_build(input);
        self
    }
    /// Adds a key-value pair to `environmentVariables`.
    ///
    /// To override the contents of this collection use [`set_environment_variables`](Self::set_environment_variables).
    ///
    /// <p> The environment variables for the branch. </p>
    pub fn environment_variables(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.environment_variables(k.into(), v.into());
        self
    }
    /// <p> The environment variables for the branch. </p>
    pub fn set_environment_variables(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_environment_variables(input);
        self
    }
    /// <p> The basic authorization credentials for the branch. You must base64-encode the authorization credentials and provide them in the format <code>user:password</code>.</p>
    pub fn basic_auth_credentials(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.basic_auth_credentials(input.into());
        self
    }
    /// <p> The basic authorization credentials for the branch. You must base64-encode the authorization credentials and provide them in the format <code>user:password</code>.</p>
    pub fn set_basic_auth_credentials(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_basic_auth_credentials(input);
        self
    }
    /// <p> Enables basic authorization for the branch. </p>
    pub fn enable_basic_auth(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_basic_auth(input);
        self
    }
    /// <p> Enables basic authorization for the branch. </p>
    pub fn set_enable_basic_auth(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_basic_auth(input);
        self
    }
    /// <p>Enables performance mode for the branch.</p>
    /// <p>Performance mode optimizes for faster hosting performance by keeping content cached at the edge for a longer interval. When performance mode is enabled, hosting configuration or code changes can take up to 10 minutes to roll out. </p>
    pub fn enable_performance_mode(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_performance_mode(input);
        self
    }
    /// <p>Enables performance mode for the branch.</p>
    /// <p>Performance mode optimizes for faster hosting performance by keeping content cached at the edge for a longer interval. When performance mode is enabled, hosting configuration or code changes can take up to 10 minutes to roll out. </p>
    pub fn set_enable_performance_mode(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_performance_mode(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p> The tag for the branch. </p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p> The tag for the branch. </p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p> The build specification (build spec) for the branch. </p>
    pub fn build_spec(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.build_spec(input.into());
        self
    }
    /// <p> The build specification (build spec) for the branch. </p>
    pub fn set_build_spec(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_build_spec(input);
        self
    }
    /// <p> The content Time To Live (TTL) for the website in seconds. </p>
    pub fn ttl(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ttl(input.into());
        self
    }
    /// <p> The content Time To Live (TTL) for the website in seconds. </p>
    pub fn set_ttl(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ttl(input);
        self
    }
    /// <p> The display name for a branch. This is used as the default domain prefix. </p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p> The display name for a branch. This is used as the default domain prefix. </p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p> Enables pull request previews for this branch. </p>
    pub fn enable_pull_request_preview(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_pull_request_preview(input);
        self
    }
    /// <p> Enables pull request previews for this branch. </p>
    pub fn set_enable_pull_request_preview(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_pull_request_preview(input);
        self
    }
    /// <p> The Amplify environment name for the pull request. </p>
    pub fn pull_request_environment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.pull_request_environment_name(input.into());
        self
    }
    /// <p> The Amplify environment name for the pull request. </p>
    pub fn set_pull_request_environment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pull_request_environment_name(input);
        self
    }
    /// <p> The Amazon Resource Name (ARN) for a backend environment that is part of an Amplify app. </p>
    pub fn backend_environment_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.backend_environment_arn(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) for a backend environment that is part of an Amplify app. </p>
    pub fn set_backend_environment_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_backend_environment_arn(input);
        self
    }
}
