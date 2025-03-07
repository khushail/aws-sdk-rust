// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Amazon Resource Name (ARN) and job type of the source of a trial component.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TrialComponentSource {
    /// <p>The source Amazon Resource Name (ARN).</p>
    #[doc(hidden)]
    pub source_arn: ::std::option::Option<::std::string::String>,
    /// <p>The source job type.</p>
    #[doc(hidden)]
    pub source_type: ::std::option::Option<::std::string::String>,
}
impl TrialComponentSource {
    /// <p>The source Amazon Resource Name (ARN).</p>
    pub fn source_arn(&self) -> ::std::option::Option<&str> {
        self.source_arn.as_deref()
    }
    /// <p>The source job type.</p>
    pub fn source_type(&self) -> ::std::option::Option<&str> {
        self.source_type.as_deref()
    }
}
impl TrialComponentSource {
    /// Creates a new builder-style object to manufacture [`TrialComponentSource`](crate::types::TrialComponentSource).
    pub fn builder() -> crate::types::builders::TrialComponentSourceBuilder {
        crate::types::builders::TrialComponentSourceBuilder::default()
    }
}

/// A builder for [`TrialComponentSource`](crate::types::TrialComponentSource).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TrialComponentSourceBuilder {
    pub(crate) source_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source_type: ::std::option::Option<::std::string::String>,
}
impl TrialComponentSourceBuilder {
    /// <p>The source Amazon Resource Name (ARN).</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source Amazon Resource Name (ARN).</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_arn = input;
        self
    }
    /// <p>The source job type.</p>
    pub fn source_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source job type.</p>
    pub fn set_source_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_type = input;
        self
    }
    /// Consumes the builder and constructs a [`TrialComponentSource`](crate::types::TrialComponentSource).
    pub fn build(self) -> crate::types::TrialComponentSource {
        crate::types::TrialComponentSource {
            source_arn: self.source_arn,
            source_type: self.source_type,
        }
    }
}
