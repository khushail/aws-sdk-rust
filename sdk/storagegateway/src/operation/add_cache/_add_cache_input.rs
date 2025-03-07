// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AddCacheInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    #[doc(hidden)]
    pub gateway_arn: ::std::option::Option<::std::string::String>,
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <code>ListLocalDisks</code> API.</p>
    #[doc(hidden)]
    pub disk_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AddCacheInput {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(&self) -> ::std::option::Option<&str> {
        self.gateway_arn.as_deref()
    }
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <code>ListLocalDisks</code> API.</p>
    pub fn disk_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.disk_ids.as_deref()
    }
}
impl AddCacheInput {
    /// Creates a new builder-style object to manufacture [`AddCacheInput`](crate::operation::add_cache::AddCacheInput).
    pub fn builder() -> crate::operation::add_cache::builders::AddCacheInputBuilder {
        crate::operation::add_cache::builders::AddCacheInputBuilder::default()
    }
}

/// A builder for [`AddCacheInput`](crate::operation::add_cache::AddCacheInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AddCacheInputBuilder {
    pub(crate) gateway_arn: ::std::option::Option<::std::string::String>,
    pub(crate) disk_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AddCacheInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.gateway_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the gateway. Use the <code>ListGateways</code> operation to return a list of gateways for your account and Amazon Web Services Region.</p>
    pub fn set_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.gateway_arn = input;
        self
    }
    /// Appends an item to `disk_ids`.
    ///
    /// To override the contents of this collection use [`set_disk_ids`](Self::set_disk_ids).
    ///
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <code>ListLocalDisks</code> API.</p>
    pub fn disk_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.disk_ids.unwrap_or_default();
        v.push(input.into());
        self.disk_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of strings that identify disks that are to be configured as working storage. Each string has a minimum length of 1 and maximum length of 300. You can get the disk IDs from the <code>ListLocalDisks</code> API.</p>
    pub fn set_disk_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.disk_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`AddCacheInput`](crate::operation::add_cache::AddCacheInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::add_cache::AddCacheInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::add_cache::AddCacheInput {
            gateway_arn: self.gateway_arn,
            disk_ids: self.disk_ids,
        })
    }
}
