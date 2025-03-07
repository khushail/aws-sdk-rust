// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetLoadBalancerInput {
    /// <p>The name of the load balancer.</p>
    #[doc(hidden)]
    pub load_balancer_name: ::std::option::Option<::std::string::String>,
}
impl GetLoadBalancerInput {
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(&self) -> ::std::option::Option<&str> {
        self.load_balancer_name.as_deref()
    }
}
impl GetLoadBalancerInput {
    /// Creates a new builder-style object to manufacture [`GetLoadBalancerInput`](crate::operation::get_load_balancer::GetLoadBalancerInput).
    pub fn builder() -> crate::operation::get_load_balancer::builders::GetLoadBalancerInputBuilder {
        crate::operation::get_load_balancer::builders::GetLoadBalancerInputBuilder::default()
    }
}

/// A builder for [`GetLoadBalancerInput`](crate::operation::get_load_balancer::GetLoadBalancerInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetLoadBalancerInputBuilder {
    pub(crate) load_balancer_name: ::std::option::Option<::std::string::String>,
}
impl GetLoadBalancerInputBuilder {
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.load_balancer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.load_balancer_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetLoadBalancerInput`](crate::operation::get_load_balancer::GetLoadBalancerInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_load_balancer::GetLoadBalancerInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_load_balancer::GetLoadBalancerInput {
            load_balancer_name: self.load_balancer_name,
        })
    }
}
