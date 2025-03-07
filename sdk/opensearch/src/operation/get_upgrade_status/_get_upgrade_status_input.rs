// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the request parameters to the <code>GetUpgradeStatus</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetUpgradeStatusInput {
    /// <p>The domain of the domain to get upgrade status information for.</p>
    #[doc(hidden)]
    pub domain_name: ::std::option::Option<::std::string::String>,
}
impl GetUpgradeStatusInput {
    /// <p>The domain of the domain to get upgrade status information for.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
}
impl GetUpgradeStatusInput {
    /// Creates a new builder-style object to manufacture [`GetUpgradeStatusInput`](crate::operation::get_upgrade_status::GetUpgradeStatusInput).
    pub fn builder() -> crate::operation::get_upgrade_status::builders::GetUpgradeStatusInputBuilder
    {
        crate::operation::get_upgrade_status::builders::GetUpgradeStatusInputBuilder::default()
    }
}

/// A builder for [`GetUpgradeStatusInput`](crate::operation::get_upgrade_status::GetUpgradeStatusInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetUpgradeStatusInputBuilder {
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
}
impl GetUpgradeStatusInputBuilder {
    /// <p>The domain of the domain to get upgrade status information for.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The domain of the domain to get upgrade status information for.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// Consumes the builder and constructs a [`GetUpgradeStatusInput`](crate::operation::get_upgrade_status::GetUpgradeStatusInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_upgrade_status::GetUpgradeStatusInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_upgrade_status::GetUpgradeStatusInput {
                domain_name: self.domain_name,
            },
        )
    }
}
