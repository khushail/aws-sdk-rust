// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The error information, such as the error code and the timestamp.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetAssetPropertyValueErrorInfo {
    /// <p>The error code.</p>
    #[doc(hidden)]
    pub error_code: ::std::option::Option<crate::types::BatchGetAssetPropertyValueErrorCode>,
    /// <p>The date the error occurred, in Unix epoch time.</p>
    #[doc(hidden)]
    pub error_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BatchGetAssetPropertyValueErrorInfo {
    /// <p>The error code.</p>
    pub fn error_code(
        &self,
    ) -> ::std::option::Option<&crate::types::BatchGetAssetPropertyValueErrorCode> {
        self.error_code.as_ref()
    }
    /// <p>The date the error occurred, in Unix epoch time.</p>
    pub fn error_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.error_timestamp.as_ref()
    }
}
impl BatchGetAssetPropertyValueErrorInfo {
    /// Creates a new builder-style object to manufacture [`BatchGetAssetPropertyValueErrorInfo`](crate::types::BatchGetAssetPropertyValueErrorInfo).
    pub fn builder() -> crate::types::builders::BatchGetAssetPropertyValueErrorInfoBuilder {
        crate::types::builders::BatchGetAssetPropertyValueErrorInfoBuilder::default()
    }
}

/// A builder for [`BatchGetAssetPropertyValueErrorInfo`](crate::types::BatchGetAssetPropertyValueErrorInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetAssetPropertyValueErrorInfoBuilder {
    pub(crate) error_code: ::std::option::Option<crate::types::BatchGetAssetPropertyValueErrorCode>,
    pub(crate) error_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl BatchGetAssetPropertyValueErrorInfoBuilder {
    /// <p>The error code.</p>
    pub fn error_code(mut self, input: crate::types::BatchGetAssetPropertyValueErrorCode) -> Self {
        self.error_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The error code.</p>
    pub fn set_error_code(
        mut self,
        input: ::std::option::Option<crate::types::BatchGetAssetPropertyValueErrorCode>,
    ) -> Self {
        self.error_code = input;
        self
    }
    /// <p>The date the error occurred, in Unix epoch time.</p>
    pub fn error_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.error_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the error occurred, in Unix epoch time.</p>
    pub fn set_error_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.error_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetAssetPropertyValueErrorInfo`](crate::types::BatchGetAssetPropertyValueErrorInfo).
    pub fn build(self) -> crate::types::BatchGetAssetPropertyValueErrorInfo {
        crate::types::BatchGetAssetPropertyValueErrorInfo {
            error_code: self.error_code,
            error_timestamp: self.error_timestamp,
        }
    }
}
