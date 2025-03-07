// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSslPoliciesInput {
    /// <p>The names of the policies.</p>
    #[doc(hidden)]
    pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    #[doc(hidden)]
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return with this call.</p>
    #[doc(hidden)]
    pub page_size: ::std::option::Option<i32>,
    /// <p> The type of load balancer. The default lists the SSL policies for all load balancers.</p>
    #[doc(hidden)]
    pub load_balancer_type: ::std::option::Option<crate::types::LoadBalancerTypeEnum>,
}
impl DescribeSslPoliciesInput {
    /// <p>The names of the policies.</p>
    pub fn names(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.names.as_deref()
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn page_size(&self) -> ::std::option::Option<i32> {
        self.page_size
    }
    /// <p> The type of load balancer. The default lists the SSL policies for all load balancers.</p>
    pub fn load_balancer_type(&self) -> ::std::option::Option<&crate::types::LoadBalancerTypeEnum> {
        self.load_balancer_type.as_ref()
    }
}
impl DescribeSslPoliciesInput {
    /// Creates a new builder-style object to manufacture [`DescribeSslPoliciesInput`](crate::operation::describe_ssl_policies::DescribeSslPoliciesInput).
    pub fn builder(
    ) -> crate::operation::describe_ssl_policies::builders::DescribeSslPoliciesInputBuilder {
        crate::operation::describe_ssl_policies::builders::DescribeSslPoliciesInputBuilder::default(
        )
    }
}

/// A builder for [`DescribeSslPoliciesInput`](crate::operation::describe_ssl_policies::DescribeSslPoliciesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeSslPoliciesInputBuilder {
    pub(crate) names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) page_size: ::std::option::Option<i32>,
    pub(crate) load_balancer_type: ::std::option::Option<crate::types::LoadBalancerTypeEnum>,
}
impl DescribeSslPoliciesInputBuilder {
    /// Appends an item to `names`.
    ///
    /// To override the contents of this collection use [`set_names`](Self::set_names).
    ///
    /// <p>The names of the policies.</p>
    pub fn names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.names.unwrap_or_default();
        v.push(input.into());
        self.names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The names of the policies.</p>
    pub fn set_names(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.names = input;
        self
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The marker for the next set of results. (You received this marker from a previous call.)</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.page_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return with this call.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.page_size = input;
        self
    }
    /// <p> The type of load balancer. The default lists the SSL policies for all load balancers.</p>
    pub fn load_balancer_type(mut self, input: crate::types::LoadBalancerTypeEnum) -> Self {
        self.load_balancer_type = ::std::option::Option::Some(input);
        self
    }
    /// <p> The type of load balancer. The default lists the SSL policies for all load balancers.</p>
    pub fn set_load_balancer_type(
        mut self,
        input: ::std::option::Option<crate::types::LoadBalancerTypeEnum>,
    ) -> Self {
        self.load_balancer_type = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSslPoliciesInput`](crate::operation::describe_ssl_policies::DescribeSslPoliciesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_ssl_policies::DescribeSslPoliciesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_ssl_policies::DescribeSslPoliciesInput {
                names: self.names,
                marker: self.marker,
                page_size: self.page_size,
                load_balancer_type: self.load_balancer_type,
            },
        )
    }
}
