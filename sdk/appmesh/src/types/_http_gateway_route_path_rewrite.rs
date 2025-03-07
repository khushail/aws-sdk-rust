// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the path to rewrite.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HttpGatewayRoutePathRewrite {
    /// <p>The exact path to rewrite.</p>
    #[doc(hidden)]
    pub exact: ::std::option::Option<::std::string::String>,
}
impl HttpGatewayRoutePathRewrite {
    /// <p>The exact path to rewrite.</p>
    pub fn exact(&self) -> ::std::option::Option<&str> {
        self.exact.as_deref()
    }
}
impl HttpGatewayRoutePathRewrite {
    /// Creates a new builder-style object to manufacture [`HttpGatewayRoutePathRewrite`](crate::types::HttpGatewayRoutePathRewrite).
    pub fn builder() -> crate::types::builders::HttpGatewayRoutePathRewriteBuilder {
        crate::types::builders::HttpGatewayRoutePathRewriteBuilder::default()
    }
}

/// A builder for [`HttpGatewayRoutePathRewrite`](crate::types::HttpGatewayRoutePathRewrite).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HttpGatewayRoutePathRewriteBuilder {
    pub(crate) exact: ::std::option::Option<::std::string::String>,
}
impl HttpGatewayRoutePathRewriteBuilder {
    /// <p>The exact path to rewrite.</p>
    pub fn exact(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.exact = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The exact path to rewrite.</p>
    pub fn set_exact(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.exact = input;
        self
    }
    /// Consumes the builder and constructs a [`HttpGatewayRoutePathRewrite`](crate::types::HttpGatewayRoutePathRewrite).
    pub fn build(self) -> crate::types::HttpGatewayRoutePathRewrite {
        crate::types::HttpGatewayRoutePathRewrite { exact: self.exact }
    }
}
