// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A skill group with attributes.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SkillGroup {
    /// <p>The ARN of a skill group.</p>
    #[doc(hidden)]
    pub skill_group_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of a skill group.</p>
    #[doc(hidden)]
    pub skill_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The description of a skill group.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl SkillGroup {
    /// <p>The ARN of a skill group.</p>
    pub fn skill_group_arn(&self) -> ::std::option::Option<&str> {
        self.skill_group_arn.as_deref()
    }
    /// <p>The name of a skill group.</p>
    pub fn skill_group_name(&self) -> ::std::option::Option<&str> {
        self.skill_group_name.as_deref()
    }
    /// <p>The description of a skill group.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl SkillGroup {
    /// Creates a new builder-style object to manufacture [`SkillGroup`](crate::types::SkillGroup).
    pub fn builder() -> crate::types::builders::SkillGroupBuilder {
        crate::types::builders::SkillGroupBuilder::default()
    }
}

/// A builder for [`SkillGroup`](crate::types::SkillGroup).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SkillGroupBuilder {
    pub(crate) skill_group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) skill_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl SkillGroupBuilder {
    /// <p>The ARN of a skill group.</p>
    pub fn skill_group_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.skill_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of a skill group.</p>
    pub fn set_skill_group_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.skill_group_arn = input;
        self
    }
    /// <p>The name of a skill group.</p>
    pub fn skill_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.skill_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a skill group.</p>
    pub fn set_skill_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.skill_group_name = input;
        self
    }
    /// <p>The description of a skill group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of a skill group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`SkillGroup`](crate::types::SkillGroup).
    pub fn build(self) -> crate::types::SkillGroup {
        crate::types::SkillGroup {
            skill_group_arn: self.skill_group_arn,
            skill_group_name: self.skill_group_name,
            description: self.description,
        }
    }
}
