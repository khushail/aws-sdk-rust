// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchSkillGroupsInput {
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The filters to use to list a specified set of skill groups. The supported filter key is SkillGroupName. </p>
    #[doc(hidden)]
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The sort order to use in listing the specified set of skill groups. The supported sort key is SkillGroupName. </p>
    #[doc(hidden)]
    pub sort_criteria: ::std::option::Option<::std::vec::Vec<crate::types::Sort>>,
}
impl SearchSkillGroupsInput {
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The filters to use to list a specified set of skill groups. The supported filter key is SkillGroupName. </p>
    pub fn filters(&self) -> ::std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The sort order to use in listing the specified set of skill groups. The supported sort key is SkillGroupName. </p>
    pub fn sort_criteria(&self) -> ::std::option::Option<&[crate::types::Sort]> {
        self.sort_criteria.as_deref()
    }
}
impl SearchSkillGroupsInput {
    /// Creates a new builder-style object to manufacture [`SearchSkillGroupsInput`](crate::operation::search_skill_groups::SearchSkillGroupsInput).
    pub fn builder(
    ) -> crate::operation::search_skill_groups::builders::SearchSkillGroupsInputBuilder {
        crate::operation::search_skill_groups::builders::SearchSkillGroupsInputBuilder::default()
    }
}

/// A builder for [`SearchSkillGroupsInput`](crate::operation::search_skill_groups::SearchSkillGroupsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SearchSkillGroupsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) sort_criteria: ::std::option::Option<::std::vec::Vec<crate::types::Sort>>,
}
impl SearchSkillGroupsInputBuilder {
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional token returned from a prior request. Use this token for pagination of results from this action. If this parameter is specified, the response includes only results beyond the token, up to the value specified by <code>MaxResults</code>. Required.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved. </p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters to use to list a specified set of skill groups. The supported filter key is SkillGroupName. </p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters to use to list a specified set of skill groups. The supported filter key is SkillGroupName. </p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// Appends an item to `sort_criteria`.
    ///
    /// To override the contents of this collection use [`set_sort_criteria`](Self::set_sort_criteria).
    ///
    /// <p>The sort order to use in listing the specified set of skill groups. The supported sort key is SkillGroupName. </p>
    pub fn sort_criteria(mut self, input: crate::types::Sort) -> Self {
        let mut v = self.sort_criteria.unwrap_or_default();
        v.push(input);
        self.sort_criteria = ::std::option::Option::Some(v);
        self
    }
    /// <p>The sort order to use in listing the specified set of skill groups. The supported sort key is SkillGroupName. </p>
    pub fn set_sort_criteria(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Sort>>,
    ) -> Self {
        self.sort_criteria = input;
        self
    }
    /// Consumes the builder and constructs a [`SearchSkillGroupsInput`](crate::operation::search_skill_groups::SearchSkillGroupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::search_skill_groups::SearchSkillGroupsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::search_skill_groups::SearchSkillGroupsInput {
                next_token: self.next_token,
                max_results: self.max_results,
                filters: self.filters,
                sort_criteria: self.sort_criteria,
            },
        )
    }
}
