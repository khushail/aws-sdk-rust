// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Request to delete an application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteApplicationInput {
    /// <p>The name of the application to delete.</p>
    #[doc(hidden)]
    pub application_name: ::std::option::Option<::std::string::String>,
    /// <p>When set to true, running environments will be terminated before deleting the application.</p>
    #[doc(hidden)]
    pub terminate_env_by_force: ::std::option::Option<bool>,
}
impl DeleteApplicationInput {
    /// <p>The name of the application to delete.</p>
    pub fn application_name(&self) -> ::std::option::Option<&str> {
        self.application_name.as_deref()
    }
    /// <p>When set to true, running environments will be terminated before deleting the application.</p>
    pub fn terminate_env_by_force(&self) -> ::std::option::Option<bool> {
        self.terminate_env_by_force
    }
}
impl DeleteApplicationInput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::operation::delete_application::DeleteApplicationInput).
    pub fn builder() -> crate::operation::delete_application::builders::DeleteApplicationInputBuilder
    {
        crate::operation::delete_application::builders::DeleteApplicationInputBuilder::default()
    }
}

/// A builder for [`DeleteApplicationInput`](crate::operation::delete_application::DeleteApplicationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteApplicationInputBuilder {
    pub(crate) application_name: ::std::option::Option<::std::string::String>,
    pub(crate) terminate_env_by_force: ::std::option::Option<bool>,
}
impl DeleteApplicationInputBuilder {
    /// <p>The name of the application to delete.</p>
    pub fn application_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the application to delete.</p>
    pub fn set_application_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_name = input;
        self
    }
    /// <p>When set to true, running environments will be terminated before deleting the application.</p>
    pub fn terminate_env_by_force(mut self, input: bool) -> Self {
        self.terminate_env_by_force = ::std::option::Option::Some(input);
        self
    }
    /// <p>When set to true, running environments will be terminated before deleting the application.</p>
    pub fn set_terminate_env_by_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.terminate_env_by_force = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteApplicationInput`](crate::operation::delete_application::DeleteApplicationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_application::DeleteApplicationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_application::DeleteApplicationInput {
                application_name: self.application_name,
                terminate_env_by_force: self.terminate_env_by_force,
            },
        )
    }
}
