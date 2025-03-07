// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RejectSkillInput {
    /// <p>The unique identifier of the skill.</p>
    #[doc(hidden)]
    pub skill_id: ::std::option::Option<::std::string::String>,
}
impl RejectSkillInput {
    /// <p>The unique identifier of the skill.</p>
    pub fn skill_id(&self) -> ::std::option::Option<&str> {
        self.skill_id.as_deref()
    }
}
impl RejectSkillInput {
    /// Creates a new builder-style object to manufacture [`RejectSkillInput`](crate::operation::reject_skill::RejectSkillInput).
    pub fn builder() -> crate::operation::reject_skill::builders::RejectSkillInputBuilder {
        crate::operation::reject_skill::builders::RejectSkillInputBuilder::default()
    }
}

/// A builder for [`RejectSkillInput`](crate::operation::reject_skill::RejectSkillInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RejectSkillInputBuilder {
    pub(crate) skill_id: ::std::option::Option<::std::string::String>,
}
impl RejectSkillInputBuilder {
    /// <p>The unique identifier of the skill.</p>
    pub fn skill_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.skill_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the skill.</p>
    pub fn set_skill_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.skill_id = input;
        self
    }
    /// Consumes the builder and constructs a [`RejectSkillInput`](crate::operation::reject_skill::RejectSkillInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reject_skill::RejectSkillInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::reject_skill::RejectSkillInput {
            skill_id: self.skill_id,
        })
    }
}
