// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for facet information. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Bucket {
    /// <p>The facet value being counted.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
    /// <p>The number of hits that contain the facet value in the specified facet field.</p>
    #[doc(hidden)]
    pub count: i64,
}
impl Bucket {
    /// <p>The facet value being counted.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
    /// <p>The number of hits that contain the facet value in the specified facet field.</p>
    pub fn count(&self) -> i64 {
        self.count
    }
}
impl Bucket {
    /// Creates a new builder-style object to manufacture [`Bucket`](crate::types::Bucket).
    pub fn builder() -> crate::types::builders::BucketBuilder {
        crate::types::builders::BucketBuilder::default()
    }
}

/// A builder for [`Bucket`](crate::types::Bucket).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BucketBuilder {
    pub(crate) value: ::std::option::Option<::std::string::String>,
    pub(crate) count: ::std::option::Option<i64>,
}
impl BucketBuilder {
    /// <p>The facet value being counted.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The facet value being counted.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The number of hits that contain the facet value in the specified facet field.</p>
    pub fn count(mut self, input: i64) -> Self {
        self.count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of hits that contain the facet value in the specified facet field.</p>
    pub fn set_count(mut self, input: ::std::option::Option<i64>) -> Self {
        self.count = input;
        self
    }
    /// Consumes the builder and constructs a [`Bucket`](crate::types::Bucket).
    pub fn build(self) -> crate::types::Bucket {
        crate::types::Bucket {
            value: self.value,
            count: self.count.unwrap_or_default(),
        }
    }
}
