// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a set of filters for a path analysis. Use path filters to scope the analysis when there can be multiple resulting paths.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PathFilter {
    /// <p>The source IPv4 address.</p>
    #[doc(hidden)]
    pub source_address: ::std::option::Option<::std::string::String>,
    /// <p>The source port range.</p>
    #[doc(hidden)]
    pub source_port_range: ::std::option::Option<crate::types::FilterPortRange>,
    /// <p>The destination IPv4 address.</p>
    #[doc(hidden)]
    pub destination_address: ::std::option::Option<::std::string::String>,
    /// <p>The destination port range.</p>
    #[doc(hidden)]
    pub destination_port_range: ::std::option::Option<crate::types::FilterPortRange>,
}
impl PathFilter {
    /// <p>The source IPv4 address.</p>
    pub fn source_address(&self) -> ::std::option::Option<&str> {
        self.source_address.as_deref()
    }
    /// <p>The source port range.</p>
    pub fn source_port_range(&self) -> ::std::option::Option<&crate::types::FilterPortRange> {
        self.source_port_range.as_ref()
    }
    /// <p>The destination IPv4 address.</p>
    pub fn destination_address(&self) -> ::std::option::Option<&str> {
        self.destination_address.as_deref()
    }
    /// <p>The destination port range.</p>
    pub fn destination_port_range(&self) -> ::std::option::Option<&crate::types::FilterPortRange> {
        self.destination_port_range.as_ref()
    }
}
impl PathFilter {
    /// Creates a new builder-style object to manufacture [`PathFilter`](crate::types::PathFilter).
    pub fn builder() -> crate::types::builders::PathFilterBuilder {
        crate::types::builders::PathFilterBuilder::default()
    }
}

/// A builder for [`PathFilter`](crate::types::PathFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PathFilterBuilder {
    pub(crate) source_address: ::std::option::Option<::std::string::String>,
    pub(crate) source_port_range: ::std::option::Option<crate::types::FilterPortRange>,
    pub(crate) destination_address: ::std::option::Option<::std::string::String>,
    pub(crate) destination_port_range: ::std::option::Option<crate::types::FilterPortRange>,
}
impl PathFilterBuilder {
    /// <p>The source IPv4 address.</p>
    pub fn source_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.source_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source IPv4 address.</p>
    pub fn set_source_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.source_address = input;
        self
    }
    /// <p>The source port range.</p>
    pub fn source_port_range(mut self, input: crate::types::FilterPortRange) -> Self {
        self.source_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source port range.</p>
    pub fn set_source_port_range(
        mut self,
        input: ::std::option::Option<crate::types::FilterPortRange>,
    ) -> Self {
        self.source_port_range = input;
        self
    }
    /// <p>The destination IPv4 address.</p>
    pub fn destination_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.destination_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination IPv4 address.</p>
    pub fn set_destination_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.destination_address = input;
        self
    }
    /// <p>The destination port range.</p>
    pub fn destination_port_range(mut self, input: crate::types::FilterPortRange) -> Self {
        self.destination_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination port range.</p>
    pub fn set_destination_port_range(
        mut self,
        input: ::std::option::Option<crate::types::FilterPortRange>,
    ) -> Self {
        self.destination_port_range = input;
        self
    }
    /// Consumes the builder and constructs a [`PathFilter`](crate::types::PathFilter).
    pub fn build(self) -> crate::types::PathFilter {
        crate::types::PathFilter {
            source_address: self.source_address,
            source_port_range: self.source_port_range,
            destination_address: self.destination_address,
            destination_port_range: self.destination_port_range,
        }
    }
}
