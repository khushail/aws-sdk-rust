// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetQualificationScoreInput {
    /// <p>The ID of the QualificationType.</p>
    #[doc(hidden)]
    pub qualification_type_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    #[doc(hidden)]
    pub worker_id: ::std::option::Option<::std::string::String>,
}
impl GetQualificationScoreInput {
    /// <p>The ID of the QualificationType.</p>
    pub fn qualification_type_id(&self) -> ::std::option::Option<&str> {
        self.qualification_type_id.as_deref()
    }
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    pub fn worker_id(&self) -> ::std::option::Option<&str> {
        self.worker_id.as_deref()
    }
}
impl GetQualificationScoreInput {
    /// Creates a new builder-style object to manufacture [`GetQualificationScoreInput`](crate::operation::get_qualification_score::GetQualificationScoreInput).
    pub fn builder(
    ) -> crate::operation::get_qualification_score::builders::GetQualificationScoreInputBuilder
    {
        crate::operation::get_qualification_score::builders::GetQualificationScoreInputBuilder::default()
    }
}

/// A builder for [`GetQualificationScoreInput`](crate::operation::get_qualification_score::GetQualificationScoreInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetQualificationScoreInputBuilder {
    pub(crate) qualification_type_id: ::std::option::Option<::std::string::String>,
    pub(crate) worker_id: ::std::option::Option<::std::string::String>,
}
impl GetQualificationScoreInputBuilder {
    /// <p>The ID of the QualificationType.</p>
    pub fn qualification_type_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.qualification_type_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the QualificationType.</p>
    pub fn set_qualification_type_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.qualification_type_id = input;
        self
    }
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    pub fn worker_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.worker_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Worker whose Qualification is being updated.</p>
    pub fn set_worker_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.worker_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetQualificationScoreInput`](crate::operation::get_qualification_score::GetQualificationScoreInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_qualification_score::GetQualificationScoreInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_qualification_score::GetQualificationScoreInput {
                qualification_type_id: self.qualification_type_id,
                worker_id: self.worker_id,
            },
        )
    }
}
