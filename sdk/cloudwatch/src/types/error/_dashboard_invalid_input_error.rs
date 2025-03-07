// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Some part of the dashboard data is invalid.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DashboardInvalidInputError {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub dashboard_validation_messages:
        ::std::option::Option<::std::vec::Vec<crate::types::DashboardValidationMessage>>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl DashboardInvalidInputError {
    #[allow(missing_docs)] // documentation missing in model
    pub fn dashboard_validation_messages(
        &self,
    ) -> ::std::option::Option<&[crate::types::DashboardValidationMessage]> {
        self.dashboard_validation_messages.as_deref()
    }
}
impl DashboardInvalidInputError {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for DashboardInvalidInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "DashboardInvalidInputError")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for DashboardInvalidInputError {}
impl ::aws_http::request_id::RequestId for crate::types::error::DashboardInvalidInputError {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DashboardInvalidInputError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl DashboardInvalidInputError {
    /// Creates a new builder-style object to manufacture [`DashboardInvalidInputError`](crate::types::error::DashboardInvalidInputError).
    pub fn builder() -> crate::types::error::builders::DashboardInvalidInputErrorBuilder {
        crate::types::error::builders::DashboardInvalidInputErrorBuilder::default()
    }
}

/// A builder for [`DashboardInvalidInputError`](crate::types::error::DashboardInvalidInputError).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DashboardInvalidInputErrorBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) dashboard_validation_messages:
        ::std::option::Option<::std::vec::Vec<crate::types::DashboardValidationMessage>>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl DashboardInvalidInputErrorBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Appends an item to `dashboard_validation_messages`.
    ///
    /// To override the contents of this collection use [`set_dashboard_validation_messages`](Self::set_dashboard_validation_messages).
    ///
    pub fn dashboard_validation_messages(
        mut self,
        input: crate::types::DashboardValidationMessage,
    ) -> Self {
        let mut v = self.dashboard_validation_messages.unwrap_or_default();
        v.push(input);
        self.dashboard_validation_messages = ::std::option::Option::Some(v);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_dashboard_validation_messages(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DashboardValidationMessage>>,
    ) -> Self {
        self.dashboard_validation_messages = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`DashboardInvalidInputError`](crate::types::error::DashboardInvalidInputError).
    pub fn build(self) -> crate::types::error::DashboardInvalidInputError {
        crate::types::error::DashboardInvalidInputError {
            message: self.message,
            dashboard_validation_messages: self.dashboard_validation_messages,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
