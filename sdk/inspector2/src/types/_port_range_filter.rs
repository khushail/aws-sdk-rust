// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that describes the details of a port range filter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PortRangeFilter {
    /// <p>The port number the port range begins at.</p>
    #[doc(hidden)]
    pub begin_inclusive: ::std::option::Option<i32>,
    /// <p>The port number the port range ends at.</p>
    #[doc(hidden)]
    pub end_inclusive: ::std::option::Option<i32>,
}
impl PortRangeFilter {
    /// <p>The port number the port range begins at.</p>
    pub fn begin_inclusive(&self) -> ::std::option::Option<i32> {
        self.begin_inclusive
    }
    /// <p>The port number the port range ends at.</p>
    pub fn end_inclusive(&self) -> ::std::option::Option<i32> {
        self.end_inclusive
    }
}
impl PortRangeFilter {
    /// Creates a new builder-style object to manufacture [`PortRangeFilter`](crate::types::PortRangeFilter).
    pub fn builder() -> crate::types::builders::PortRangeFilterBuilder {
        crate::types::builders::PortRangeFilterBuilder::default()
    }
}

/// A builder for [`PortRangeFilter`](crate::types::PortRangeFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PortRangeFilterBuilder {
    pub(crate) begin_inclusive: ::std::option::Option<i32>,
    pub(crate) end_inclusive: ::std::option::Option<i32>,
}
impl PortRangeFilterBuilder {
    /// <p>The port number the port range begins at.</p>
    pub fn begin_inclusive(mut self, input: i32) -> Self {
        self.begin_inclusive = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port number the port range begins at.</p>
    pub fn set_begin_inclusive(mut self, input: ::std::option::Option<i32>) -> Self {
        self.begin_inclusive = input;
        self
    }
    /// <p>The port number the port range ends at.</p>
    pub fn end_inclusive(mut self, input: i32) -> Self {
        self.end_inclusive = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port number the port range ends at.</p>
    pub fn set_end_inclusive(mut self, input: ::std::option::Option<i32>) -> Self {
        self.end_inclusive = input;
        self
    }
    /// Consumes the builder and constructs a [`PortRangeFilter`](crate::types::PortRangeFilter).
    pub fn build(self) -> crate::types::PortRangeFilter {
        crate::types::PortRangeFilter {
            begin_inclusive: self.begin_inclusive,
            end_inclusive: self.end_inclusive,
        }
    }
}
