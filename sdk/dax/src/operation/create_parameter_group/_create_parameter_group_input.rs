// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateParameterGroupInput {
    /// <p>The name of the parameter group to apply to all of the clusters in this replication group.</p>
    #[doc(hidden)]
    pub parameter_group_name: ::std::option::Option<::std::string::String>,
    /// <p>A description of the parameter group.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl CreateParameterGroupInput {
    /// <p>The name of the parameter group to apply to all of the clusters in this replication group.</p>
    pub fn parameter_group_name(&self) -> ::std::option::Option<&str> {
        self.parameter_group_name.as_deref()
    }
    /// <p>A description of the parameter group.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl CreateParameterGroupInput {
    /// Creates a new builder-style object to manufacture [`CreateParameterGroupInput`](crate::operation::create_parameter_group::CreateParameterGroupInput).
    pub fn builder(
    ) -> crate::operation::create_parameter_group::builders::CreateParameterGroupInputBuilder {
        crate::operation::create_parameter_group::builders::CreateParameterGroupInputBuilder::default()
    }
}

/// A builder for [`CreateParameterGroupInput`](crate::operation::create_parameter_group::CreateParameterGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateParameterGroupInputBuilder {
    pub(crate) parameter_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl CreateParameterGroupInputBuilder {
    /// <p>The name of the parameter group to apply to all of the clusters in this replication group.</p>
    pub fn parameter_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.parameter_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the parameter group to apply to all of the clusters in this replication group.</p>
    pub fn set_parameter_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.parameter_group_name = input;
        self
    }
    /// <p>A description of the parameter group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the parameter group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateParameterGroupInput`](crate::operation::create_parameter_group::CreateParameterGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_parameter_group::CreateParameterGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_parameter_group::CreateParameterGroupInput {
                parameter_group_name: self.parameter_group_name,
                description: self.description,
            },
        )
    }
}
