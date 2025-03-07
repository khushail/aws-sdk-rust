// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Settings that specify the dialog code hook that is called by Amazon Lex at a step of the conversation. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DialogCodeHookInvocationSetting {
    /// <p>Indicates whether a Lambda function should be invoked for the dialog.</p>
    #[doc(hidden)]
    pub enable_code_hook_invocation: ::std::option::Option<bool>,
    /// <p>Determines whether a dialog code hook is used when the intent is activated.</p>
    #[doc(hidden)]
    pub active: ::std::option::Option<bool>,
    /// <p>A label that indicates the dialog step from which the dialog code hook is happening.</p>
    #[doc(hidden)]
    pub invocation_label: ::std::option::Option<::std::string::String>,
    /// <p>Contains the responses and actions that Amazon Lex takes after the Lambda function is complete.</p>
    #[doc(hidden)]
    pub post_code_hook_specification:
        ::std::option::Option<crate::types::PostDialogCodeHookInvocationSpecification>,
}
impl DialogCodeHookInvocationSetting {
    /// <p>Indicates whether a Lambda function should be invoked for the dialog.</p>
    pub fn enable_code_hook_invocation(&self) -> ::std::option::Option<bool> {
        self.enable_code_hook_invocation
    }
    /// <p>Determines whether a dialog code hook is used when the intent is activated.</p>
    pub fn active(&self) -> ::std::option::Option<bool> {
        self.active
    }
    /// <p>A label that indicates the dialog step from which the dialog code hook is happening.</p>
    pub fn invocation_label(&self) -> ::std::option::Option<&str> {
        self.invocation_label.as_deref()
    }
    /// <p>Contains the responses and actions that Amazon Lex takes after the Lambda function is complete.</p>
    pub fn post_code_hook_specification(
        &self,
    ) -> ::std::option::Option<&crate::types::PostDialogCodeHookInvocationSpecification> {
        self.post_code_hook_specification.as_ref()
    }
}
impl DialogCodeHookInvocationSetting {
    /// Creates a new builder-style object to manufacture [`DialogCodeHookInvocationSetting`](crate::types::DialogCodeHookInvocationSetting).
    pub fn builder() -> crate::types::builders::DialogCodeHookInvocationSettingBuilder {
        crate::types::builders::DialogCodeHookInvocationSettingBuilder::default()
    }
}

/// A builder for [`DialogCodeHookInvocationSetting`](crate::types::DialogCodeHookInvocationSetting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DialogCodeHookInvocationSettingBuilder {
    pub(crate) enable_code_hook_invocation: ::std::option::Option<bool>,
    pub(crate) active: ::std::option::Option<bool>,
    pub(crate) invocation_label: ::std::option::Option<::std::string::String>,
    pub(crate) post_code_hook_specification:
        ::std::option::Option<crate::types::PostDialogCodeHookInvocationSpecification>,
}
impl DialogCodeHookInvocationSettingBuilder {
    /// <p>Indicates whether a Lambda function should be invoked for the dialog.</p>
    pub fn enable_code_hook_invocation(mut self, input: bool) -> Self {
        self.enable_code_hook_invocation = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether a Lambda function should be invoked for the dialog.</p>
    pub fn set_enable_code_hook_invocation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_code_hook_invocation = input;
        self
    }
    /// <p>Determines whether a dialog code hook is used when the intent is activated.</p>
    pub fn active(mut self, input: bool) -> Self {
        self.active = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines whether a dialog code hook is used when the intent is activated.</p>
    pub fn set_active(mut self, input: ::std::option::Option<bool>) -> Self {
        self.active = input;
        self
    }
    /// <p>A label that indicates the dialog step from which the dialog code hook is happening.</p>
    pub fn invocation_label(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.invocation_label = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A label that indicates the dialog step from which the dialog code hook is happening.</p>
    pub fn set_invocation_label(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.invocation_label = input;
        self
    }
    /// <p>Contains the responses and actions that Amazon Lex takes after the Lambda function is complete.</p>
    pub fn post_code_hook_specification(
        mut self,
        input: crate::types::PostDialogCodeHookInvocationSpecification,
    ) -> Self {
        self.post_code_hook_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains the responses and actions that Amazon Lex takes after the Lambda function is complete.</p>
    pub fn set_post_code_hook_specification(
        mut self,
        input: ::std::option::Option<crate::types::PostDialogCodeHookInvocationSpecification>,
    ) -> Self {
        self.post_code_hook_specification = input;
        self
    }
    /// Consumes the builder and constructs a [`DialogCodeHookInvocationSetting`](crate::types::DialogCodeHookInvocationSetting).
    pub fn build(self) -> crate::types::DialogCodeHookInvocationSetting {
        crate::types::DialogCodeHookInvocationSetting {
            enable_code_hook_invocation: self.enable_code_hook_invocation,
            active: self.active,
            invocation_label: self.invocation_label,
            post_code_hook_specification: self.post_code_hook_specification,
        }
    }
}
