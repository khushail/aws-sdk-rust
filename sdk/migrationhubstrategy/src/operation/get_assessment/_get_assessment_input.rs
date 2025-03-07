// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAssessmentInput {
    /// <p> The <code>assessmentid</code> returned by <code>StartAssessment</code>.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetAssessmentInput {
    /// <p> The <code>assessmentid</code> returned by <code>StartAssessment</code>.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetAssessmentInput {
    /// Creates a new builder-style object to manufacture [`GetAssessmentInput`](crate::operation::get_assessment::GetAssessmentInput).
    pub fn builder() -> crate::operation::get_assessment::builders::GetAssessmentInputBuilder {
        crate::operation::get_assessment::builders::GetAssessmentInputBuilder::default()
    }
}

/// A builder for [`GetAssessmentInput`](crate::operation::get_assessment::GetAssessmentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAssessmentInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetAssessmentInputBuilder {
    /// <p> The <code>assessmentid</code> returned by <code>StartAssessment</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The <code>assessmentid</code> returned by <code>StartAssessment</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetAssessmentInput`](crate::operation::get_assessment::GetAssessmentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_assessment::GetAssessmentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_assessment::GetAssessmentInput {
            id: self.id,
        })
    }
}
