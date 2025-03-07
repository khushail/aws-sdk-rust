// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A string parameter.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StringParameter {
    /// <p>A display name for a string parameter.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The values of a string parameter.</p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl StringParameter {
    /// <p>A display name for a string parameter.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The values of a string parameter.</p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl StringParameter {
    /// Creates a new builder-style object to manufacture [`StringParameter`](crate::types::StringParameter).
    pub fn builder() -> crate::types::builders::StringParameterBuilder {
        crate::types::builders::StringParameterBuilder::default()
    }
}

/// A builder for [`StringParameter`](crate::types::StringParameter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StringParameterBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl StringParameterBuilder {
    /// <p>A display name for a string parameter.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A display name for a string parameter.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>The values of a string parameter.</p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>The values of a string parameter.</p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`StringParameter`](crate::types::StringParameter).
    pub fn build(self) -> crate::types::StringParameter {
        crate::types::StringParameter {
            name: self.name,
            values: self.values,
        }
    }
}
