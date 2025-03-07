// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration for the URI path route type. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UriPathRouteInput {
    /// <p>The path to use to match traffic. Paths must start with <code>/</code> and are relative to the base of the application.</p>
    #[doc(hidden)]
    pub source_path: ::std::option::Option<::std::string::String>,
    /// <p>If set to <code>ACTIVE</code>, traffic is forwarded to this route’s service after the route is created. </p>
    #[doc(hidden)]
    pub activation_state: ::std::option::Option<crate::types::RouteActivationState>,
    /// <p>A list of HTTP methods to match. An empty list matches all values. If a method is present, only HTTP requests using that method are forwarded to this route’s service. </p>
    #[doc(hidden)]
    pub methods: ::std::option::Option<::std::vec::Vec<crate::types::HttpMethod>>,
    /// <p>Indicates whether to match all subpaths of the given source path. If this value is <code>false</code>, requests must match the source path exactly before they are forwarded to this route's service. </p>
    #[doc(hidden)]
    pub include_child_paths: ::std::option::Option<bool>,
}
impl UriPathRouteInput {
    /// <p>The path to use to match traffic. Paths must start with <code>/</code> and are relative to the base of the application.</p>
    pub fn source_path(&self) -> ::std::option::Option<&str> {
        self.source_path.as_deref()
    }
    /// <p>If set to <code>ACTIVE</code>, traffic is forwarded to this route’s service after the route is created. </p>
    pub fn activation_state(&self) -> ::std::option::Option<&crate::types::RouteActivationState> {
        self.activation_state.as_ref()
    }
    /// <p>A list of HTTP methods to match. An empty list matches all values. If a method is present, only HTTP requests using that method are forwarded to this route’s service. </p>
    pub fn methods(&self) -> ::std::option::Option<&[crate::types::HttpMethod]> {
        self.methods.as_deref()
    }
    /// <p>Indicates whether to match all subpaths of the given source path. If this value is <code>false</code>, requests must match the source path exactly before they are forwarded to this route's service. </p>
    pub fn include_child_paths(&self) -> ::std::option::Option<bool> {
        self.include_child_paths
    }
}
impl UriPathRouteInput {
    /// Creates a new builder-style object to manufacture [`UriPathRouteInput`](crate::types::UriPathRouteInput).
    pub fn builder() -> crate::types::builders::UriPathRouteInputBuilder {
        crate::types::builders::UriPathRouteInputBuilder::default()
    }
}

/// A builder for [`UriPathRouteInput`](crate::types::UriPathRouteInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UriPathRouteInputBuilder {
    pub(crate) source_path: ::std::option::Option<::std::string::String>,
    pub(crate) activation_state: ::std::option::Option<crate::types::RouteActivationState>,
    pub(crate) methods: ::std::option::Option<::std::vec::Vec<crate::types::HttpMethod>>,
    pub(crate) include_child_paths: ::std::option::Option<bool>,
}
impl UriPathRouteInputBuilder {
    /// <p>The path to use to match traffic. Paths must start with <code>/</code> and are relative to the base of the application.</p>
    pub fn source_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to use to match traffic. Paths must start with <code>/</code> and are relative to the base of the application.</p>
    pub fn set_source_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_path = input;
        self
    }
    /// <p>If set to <code>ACTIVE</code>, traffic is forwarded to this route’s service after the route is created. </p>
    pub fn activation_state(mut self, input: crate::types::RouteActivationState) -> Self {
        self.activation_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>If set to <code>ACTIVE</code>, traffic is forwarded to this route’s service after the route is created. </p>
    pub fn set_activation_state(
        mut self,
        input: ::std::option::Option<crate::types::RouteActivationState>,
    ) -> Self {
        self.activation_state = input;
        self
    }
    /// Appends an item to `methods`.
    ///
    /// To override the contents of this collection use [`set_methods`](Self::set_methods).
    ///
    /// <p>A list of HTTP methods to match. An empty list matches all values. If a method is present, only HTTP requests using that method are forwarded to this route’s service. </p>
    pub fn methods(mut self, input: crate::types::HttpMethod) -> Self {
        let mut v = self.methods.unwrap_or_default();
        v.push(input);
        self.methods = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of HTTP methods to match. An empty list matches all values. If a method is present, only HTTP requests using that method are forwarded to this route’s service. </p>
    pub fn set_methods(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::HttpMethod>>,
    ) -> Self {
        self.methods = input;
        self
    }
    /// <p>Indicates whether to match all subpaths of the given source path. If this value is <code>false</code>, requests must match the source path exactly before they are forwarded to this route's service. </p>
    pub fn include_child_paths(mut self, input: bool) -> Self {
        self.include_child_paths = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to match all subpaths of the given source path. If this value is <code>false</code>, requests must match the source path exactly before they are forwarded to this route's service. </p>
    pub fn set_include_child_paths(mut self, input: ::std::option::Option<bool>) -> Self {
        self.include_child_paths = input;
        self
    }
    /// Consumes the builder and constructs a [`UriPathRouteInput`](crate::types::UriPathRouteInput).
    pub fn build(self) -> crate::types::UriPathRouteInput {
        crate::types::UriPathRouteInput {
            source_path: self.source_path,
            activation_state: self.activation_state,
            methods: self.methods,
            include_child_paths: self.include_child_paths,
        }
    }
}
