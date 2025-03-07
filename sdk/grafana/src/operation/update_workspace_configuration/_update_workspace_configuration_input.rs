// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateWorkspaceConfigurationInput {
    /// <p>The new configuration string for the workspace. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    #[doc(hidden)]
    pub configuration: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the workspace to update.</p>
    #[doc(hidden)]
    pub workspace_id: ::std::option::Option<::std::string::String>,
}
impl UpdateWorkspaceConfigurationInput {
    /// <p>The new configuration string for the workspace. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    pub fn configuration(&self) -> ::std::option::Option<&str> {
        self.configuration.as_deref()
    }
    /// <p>The ID of the workspace to update.</p>
    pub fn workspace_id(&self) -> ::std::option::Option<&str> {
        self.workspace_id.as_deref()
    }
}
impl UpdateWorkspaceConfigurationInput {
    /// Creates a new builder-style object to manufacture [`UpdateWorkspaceConfigurationInput`](crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationInput).
    pub fn builder() -> crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationInputBuilder{
        crate::operation::update_workspace_configuration::builders::UpdateWorkspaceConfigurationInputBuilder::default()
    }
}

/// A builder for [`UpdateWorkspaceConfigurationInput`](crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateWorkspaceConfigurationInputBuilder {
    pub(crate) configuration: ::std::option::Option<::std::string::String>,
    pub(crate) workspace_id: ::std::option::Option<::std::string::String>,
}
impl UpdateWorkspaceConfigurationInputBuilder {
    /// <p>The new configuration string for the workspace. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    pub fn configuration(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.configuration = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new configuration string for the workspace. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    pub fn set_configuration(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.configuration = input;
        self
    }
    /// <p>The ID of the workspace to update.</p>
    pub fn workspace_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workspace_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the workspace to update.</p>
    pub fn set_workspace_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workspace_id = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateWorkspaceConfigurationInput`](crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_workspace_configuration::UpdateWorkspaceConfigurationInput {
                configuration: self.configuration,
                workspace_id: self.workspace_id,
            },
        )
    }
}
