// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This data type is used as a response element in the <code>DescribeDBSecurityGroups</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IpRange {
    /// <p>Specifies the status of the IP range. Status can be "authorizing", "authorized", "revoking", and "revoked".</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the IP range.</p>
    #[doc(hidden)]
    pub cidrip: ::std::option::Option<::std::string::String>,
}
impl IpRange {
    /// <p>Specifies the status of the IP range. Status can be "authorizing", "authorized", "revoking", and "revoked".</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>Specifies the IP range.</p>
    pub fn cidrip(&self) -> ::std::option::Option<&str> {
        self.cidrip.as_deref()
    }
}
impl IpRange {
    /// Creates a new builder-style object to manufacture [`IpRange`](crate::types::IpRange).
    pub fn builder() -> crate::types::builders::IpRangeBuilder {
        crate::types::builders::IpRangeBuilder::default()
    }
}

/// A builder for [`IpRange`](crate::types::IpRange).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IpRangeBuilder {
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) cidrip: ::std::option::Option<::std::string::String>,
}
impl IpRangeBuilder {
    /// <p>Specifies the status of the IP range. Status can be "authorizing", "authorized", "revoking", and "revoked".</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the status of the IP range. Status can be "authorizing", "authorized", "revoking", and "revoked".</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies the IP range.</p>
    pub fn cidrip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidrip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the IP range.</p>
    pub fn set_cidrip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidrip = input;
        self
    }
    /// Consumes the builder and constructs a [`IpRange`](crate::types::IpRange).
    pub fn build(self) -> crate::types::IpRange {
        crate::types::IpRange {
            status: self.status,
            cidrip: self.cidrip,
        }
    }
}
