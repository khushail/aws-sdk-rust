// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::rotate_secret::_rotate_secret_output::RotateSecretOutputBuilder;

pub use crate::operation::rotate_secret::_rotate_secret_input::RotateSecretInputBuilder;

/// Fluent builder constructing a request to `RotateSecret`.
///
/// <p>Configures and starts the asynchronous process of rotating the secret. For information about rotation, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets.html">Rotate secrets</a> in the <i>Secrets Manager User Guide</i>. If you include the configuration parameters, the operation sets the values for the secret and then immediately starts a rotation. If you don't include the configuration parameters, the operation starts a rotation with the values already stored in the secret. </p>
/// <p>When rotation is successful, the <code>AWSPENDING</code> staging label might be attached to the same version as the <code>AWSCURRENT</code> version, or it might not be attached to any version. If the <code>AWSPENDING</code> staging label is present but not attached to the same version as <code>AWSCURRENT</code>, then any later invocation of <code>RotateSecret</code> assumes that a previous rotation request is still in progress and returns an error. When rotation is unsuccessful, the <code>AWSPENDING</code> staging label might be attached to an empty secret version. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot_rotation.html">Troubleshoot rotation</a> in the <i>Secrets Manager User Guide</i>.</p>
/// <p>Secrets Manager generates a CloudTrail log entry when you call this action. Do not include sensitive information in request parameters because it might be logged. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/retrieve-ct-entries.html">Logging Secrets Manager events with CloudTrail</a>.</p>
/// <p> <b>Required permissions: </b> <code>secretsmanager:RotateSecret</code>. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/reference_iam-permissions.html#reference_iam-permissions_actions"> IAM policy actions for Secrets Manager</a> and <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/auth-and-access.html">Authentication and access control in Secrets Manager</a>. You also need <code>lambda:InvokeFunction</code> permissions on the rotation function. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotating-secrets-required-permissions-function.html"> Permissions for rotation</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RotateSecretFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::rotate_secret::builders::RotateSecretInputBuilder,
}
impl RotateSecretFluentBuilder {
    /// Creates a new `RotateSecret`.
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
            crate::operation::rotate_secret::RotateSecret,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::rotate_secret::RotateSecretError>,
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
        crate::operation::rotate_secret::RotateSecretOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::rotate_secret::RotateSecretError>,
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
        crate::operation::rotate_secret::RotateSecretOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::rotate_secret::RotateSecretError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::rotate_secret::RotateSecret,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::rotate_secret::RotateSecretError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ARN or name of the secret to rotate.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn secret_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.secret_id(input.into());
        self
    }
    /// <p>The ARN or name of the secret to rotate.</p>
    /// <p>For an ARN, we recommend that you specify a complete ARN rather than a partial ARN. See <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/troubleshoot.html#ARN_secretnamehyphen">Finding a secret from a partial ARN</a>.</p>
    pub fn set_secret_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_secret_id(input);
        self
    }
    /// <p>A unique identifier for the new version of the secret that helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during rotation. This value becomes the <code>VersionId</code> of the new version.</p>
    /// <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request for this parameter. If you don't use the SDK and instead generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> yourself for new versions and include that value in the request.</p>
    /// <p>You only need to specify this value if you implement your own retry logic and you want to ensure that Secrets Manager doesn't attempt to create a secret version twice. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness within the specified secret. </p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>A unique identifier for the new version of the secret that helps ensure idempotency. Secrets Manager uses this value to prevent the accidental creation of duplicate versions if there are failures and retries during rotation. This value becomes the <code>VersionId</code> of the new version.</p>
    /// <p>If you use the Amazon Web Services CLI or one of the Amazon Web Services SDK to call this operation, then you can leave this parameter empty. The CLI or SDK generates a random UUID for you and includes that in the request for this parameter. If you don't use the SDK and instead generate a raw HTTP request to the Secrets Manager service endpoint, then you must generate a <code>ClientRequestToken</code> yourself for new versions and include that value in the request.</p>
    /// <p>You only need to specify this value if you implement your own retry logic and you want to ensure that Secrets Manager doesn't attempt to create a secret version twice. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type</a> value to ensure uniqueness within the specified secret. </p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>For secrets that use a Lambda rotation function to rotate, the ARN of the Lambda rotation function. </p>
    /// <p>For secrets that use <i>managed rotation</i>, omit this field. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_managed.html">Managed rotation</a> in the <i>Secrets Manager User Guide</i>.</p>
    pub fn rotation_lambda_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.rotation_lambda_arn(input.into());
        self
    }
    /// <p>For secrets that use a Lambda rotation function to rotate, the ARN of the Lambda rotation function. </p>
    /// <p>For secrets that use <i>managed rotation</i>, omit this field. For more information, see <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_managed.html">Managed rotation</a> in the <i>Secrets Manager User Guide</i>.</p>
    pub fn set_rotation_lambda_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_rotation_lambda_arn(input);
        self
    }
    /// <p>A structure that defines the rotation configuration for this secret.</p>
    pub fn rotation_rules(mut self, input: crate::types::RotationRulesType) -> Self {
        self.inner = self.inner.rotation_rules(input);
        self
    }
    /// <p>A structure that defines the rotation configuration for this secret.</p>
    pub fn set_rotation_rules(
        mut self,
        input: ::std::option::Option<crate::types::RotationRulesType>,
    ) -> Self {
        self.inner = self.inner.set_rotation_rules(input);
        self
    }
    /// <p>Specifies whether to rotate the secret immediately or wait until the next scheduled rotation window. The rotation schedule is defined in <code>RotateSecretRequest$RotationRules</code>.</p>
    /// <p>For secrets that use a Lambda rotation function to rotate, if you don't immediately rotate the secret, Secrets Manager tests the rotation configuration by running the <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html"> <code>testSecret</code> step</a> of the Lambda rotation function. The test creates an <code>AWSPENDING</code> version of the secret and then removes it.</p>
    /// <p>By default, Secrets Manager rotates the secret immediately.</p>
    pub fn rotate_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.rotate_immediately(input);
        self
    }
    /// <p>Specifies whether to rotate the secret immediately or wait until the next scheduled rotation window. The rotation schedule is defined in <code>RotateSecretRequest$RotationRules</code>.</p>
    /// <p>For secrets that use a Lambda rotation function to rotate, if you don't immediately rotate the secret, Secrets Manager tests the rotation configuration by running the <a href="https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_how.html"> <code>testSecret</code> step</a> of the Lambda rotation function. The test creates an <code>AWSPENDING</code> version of the secret and then removes it.</p>
    /// <p>By default, Secrets Manager rotates the secret immediately.</p>
    pub fn set_rotate_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_rotate_immediately(input);
        self
    }
}
