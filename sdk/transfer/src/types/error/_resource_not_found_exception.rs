// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This exception is thrown when a resource is not found by the Amazon Web ServicesTransfer Family service.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub resource: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn resource(&self) -> ::std::option::Option<&str> {
        self.resource.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_type(&self) -> ::std::option::Option<&str> {
        self.resource_type.as_deref()
    }
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "ResourceNotFoundException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ResourceNotFoundException {}
impl ::aws_http::request_id::RequestId for crate::types::error::ResourceNotFoundException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ResourceNotFoundException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::types::error::ResourceNotFoundException).
    pub fn builder() -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default()
    }
}

/// A builder for [`ResourceNotFoundException`](crate::types::error::ResourceNotFoundException).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceNotFoundExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) resource: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl ResourceNotFoundExceptionBuilder {
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
    #[allow(missing_docs)] // documentation missing in model
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_type = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_type = input;
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
    /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::types::error::ResourceNotFoundException).
    pub fn build(self) -> crate::types::error::ResourceNotFoundException {
        crate::types::error::ResourceNotFoundException {
            message: self.message,
            resource: self.resource,
            resource_type: self.resource_type,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
