// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateInstanceStorageConfigInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>A valid resource type.</p>
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<crate::types::InstanceStorageResourceType>,
    /// <p>A valid storage type.</p>
    #[doc(hidden)]
    pub storage_config: ::std::option::Option<crate::types::InstanceStorageConfig>,
}
impl AssociateInstanceStorageConfigInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>A valid resource type.</p>
    pub fn resource_type(
        &self,
    ) -> ::std::option::Option<&crate::types::InstanceStorageResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>A valid storage type.</p>
    pub fn storage_config(&self) -> ::std::option::Option<&crate::types::InstanceStorageConfig> {
        self.storage_config.as_ref()
    }
}
impl AssociateInstanceStorageConfigInput {
    /// Creates a new builder-style object to manufacture [`AssociateInstanceStorageConfigInput`](crate::operation::associate_instance_storage_config::AssociateInstanceStorageConfigInput).
    pub fn builder() -> crate::operation::associate_instance_storage_config::builders::AssociateInstanceStorageConfigInputBuilder{
        crate::operation::associate_instance_storage_config::builders::AssociateInstanceStorageConfigInputBuilder::default()
    }
}

/// A builder for [`AssociateInstanceStorageConfigInput`](crate::operation::associate_instance_storage_config::AssociateInstanceStorageConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateInstanceStorageConfigInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type: ::std::option::Option<crate::types::InstanceStorageResourceType>,
    pub(crate) storage_config: ::std::option::Option<crate::types::InstanceStorageConfig>,
}
impl AssociateInstanceStorageConfigInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>A valid resource type.</p>
    pub fn resource_type(mut self, input: crate::types::InstanceStorageResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>A valid resource type.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::InstanceStorageResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>A valid storage type.</p>
    pub fn storage_config(mut self, input: crate::types::InstanceStorageConfig) -> Self {
        self.storage_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>A valid storage type.</p>
    pub fn set_storage_config(
        mut self,
        input: ::std::option::Option<crate::types::InstanceStorageConfig>,
    ) -> Self {
        self.storage_config = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateInstanceStorageConfigInput`](crate::operation::associate_instance_storage_config::AssociateInstanceStorageConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_instance_storage_config::AssociateInstanceStorageConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_instance_storage_config::AssociateInstanceStorageConfigInput {
                instance_id: self.instance_id
                ,
                resource_type: self.resource_type
                ,
                storage_config: self.storage_config
                ,
            }
        )
    }
}
