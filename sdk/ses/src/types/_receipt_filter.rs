// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A receipt IP address filter enables you to specify whether to accept or reject mail originating from an IP address or range of IP addresses.</p>
/// <p>For information about setting up IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReceiptFilter {
    /// <p>The name of the IP address filter. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>A structure that provides the IP addresses to block or allow, and whether to block or allow incoming mail from them.</p>
    #[doc(hidden)]
    pub ip_filter: ::std::option::Option<crate::types::ReceiptIpFilter>,
}
impl ReceiptFilter {
    /// <p>The name of the IP address filter. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>A structure that provides the IP addresses to block or allow, and whether to block or allow incoming mail from them.</p>
    pub fn ip_filter(&self) -> ::std::option::Option<&crate::types::ReceiptIpFilter> {
        self.ip_filter.as_ref()
    }
}
impl ReceiptFilter {
    /// Creates a new builder-style object to manufacture [`ReceiptFilter`](crate::types::ReceiptFilter).
    pub fn builder() -> crate::types::builders::ReceiptFilterBuilder {
        crate::types::builders::ReceiptFilterBuilder::default()
    }
}

/// A builder for [`ReceiptFilter`](crate::types::ReceiptFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReceiptFilterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) ip_filter: ::std::option::Option<crate::types::ReceiptIpFilter>,
}
impl ReceiptFilterBuilder {
    /// <p>The name of the IP address filter. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the IP address filter. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A structure that provides the IP addresses to block or allow, and whether to block or allow incoming mail from them.</p>
    pub fn ip_filter(mut self, input: crate::types::ReceiptIpFilter) -> Self {
        self.ip_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure that provides the IP addresses to block or allow, and whether to block or allow incoming mail from them.</p>
    pub fn set_ip_filter(
        mut self,
        input: ::std::option::Option<crate::types::ReceiptIpFilter>,
    ) -> Self {
        self.ip_filter = input;
        self
    }
    /// Consumes the builder and constructs a [`ReceiptFilter`](crate::types::ReceiptFilter).
    pub fn build(self) -> crate::types::ReceiptFilter {
        crate::types::ReceiptFilter {
            name: self.name,
            ip_filter: self.ip_filter,
        }
    }
}
