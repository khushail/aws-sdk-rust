// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request body for UpdateBackendStorage.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateBackendStorageInput {
    /// <p>The app ID.</p>
    #[doc(hidden)]
    pub app_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the backend environment.</p>
    #[doc(hidden)]
    pub backend_environment_name: ::std::option::Option<::std::string::String>,
    /// <p>The resource configuration for updating backend storage.</p>
    #[doc(hidden)]
    pub resource_config: ::std::option::Option<crate::types::UpdateBackendStorageResourceConfig>,
    /// <p>The name of the storage resource.</p>
    #[doc(hidden)]
    pub resource_name: ::std::option::Option<::std::string::String>,
}
impl UpdateBackendStorageInput {
    /// <p>The app ID.</p>
    pub fn app_id(&self) -> ::std::option::Option<&str> {
        self.app_id.as_deref()
    }
    /// <p>The name of the backend environment.</p>
    pub fn backend_environment_name(&self) -> ::std::option::Option<&str> {
        self.backend_environment_name.as_deref()
    }
    /// <p>The resource configuration for updating backend storage.</p>
    pub fn resource_config(
        &self,
    ) -> ::std::option::Option<&crate::types::UpdateBackendStorageResourceConfig> {
        self.resource_config.as_ref()
    }
    /// <p>The name of the storage resource.</p>
    pub fn resource_name(&self) -> ::std::option::Option<&str> {
        self.resource_name.as_deref()
    }
}
impl UpdateBackendStorageInput {
    /// Creates a new builder-style object to manufacture [`UpdateBackendStorageInput`](crate::operation::update_backend_storage::UpdateBackendStorageInput).
    pub fn builder(
    ) -> crate::operation::update_backend_storage::builders::UpdateBackendStorageInputBuilder {
        crate::operation::update_backend_storage::builders::UpdateBackendStorageInputBuilder::default()
    }
}

/// A builder for [`UpdateBackendStorageInput`](crate::operation::update_backend_storage::UpdateBackendStorageInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateBackendStorageInputBuilder {
    pub(crate) app_id: ::std::option::Option<::std::string::String>,
    pub(crate) backend_environment_name: ::std::option::Option<::std::string::String>,
    pub(crate) resource_config:
        ::std::option::Option<crate::types::UpdateBackendStorageResourceConfig>,
    pub(crate) resource_name: ::std::option::Option<::std::string::String>,
}
impl UpdateBackendStorageInputBuilder {
    /// <p>The app ID.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The app ID.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_id = input;
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn backend_environment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.backend_environment_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn set_backend_environment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.backend_environment_name = input;
        self
    }
    /// <p>The resource configuration for updating backend storage.</p>
    pub fn resource_config(
        mut self,
        input: crate::types::UpdateBackendStorageResourceConfig,
    ) -> Self {
        self.resource_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The resource configuration for updating backend storage.</p>
    pub fn set_resource_config(
        mut self,
        input: ::std::option::Option<crate::types::UpdateBackendStorageResourceConfig>,
    ) -> Self {
        self.resource_config = input;
        self
    }
    /// <p>The name of the storage resource.</p>
    pub fn resource_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the storage resource.</p>
    pub fn set_resource_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_name = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateBackendStorageInput`](crate::operation::update_backend_storage::UpdateBackendStorageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_backend_storage::UpdateBackendStorageInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_backend_storage::UpdateBackendStorageInput {
                app_id: self.app_id,
                backend_environment_name: self.backend_environment_name,
                resource_config: self.resource_config,
                resource_name: self.resource_name,
            },
        )
    }
}
