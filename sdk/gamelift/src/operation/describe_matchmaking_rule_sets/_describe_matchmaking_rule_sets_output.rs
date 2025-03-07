// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeMatchmakingRuleSetsOutput {
    /// <p>A collection of requested matchmaking rule set objects. </p>
    #[doc(hidden)]
    pub rule_sets: ::std::option::Option<::std::vec::Vec<crate::types::MatchmakingRuleSet>>,
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeMatchmakingRuleSetsOutput {
    /// <p>A collection of requested matchmaking rule set objects. </p>
    pub fn rule_sets(&self) -> ::std::option::Option<&[crate::types::MatchmakingRuleSet]> {
        self.rule_sets.as_deref()
    }
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeMatchmakingRuleSetsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeMatchmakingRuleSetsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeMatchmakingRuleSetsOutput`](crate::operation::describe_matchmaking_rule_sets::DescribeMatchmakingRuleSetsOutput).
    pub fn builder() -> crate::operation::describe_matchmaking_rule_sets::builders::DescribeMatchmakingRuleSetsOutputBuilder{
        crate::operation::describe_matchmaking_rule_sets::builders::DescribeMatchmakingRuleSetsOutputBuilder::default()
    }
}

/// A builder for [`DescribeMatchmakingRuleSetsOutput`](crate::operation::describe_matchmaking_rule_sets::DescribeMatchmakingRuleSetsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeMatchmakingRuleSetsOutputBuilder {
    pub(crate) rule_sets: ::std::option::Option<::std::vec::Vec<crate::types::MatchmakingRuleSet>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeMatchmakingRuleSetsOutputBuilder {
    /// Appends an item to `rule_sets`.
    ///
    /// To override the contents of this collection use [`set_rule_sets`](Self::set_rule_sets).
    ///
    /// <p>A collection of requested matchmaking rule set objects. </p>
    pub fn rule_sets(mut self, input: crate::types::MatchmakingRuleSet) -> Self {
        let mut v = self.rule_sets.unwrap_or_default();
        v.push(input);
        self.rule_sets = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of requested matchmaking rule set objects. </p>
    pub fn set_rule_sets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MatchmakingRuleSet>>,
    ) -> Self {
        self.rule_sets = input;
        self
    }
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
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
    /// Consumes the builder and constructs a [`DescribeMatchmakingRuleSetsOutput`](crate::operation::describe_matchmaking_rule_sets::DescribeMatchmakingRuleSetsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_matchmaking_rule_sets::DescribeMatchmakingRuleSetsOutput {
        crate::operation::describe_matchmaking_rule_sets::DescribeMatchmakingRuleSetsOutput {
            rule_sets: self.rule_sets,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
