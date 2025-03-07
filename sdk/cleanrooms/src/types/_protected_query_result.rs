// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about the query results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProtectedQueryResult {
    /// <p>The output of the protected query.</p>
    #[doc(hidden)]
    pub output: ::std::option::Option<crate::types::ProtectedQueryOutput>,
}
impl ProtectedQueryResult {
    /// <p>The output of the protected query.</p>
    pub fn output(&self) -> ::std::option::Option<&crate::types::ProtectedQueryOutput> {
        self.output.as_ref()
    }
}
impl ProtectedQueryResult {
    /// Creates a new builder-style object to manufacture [`ProtectedQueryResult`](crate::types::ProtectedQueryResult).
    pub fn builder() -> crate::types::builders::ProtectedQueryResultBuilder {
        crate::types::builders::ProtectedQueryResultBuilder::default()
    }
}

/// A builder for [`ProtectedQueryResult`](crate::types::ProtectedQueryResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProtectedQueryResultBuilder {
    pub(crate) output: ::std::option::Option<crate::types::ProtectedQueryOutput>,
}
impl ProtectedQueryResultBuilder {
    /// <p>The output of the protected query.</p>
    pub fn output(mut self, input: crate::types::ProtectedQueryOutput) -> Self {
        self.output = ::std::option::Option::Some(input);
        self
    }
    /// <p>The output of the protected query.</p>
    pub fn set_output(
        mut self,
        input: ::std::option::Option<crate::types::ProtectedQueryOutput>,
    ) -> Self {
        self.output = input;
        self
    }
    /// Consumes the builder and constructs a [`ProtectedQueryResult`](crate::types::ProtectedQueryResult).
    pub fn build(self) -> crate::types::ProtectedQueryResult {
        crate::types::ProtectedQueryResult {
            output: self.output,
        }
    }
}
