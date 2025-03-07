// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartDeploymentInput {
    /// <p>The application ID.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The environment ID.</p>
    #[doc(hidden)]
    pub environment_id: ::std::option::Option<::std::string::String>,
    /// <p>The deployment strategy ID.</p>
    #[doc(hidden)]
    pub deployment_strategy_id: ::std::option::Option<::std::string::String>,
    /// <p>The configuration profile ID.</p>
    #[doc(hidden)]
    pub configuration_profile_id: ::std::option::Option<::std::string::String>,
    /// <p>The configuration version to deploy. If deploying an AppConfig hosted configuration version, you can specify either the version number or version label.</p>
    #[doc(hidden)]
    pub configuration_version: ::std::option::Option<::std::string::String>,
    /// <p>A description of the deployment.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Metadata to assign to the deployment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this ID to encrypt the configuration data using a customer managed key. </p>
    #[doc(hidden)]
    pub kms_key_identifier: ::std::option::Option<::std::string::String>,
}
impl StartDeploymentInput {
    /// <p>The application ID.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The environment ID.</p>
    pub fn environment_id(&self) -> ::std::option::Option<&str> {
        self.environment_id.as_deref()
    }
    /// <p>The deployment strategy ID.</p>
    pub fn deployment_strategy_id(&self) -> ::std::option::Option<&str> {
        self.deployment_strategy_id.as_deref()
    }
    /// <p>The configuration profile ID.</p>
    pub fn configuration_profile_id(&self) -> ::std::option::Option<&str> {
        self.configuration_profile_id.as_deref()
    }
    /// <p>The configuration version to deploy. If deploying an AppConfig hosted configuration version, you can specify either the version number or version label.</p>
    pub fn configuration_version(&self) -> ::std::option::Option<&str> {
        self.configuration_version.as_deref()
    }
    /// <p>A description of the deployment.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Metadata to assign to the deployment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this ID to encrypt the configuration data using a customer managed key. </p>
    pub fn kms_key_identifier(&self) -> ::std::option::Option<&str> {
        self.kms_key_identifier.as_deref()
    }
}
impl StartDeploymentInput {
    /// Creates a new builder-style object to manufacture [`StartDeploymentInput`](crate::operation::start_deployment::StartDeploymentInput).
    pub fn builder() -> crate::operation::start_deployment::builders::StartDeploymentInputBuilder {
        crate::operation::start_deployment::builders::StartDeploymentInputBuilder::default()
    }
}

/// A builder for [`StartDeploymentInput`](crate::operation::start_deployment::StartDeploymentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartDeploymentInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) environment_id: ::std::option::Option<::std::string::String>,
    pub(crate) deployment_strategy_id: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_profile_id: ::std::option::Option<::std::string::String>,
    pub(crate) configuration_version: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) kms_key_identifier: ::std::option::Option<::std::string::String>,
}
impl StartDeploymentInputBuilder {
    /// <p>The application ID.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The application ID.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The environment ID.</p>
    pub fn environment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The environment ID.</p>
    pub fn set_environment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_id = input;
        self
    }
    /// <p>The deployment strategy ID.</p>
    pub fn deployment_strategy_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.deployment_strategy_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The deployment strategy ID.</p>
    pub fn set_deployment_strategy_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.deployment_strategy_id = input;
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn configuration_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_profile_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The configuration profile ID.</p>
    pub fn set_configuration_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_profile_id = input;
        self
    }
    /// <p>The configuration version to deploy. If deploying an AppConfig hosted configuration version, you can specify either the version number or version label.</p>
    pub fn configuration_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The configuration version to deploy. If deploying an AppConfig hosted configuration version, you can specify either the version number or version label.</p>
    pub fn set_configuration_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration_version = input;
        self
    }
    /// <p>A description of the deployment.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the deployment.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata to assign to the deployment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Metadata to assign to the deployment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this ID to encrypt the configuration data using a customer managed key. </p>
    pub fn kms_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.kms_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The KMS key identifier (key ID, key alias, or key ARN). AppConfig uses this ID to encrypt the configuration data using a customer managed key. </p>
    pub fn set_kms_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.kms_key_identifier = input;
        self
    }
    /// Consumes the builder and constructs a [`StartDeploymentInput`](crate::operation::start_deployment::StartDeploymentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_deployment::StartDeploymentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_deployment::StartDeploymentInput {
            application_id: self.application_id,
            environment_id: self.environment_id,
            deployment_strategy_id: self.deployment_strategy_id,
            configuration_profile_id: self.configuration_profile_id,
            configuration_version: self.configuration_version,
            description: self.description,
            tags: self.tags,
            kms_key_identifier: self.kms_key_identifier,
        })
    }
}
