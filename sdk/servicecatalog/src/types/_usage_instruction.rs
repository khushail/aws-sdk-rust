// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Additional information provided by the administrator.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UsageInstruction {
    /// <p>The usage instruction type for the value.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
    /// <p>The usage instruction value for this type.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
}
impl UsageInstruction {
    /// <p>The usage instruction type for the value.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>The usage instruction value for this type.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl UsageInstruction {
    /// Creates a new builder-style object to manufacture [`UsageInstruction`](crate::types::UsageInstruction).
    pub fn builder() -> crate::types::builders::UsageInstructionBuilder {
        crate::types::builders::UsageInstructionBuilder::default()
    }
}

/// A builder for [`UsageInstruction`](crate::types::UsageInstruction).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UsageInstructionBuilder {
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl UsageInstructionBuilder {
    /// <p>The usage instruction type for the value.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The usage instruction type for the value.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The usage instruction value for this type.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The usage instruction value for this type.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`UsageInstruction`](crate::types::UsageInstruction).
    pub fn build(self) -> crate::types::UsageInstruction {
        crate::types::UsageInstruction {
            r#type: self.r#type,
            value: self.value,
        }
    }
}
