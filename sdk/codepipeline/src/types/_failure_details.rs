// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents information about failure details.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FailureDetails {
    /// <p>The type of the failure.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::FailureType>,
    /// <p>The message about the failure.</p>
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// <p>The external ID of the run of the action that failed.</p>
    #[doc(hidden)]
    pub external_execution_id: ::std::option::Option<::std::string::String>,
}
impl FailureDetails {
    /// <p>The type of the failure.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::FailureType> {
        self.r#type.as_ref()
    }
    /// <p>The message about the failure.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>The external ID of the run of the action that failed.</p>
    pub fn external_execution_id(&self) -> ::std::option::Option<&str> {
        self.external_execution_id.as_deref()
    }
}
impl FailureDetails {
    /// Creates a new builder-style object to manufacture [`FailureDetails`](crate::types::FailureDetails).
    pub fn builder() -> crate::types::builders::FailureDetailsBuilder {
        crate::types::builders::FailureDetailsBuilder::default()
    }
}

/// A builder for [`FailureDetails`](crate::types::FailureDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailureDetailsBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::FailureType>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) external_execution_id: ::std::option::Option<::std::string::String>,
}
impl FailureDetailsBuilder {
    /// <p>The type of the failure.</p>
    pub fn r#type(mut self, input: crate::types::FailureType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the failure.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::FailureType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The message about the failure.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message about the failure.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The external ID of the run of the action that failed.</p>
    pub fn external_execution_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.external_execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The external ID of the run of the action that failed.</p>
    pub fn set_external_execution_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.external_execution_id = input;
        self
    }
    /// Consumes the builder and constructs a [`FailureDetails`](crate::types::FailureDetails).
    pub fn build(self) -> crate::types::FailureDetails {
        crate::types::FailureDetails {
            r#type: self.r#type,
            message: self.message,
            external_execution_id: self.external_execution_id,
        }
    }
}
