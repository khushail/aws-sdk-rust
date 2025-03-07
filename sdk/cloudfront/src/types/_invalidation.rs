// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An invalidation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Invalidation {
    /// <p>The identifier for the invalidation request. For example: <code>IDFDVBD632BHDS5</code>.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the invalidation request. When the invalidation batch is finished, the status is <code>Completed</code>.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The date and time the invalidation request was first made.</p>
    #[doc(hidden)]
    pub create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The current invalidation information for the batch request.</p>
    #[doc(hidden)]
    pub invalidation_batch: ::std::option::Option<crate::types::InvalidationBatch>,
}
impl Invalidation {
    /// <p>The identifier for the invalidation request. For example: <code>IDFDVBD632BHDS5</code>.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The status of the invalidation request. When the invalidation batch is finished, the status is <code>Completed</code>.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The date and time the invalidation request was first made.</p>
    pub fn create_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>The current invalidation information for the batch request.</p>
    pub fn invalidation_batch(&self) -> ::std::option::Option<&crate::types::InvalidationBatch> {
        self.invalidation_batch.as_ref()
    }
}
impl Invalidation {
    /// Creates a new builder-style object to manufacture [`Invalidation`](crate::types::Invalidation).
    pub fn builder() -> crate::types::builders::InvalidationBuilder {
        crate::types::builders::InvalidationBuilder::default()
    }
}

/// A builder for [`Invalidation`](crate::types::Invalidation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InvalidationBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) invalidation_batch: ::std::option::Option<crate::types::InvalidationBatch>,
}
impl InvalidationBuilder {
    /// <p>The identifier for the invalidation request. For example: <code>IDFDVBD632BHDS5</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the invalidation request. For example: <code>IDFDVBD632BHDS5</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The status of the invalidation request. When the invalidation batch is finished, the status is <code>Completed</code>.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of the invalidation request. When the invalidation batch is finished, the status is <code>Completed</code>.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The date and time the invalidation request was first made.</p>
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the invalidation request was first made.</p>
    pub fn set_create_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The current invalidation information for the batch request.</p>
    pub fn invalidation_batch(mut self, input: crate::types::InvalidationBatch) -> Self {
        self.invalidation_batch = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current invalidation information for the batch request.</p>
    pub fn set_invalidation_batch(
        mut self,
        input: ::std::option::Option<crate::types::InvalidationBatch>,
    ) -> Self {
        self.invalidation_batch = input;
        self
    }
    /// Consumes the builder and constructs a [`Invalidation`](crate::types::Invalidation).
    pub fn build(self) -> crate::types::Invalidation {
        crate::types::Invalidation {
            id: self.id,
            status: self.status,
            create_time: self.create_time,
            invalidation_batch: self.invalidation_batch,
        }
    }
}
