// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents the output of a ListRuleGroupsNamespaces operation.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListRuleGroupsNamespacesOutput {
    /// The list of the selected rule groups namespaces.
    #[doc(hidden)]
    pub rule_groups_namespaces:
        ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupsNamespaceSummary>>,
    /// Pagination token to use when requesting the next page in this list.
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListRuleGroupsNamespacesOutput {
    /// The list of the selected rule groups namespaces.
    pub fn rule_groups_namespaces(
        &self,
    ) -> ::std::option::Option<&[crate::types::RuleGroupsNamespaceSummary]> {
        self.rule_groups_namespaces.as_deref()
    }
    /// Pagination token to use when requesting the next page in this list.
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListRuleGroupsNamespacesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListRuleGroupsNamespacesOutput {
    /// Creates a new builder-style object to manufacture [`ListRuleGroupsNamespacesOutput`](crate::operation::list_rule_groups_namespaces::ListRuleGroupsNamespacesOutput).
    pub fn builder() -> crate::operation::list_rule_groups_namespaces::builders::ListRuleGroupsNamespacesOutputBuilder{
        crate::operation::list_rule_groups_namespaces::builders::ListRuleGroupsNamespacesOutputBuilder::default()
    }
}

/// A builder for [`ListRuleGroupsNamespacesOutput`](crate::operation::list_rule_groups_namespaces::ListRuleGroupsNamespacesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListRuleGroupsNamespacesOutputBuilder {
    pub(crate) rule_groups_namespaces:
        ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupsNamespaceSummary>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListRuleGroupsNamespacesOutputBuilder {
    /// Appends an item to `rule_groups_namespaces`.
    ///
    /// To override the contents of this collection use [`set_rule_groups_namespaces`](Self::set_rule_groups_namespaces).
    ///
    /// The list of the selected rule groups namespaces.
    pub fn rule_groups_namespaces(
        mut self,
        input: crate::types::RuleGroupsNamespaceSummary,
    ) -> Self {
        let mut v = self.rule_groups_namespaces.unwrap_or_default();
        v.push(input);
        self.rule_groups_namespaces = ::std::option::Option::Some(v);
        self
    }
    /// The list of the selected rule groups namespaces.
    pub fn set_rule_groups_namespaces(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupsNamespaceSummary>>,
    ) -> Self {
        self.rule_groups_namespaces = input;
        self
    }
    /// Pagination token to use when requesting the next page in this list.
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// Pagination token to use when requesting the next page in this list.
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
    /// Consumes the builder and constructs a [`ListRuleGroupsNamespacesOutput`](crate::operation::list_rule_groups_namespaces::ListRuleGroupsNamespacesOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_rule_groups_namespaces::ListRuleGroupsNamespacesOutput {
        crate::operation::list_rule_groups_namespaces::ListRuleGroupsNamespacesOutput {
            rule_groups_namespaces: self.rule_groups_namespaces,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
