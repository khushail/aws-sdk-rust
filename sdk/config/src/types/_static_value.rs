// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The static value of the resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StaticValue {
    /// <p>A list of values. For example, the ARN of the assumed role. </p>
    #[doc(hidden)]
    pub values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl StaticValue {
    /// <p>A list of values. For example, the ARN of the assumed role. </p>
    pub fn values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.values.as_deref()
    }
}
impl StaticValue {
    /// Creates a new builder-style object to manufacture [`StaticValue`](crate::types::StaticValue).
    pub fn builder() -> crate::types::builders::StaticValueBuilder {
        crate::types::builders::StaticValueBuilder::default()
    }
}

/// A builder for [`StaticValue`](crate::types::StaticValue).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StaticValueBuilder {
    pub(crate) values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl StaticValueBuilder {
    /// Appends an item to `values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>A list of values. For example, the ARN of the assumed role. </p>
    pub fn values(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.values.unwrap_or_default();
        v.push(input.into());
        self.values = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of values. For example, the ARN of the assumed role. </p>
    pub fn set_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.values = input;
        self
    }
    /// Consumes the builder and constructs a [`StaticValue`](crate::types::StaticValue).
    pub fn build(self) -> crate::types::StaticValue {
        crate::types::StaticValue {
            values: self.values,
        }
    }
}
