// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A filter used to limit results when describing inbound or outbound cross-cluster search connections. Multiple values can be specified per filter. A cross-cluster search connection must match at least one of the specified values for it to be returned from an operation. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Filter {
    /// <p> Specifies the name of the filter. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> Contains one or more values for the filter. </p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl Filter {
    /// <p> Specifies the name of the filter. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> Contains one or more values for the filter. </p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl Filter {
    /// Creates a new builder-style object to manufacture [`Filter`](crate::types::Filter).
    pub fn builder() -> crate::types::builders::FilterBuilder {
        crate::types::builders::FilterBuilder::default()
    }
}

/// A builder for [`Filter`](crate::types::Filter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FilterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FilterBuilder {
    /// <p> Specifies the name of the filter. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> Specifies the name of the filter. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p> Contains one or more values for the filter. </p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p> Contains one or more values for the filter. </p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`Filter`](crate::types::Filter).
    pub fn build(self) -> crate::types::Filter {
        crate::types::Filter {
            name: self.name,
            values: self.values,
        }
    }
}
