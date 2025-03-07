// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSkillsOutput {
    /// <p>The list of enabled skills requested. Required.</p>
    #[doc(hidden)]
    pub skill_summaries: ::std::option::Option<::std::vec::Vec<crate::types::SkillSummary>>,
    /// <p>The token returned to indicate that there is more data available.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSkillsOutput {
    /// <p>The list of enabled skills requested. Required.</p>
    pub fn skill_summaries(&self) -> ::std::option::Option<&[crate::types::SkillSummary]> {
        self.skill_summaries.as_deref()
    }
    /// <p>The token returned to indicate that there is more data available.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListSkillsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListSkillsOutput {
    /// Creates a new builder-style object to manufacture [`ListSkillsOutput`](crate::operation::list_skills::ListSkillsOutput).
    pub fn builder() -> crate::operation::list_skills::builders::ListSkillsOutputBuilder {
        crate::operation::list_skills::builders::ListSkillsOutputBuilder::default()
    }
}

/// A builder for [`ListSkillsOutput`](crate::operation::list_skills::ListSkillsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSkillsOutputBuilder {
    pub(crate) skill_summaries: ::std::option::Option<::std::vec::Vec<crate::types::SkillSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListSkillsOutputBuilder {
    /// Appends an item to `skill_summaries`.
    ///
    /// To override the contents of this collection use [`set_skill_summaries`](Self::set_skill_summaries).
    ///
    /// <p>The list of enabled skills requested. Required.</p>
    pub fn skill_summaries(mut self, input: crate::types::SkillSummary) -> Self {
        let mut v = self.skill_summaries.unwrap_or_default();
        v.push(input);
        self.skill_summaries = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of enabled skills requested. Required.</p>
    pub fn set_skill_summaries(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SkillSummary>>,
    ) -> Self {
        self.skill_summaries = input;
        self
    }
    /// <p>The token returned to indicate that there is more data available.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned to indicate that there is more data available.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListSkillsOutput`](crate::operation::list_skills::ListSkillsOutput).
    pub fn build(self) -> crate::operation::list_skills::ListSkillsOutput {
        crate::operation::list_skills::ListSkillsOutput {
            skill_summaries: self.skill_summaries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
