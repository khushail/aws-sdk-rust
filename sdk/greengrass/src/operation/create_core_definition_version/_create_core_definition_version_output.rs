// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCoreDefinitionVersionOutput {
    /// The ARN of the version.
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// The time, in milliseconds since the epoch, when the version was created.
    #[doc(hidden)]
    pub creation_timestamp: ::std::option::Option<::std::string::String>,
    /// The ID of the parent definition that the version is associated with.
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// The ID of the version.
    #[doc(hidden)]
    pub version: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateCoreDefinitionVersionOutput {
    /// The ARN of the version.
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// The time, in milliseconds since the epoch, when the version was created.
    pub fn creation_timestamp(&self) -> ::std::option::Option<&str> {
        self.creation_timestamp.as_deref()
    }
    /// The ID of the parent definition that the version is associated with.
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// The ID of the version.
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateCoreDefinitionVersionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateCoreDefinitionVersionOutput {
    /// Creates a new builder-style object to manufacture [`CreateCoreDefinitionVersionOutput`](crate::operation::create_core_definition_version::CreateCoreDefinitionVersionOutput).
    pub fn builder() -> crate::operation::create_core_definition_version::builders::CreateCoreDefinitionVersionOutputBuilder{
        crate::operation::create_core_definition_version::builders::CreateCoreDefinitionVersionOutputBuilder::default()
    }
}

/// A builder for [`CreateCoreDefinitionVersionOutput`](crate::operation::create_core_definition_version::CreateCoreDefinitionVersionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCoreDefinitionVersionOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_timestamp: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateCoreDefinitionVersionOutputBuilder {
    /// The ARN of the version.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the version.
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// The time, in milliseconds since the epoch, when the version was created.
    pub fn creation_timestamp(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.creation_timestamp = ::std::option::Option::Some(input.into());
        self
    }
    /// The time, in milliseconds since the epoch, when the version was created.
    pub fn set_creation_timestamp(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.creation_timestamp = input;
        self
    }
    /// The ID of the parent definition that the version is associated with.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the parent definition that the version is associated with.
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// The ID of the version.
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the version.
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateCoreDefinitionVersionOutput`](crate::operation::create_core_definition_version::CreateCoreDefinitionVersionOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_core_definition_version::CreateCoreDefinitionVersionOutput {
        crate::operation::create_core_definition_version::CreateCoreDefinitionVersionOutput {
            arn: self.arn,
            creation_timestamp: self.creation_timestamp,
            id: self.id,
            version: self.version,
            _request_id: self._request_id,
        }
    }
}
