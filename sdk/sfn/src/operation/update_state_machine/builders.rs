// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_state_machine::_update_state_machine_output::UpdateStateMachineOutputBuilder;

pub use crate::operation::update_state_machine::_update_state_machine_input::UpdateStateMachineInputBuilder;

/// Fluent builder constructing a request to `UpdateStateMachine`.
///
/// <p>Updates an existing state machine by modifying its <code>definition</code>, <code>roleArn</code>, or <code>loggingConfiguration</code>. Running executions will continue to use the previous <code>definition</code> and <code>roleArn</code>. You must include at least one of <code>definition</code> or <code>roleArn</code> or you will receive a <code>MissingRequiredParameter</code> error.</p>
/// <p>If the given state machine Amazon Resource Name (ARN) is a qualified state machine ARN, it will fail with ValidationException.</p>
/// <p>A qualified state machine ARN refers to a <i>Distributed Map state</i> defined within a state machine. For example, the qualified state machine ARN <code>arn:partition:states:region:account-id:stateMachine:stateMachineName/mapStateLabel</code> refers to a <i>Distributed Map state</i> with a label <code>mapStateLabel</code> in the state machine named <code>stateMachineName</code>.</p> <note>
/// <p>All <code>StartExecution</code> calls within a few seconds will use the updated <code>definition</code> and <code>roleArn</code>. Executions started immediately after calling <code>UpdateStateMachine</code> may use the previous state machine <code>definition</code> and <code>roleArn</code>. </p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateStateMachineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_state_machine::builders::UpdateStateMachineInputBuilder,
}
impl UpdateStateMachineFluentBuilder {
    /// Creates a new `UpdateStateMachine`.
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
            crate::operation::update_state_machine::UpdateStateMachine,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_state_machine::UpdateStateMachineError,
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
        crate::operation::update_state_machine::UpdateStateMachineOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_state_machine::UpdateStateMachineError,
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
        crate::operation::update_state_machine::UpdateStateMachineOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_state_machine::UpdateStateMachineError,
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
            crate::operation::update_state_machine::UpdateStateMachine,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_state_machine::UpdateStateMachineError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The Amazon Resource Name (ARN) of the state machine.</p>
    pub fn state_machine_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.state_machine_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the state machine.</p>
    pub fn set_state_machine_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_state_machine_arn(input);
        self
    }
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    pub fn definition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.definition(input.into());
        self
    }
    /// <p>The Amazon States Language definition of the state machine. See <a href="https://docs.aws.amazon.com/step-functions/latest/dg/concepts-amazon-states-language.html">Amazon States Language</a>.</p>
    pub fn set_definition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_definition(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role of the state machine.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role of the state machine.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    pub fn logging_configuration(mut self, input: crate::types::LoggingConfiguration) -> Self {
        self.inner = self.inner.logging_configuration(input);
        self
    }
    /// <p>The <code>LoggingConfiguration</code> data type is used to set CloudWatch Logs options.</p>
    pub fn set_logging_configuration(
        mut self,
        input: ::std::option::Option<crate::types::LoggingConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_logging_configuration(input);
        self
    }
    /// <p>Selects whether X-Ray tracing is enabled.</p>
    pub fn tracing_configuration(mut self, input: crate::types::TracingConfiguration) -> Self {
        self.inner = self.inner.tracing_configuration(input);
        self
    }
    /// <p>Selects whether X-Ray tracing is enabled.</p>
    pub fn set_tracing_configuration(
        mut self,
        input: ::std::option::Option<crate::types::TracingConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_tracing_configuration(input);
        self
    }
}
