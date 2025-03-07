// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_option_group::_modify_option_group_output::ModifyOptionGroupOutputBuilder;

pub use crate::operation::modify_option_group::_modify_option_group_input::ModifyOptionGroupInputBuilder;

/// Fluent builder constructing a request to `ModifyOptionGroup`.
///
/// <p>Modifies an existing option group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyOptionGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_option_group::builders::ModifyOptionGroupInputBuilder,
}
impl ModifyOptionGroupFluentBuilder {
    /// Creates a new `ModifyOptionGroup`.
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
            crate::operation::modify_option_group::ModifyOptionGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_option_group::ModifyOptionGroupError,
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
        crate::operation::modify_option_group::ModifyOptionGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_option_group::ModifyOptionGroupError,
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
        crate::operation::modify_option_group::ModifyOptionGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_option_group::ModifyOptionGroupError,
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
            crate::operation::modify_option_group::ModifyOptionGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::modify_option_group::ModifyOptionGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the option group to be modified.</p>
    /// <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed from an option group, and that option group can't be removed from a DB instance once it is associated with a DB instance</p>
    pub fn option_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.option_group_name(input.into());
        self
    }
    /// <p>The name of the option group to be modified.</p>
    /// <p>Permanent options, such as the TDE option for Oracle Advanced Security TDE, can't be removed from an option group, and that option group can't be removed from a DB instance once it is associated with a DB instance</p>
    pub fn set_option_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_option_group_name(input);
        self
    }
    /// Appends an item to `OptionsToInclude`.
    ///
    /// To override the contents of this collection use [`set_options_to_include`](Self::set_options_to_include).
    ///
    /// <p>Options in this list are added to the option group or, if already present, the specified configuration is used to update the existing configuration.</p>
    pub fn options_to_include(mut self, input: crate::types::OptionConfiguration) -> Self {
        self.inner = self.inner.options_to_include(input);
        self
    }
    /// <p>Options in this list are added to the option group or, if already present, the specified configuration is used to update the existing configuration.</p>
    pub fn set_options_to_include(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::OptionConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_options_to_include(input);
        self
    }
    /// Appends an item to `OptionsToRemove`.
    ///
    /// To override the contents of this collection use [`set_options_to_remove`](Self::set_options_to_remove).
    ///
    /// <p>Options in this list are removed from the option group.</p>
    pub fn options_to_remove(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.options_to_remove(input.into());
        self
    }
    /// <p>Options in this list are removed from the option group.</p>
    pub fn set_options_to_remove(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_options_to_remove(input);
        self
    }
    /// <p>A value that indicates whether to apply the change immediately or during the next maintenance window for each instance associated with the option group.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>A value that indicates whether to apply the change immediately or during the next maintenance window for each instance associated with the option group.</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
}
