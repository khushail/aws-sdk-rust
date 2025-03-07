// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Information about the Git submodules configuration for an CodeBuild build project. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GitSubmodulesConfig {
    /// <p> Set to true to fetch Git submodules for your CodeBuild build project. </p>
    #[doc(hidden)]
    pub fetch_submodules: ::std::option::Option<bool>,
}
impl GitSubmodulesConfig {
    /// <p> Set to true to fetch Git submodules for your CodeBuild build project. </p>
    pub fn fetch_submodules(&self) -> ::std::option::Option<bool> {
        self.fetch_submodules
    }
}
impl GitSubmodulesConfig {
    /// Creates a new builder-style object to manufacture [`GitSubmodulesConfig`](crate::types::GitSubmodulesConfig).
    pub fn builder() -> crate::types::builders::GitSubmodulesConfigBuilder {
        crate::types::builders::GitSubmodulesConfigBuilder::default()
    }
}

/// A builder for [`GitSubmodulesConfig`](crate::types::GitSubmodulesConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GitSubmodulesConfigBuilder {
    pub(crate) fetch_submodules: ::std::option::Option<bool>,
}
impl GitSubmodulesConfigBuilder {
    /// <p> Set to true to fetch Git submodules for your CodeBuild build project. </p>
    pub fn fetch_submodules(mut self, input: bool) -> Self {
        self.fetch_submodules = ::std::option::Option::Some(input);
        self
    }
    /// <p> Set to true to fetch Git submodules for your CodeBuild build project. </p>
    pub fn set_fetch_submodules(mut self, input: ::std::option::Option<bool>) -> Self {
        self.fetch_submodules = input;
        self
    }
    /// Consumes the builder and constructs a [`GitSubmodulesConfig`](crate::types::GitSubmodulesConfig).
    pub fn build(self) -> crate::types::GitSubmodulesConfig {
        crate::types::GitSubmodulesConfig {
            fetch_submodules: self.fetch_submodules,
        }
    }
}
