// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input structure for CloudMasking operation type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudMaskingConfigInput {}
impl CloudMaskingConfigInput {
    /// Creates a new builder-style object to manufacture [`CloudMaskingConfigInput`](crate::types::CloudMaskingConfigInput).
    pub fn builder() -> crate::types::builders::CloudMaskingConfigInputBuilder {
        crate::types::builders::CloudMaskingConfigInputBuilder::default()
    }
}

/// A builder for [`CloudMaskingConfigInput`](crate::types::CloudMaskingConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CloudMaskingConfigInputBuilder {}
impl CloudMaskingConfigInputBuilder {
    /// Consumes the builder and constructs a [`CloudMaskingConfigInput`](crate::types::CloudMaskingConfigInput).
    pub fn build(self) -> crate::types::CloudMaskingConfigInput {
        crate::types::CloudMaskingConfigInput {}
    }
}
