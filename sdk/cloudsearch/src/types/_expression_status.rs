// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The value of an <code>Expression</code> and its current status.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExpressionStatus {
    /// <p>The expression that is evaluated for sorting while processing a search request.</p>
    #[doc(hidden)]
    pub options: ::std::option::Option<crate::types::Expression>,
    /// <p>The status of domain configuration option.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::OptionStatus>,
}
impl ExpressionStatus {
    /// <p>The expression that is evaluated for sorting while processing a search request.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::Expression> {
        self.options.as_ref()
    }
    /// <p>The status of domain configuration option.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::OptionStatus> {
        self.status.as_ref()
    }
}
impl ExpressionStatus {
    /// Creates a new builder-style object to manufacture [`ExpressionStatus`](crate::types::ExpressionStatus).
    pub fn builder() -> crate::types::builders::ExpressionStatusBuilder {
        crate::types::builders::ExpressionStatusBuilder::default()
    }
}

/// A builder for [`ExpressionStatus`](crate::types::ExpressionStatus).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ExpressionStatusBuilder {
    pub(crate) options: ::std::option::Option<crate::types::Expression>,
    pub(crate) status: ::std::option::Option<crate::types::OptionStatus>,
}
impl ExpressionStatusBuilder {
    /// <p>The expression that is evaluated for sorting while processing a search request.</p>
    pub fn options(mut self, input: crate::types::Expression) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The expression that is evaluated for sorting while processing a search request.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::Expression>) -> Self {
        self.options = input;
        self
    }
    /// <p>The status of domain configuration option.</p>
    pub fn status(mut self, input: crate::types::OptionStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of domain configuration option.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::OptionStatus>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`ExpressionStatus`](crate::types::ExpressionStatus).
    pub fn build(self) -> crate::types::ExpressionStatus {
        crate::types::ExpressionStatus {
            options: self.options,
            status: self.status,
        }
    }
}
