// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an Auto Scaling launch configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchConfiguration {
    /// <p>The name of the launch configuration.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl LaunchConfiguration {
    /// <p>The name of the launch configuration.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl LaunchConfiguration {
    /// Creates a new builder-style object to manufacture [`LaunchConfiguration`](crate::types::LaunchConfiguration).
    pub fn builder() -> crate::types::builders::LaunchConfigurationBuilder {
        crate::types::builders::LaunchConfigurationBuilder::default()
    }
}

/// A builder for [`LaunchConfiguration`](crate::types::LaunchConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LaunchConfigurationBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl LaunchConfigurationBuilder {
    /// <p>The name of the launch configuration.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the launch configuration.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchConfiguration`](crate::types::LaunchConfiguration).
    pub fn build(self) -> crate::types::LaunchConfiguration {
        crate::types::LaunchConfiguration { name: self.name }
    }
}
