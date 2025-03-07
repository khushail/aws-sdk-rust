// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the use case.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UseCase {
    /// <p>The identifier for the use case.</p>
    #[doc(hidden)]
    pub use_case_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the use case.</p>
    #[doc(hidden)]
    pub use_case_arn: ::std::option::Option<::std::string::String>,
    /// <p>The type of use case to associate to the integration association. Each integration association can have only one of each use case type.</p>
    #[doc(hidden)]
    pub use_case_type: ::std::option::Option<crate::types::UseCaseType>,
}
impl UseCase {
    /// <p>The identifier for the use case.</p>
    pub fn use_case_id(&self) -> ::std::option::Option<&str> {
        self.use_case_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the use case.</p>
    pub fn use_case_arn(&self) -> ::std::option::Option<&str> {
        self.use_case_arn.as_deref()
    }
    /// <p>The type of use case to associate to the integration association. Each integration association can have only one of each use case type.</p>
    pub fn use_case_type(&self) -> ::std::option::Option<&crate::types::UseCaseType> {
        self.use_case_type.as_ref()
    }
}
impl UseCase {
    /// Creates a new builder-style object to manufacture [`UseCase`](crate::types::UseCase).
    pub fn builder() -> crate::types::builders::UseCaseBuilder {
        crate::types::builders::UseCaseBuilder::default()
    }
}

/// A builder for [`UseCase`](crate::types::UseCase).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UseCaseBuilder {
    pub(crate) use_case_id: ::std::option::Option<::std::string::String>,
    pub(crate) use_case_arn: ::std::option::Option<::std::string::String>,
    pub(crate) use_case_type: ::std::option::Option<crate::types::UseCaseType>,
}
impl UseCaseBuilder {
    /// <p>The identifier for the use case.</p>
    pub fn use_case_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.use_case_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the use case.</p>
    pub fn set_use_case_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.use_case_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the use case.</p>
    pub fn use_case_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.use_case_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the use case.</p>
    pub fn set_use_case_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.use_case_arn = input;
        self
    }
    /// <p>The type of use case to associate to the integration association. Each integration association can have only one of each use case type.</p>
    pub fn use_case_type(mut self, input: crate::types::UseCaseType) -> Self {
        self.use_case_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of use case to associate to the integration association. Each integration association can have only one of each use case type.</p>
    pub fn set_use_case_type(
        mut self,
        input: ::std::option::Option<crate::types::UseCaseType>,
    ) -> Self {
        self.use_case_type = input;
        self
    }
    /// Consumes the builder and constructs a [`UseCase`](crate::types::UseCase).
    pub fn build(self) -> crate::types::UseCase {
        crate::types::UseCase {
            use_case_id: self.use_case_id,
            use_case_arn: self.use_case_arn,
            use_case_type: self.use_case_type,
        }
    }
}
