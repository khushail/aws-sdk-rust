// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListFirewallRuleGroupsOutput {
    /// <p>If objects are still available for retrieval, Resolver returns this token in the response. To retrieve the next batch of objects, provide this token in your next request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>A list of your firewall rule groups.</p>
    /// <p>This might be a partial list of the rule groups that you have defined. For information, see <code>MaxResults</code>. </p>
    #[doc(hidden)]
    pub firewall_rule_groups:
        ::std::option::Option<::std::vec::Vec<crate::types::FirewallRuleGroupMetadata>>,
    _request_id: Option<String>,
}
impl ListFirewallRuleGroupsOutput {
    /// <p>If objects are still available for retrieval, Resolver returns this token in the response. To retrieve the next batch of objects, provide this token in your next request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>A list of your firewall rule groups.</p>
    /// <p>This might be a partial list of the rule groups that you have defined. For information, see <code>MaxResults</code>. </p>
    pub fn firewall_rule_groups(
        &self,
    ) -> ::std::option::Option<&[crate::types::FirewallRuleGroupMetadata]> {
        self.firewall_rule_groups.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListFirewallRuleGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListFirewallRuleGroupsOutput {
    /// Creates a new builder-style object to manufacture [`ListFirewallRuleGroupsOutput`](crate::operation::list_firewall_rule_groups::ListFirewallRuleGroupsOutput).
    pub fn builder(
    ) -> crate::operation::list_firewall_rule_groups::builders::ListFirewallRuleGroupsOutputBuilder
    {
        crate::operation::list_firewall_rule_groups::builders::ListFirewallRuleGroupsOutputBuilder::default()
    }
}

/// A builder for [`ListFirewallRuleGroupsOutput`](crate::operation::list_firewall_rule_groups::ListFirewallRuleGroupsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListFirewallRuleGroupsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) firewall_rule_groups:
        ::std::option::Option<::std::vec::Vec<crate::types::FirewallRuleGroupMetadata>>,
    _request_id: Option<String>,
}
impl ListFirewallRuleGroupsOutputBuilder {
    /// <p>If objects are still available for retrieval, Resolver returns this token in the response. To retrieve the next batch of objects, provide this token in your next request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If objects are still available for retrieval, Resolver returns this token in the response. To retrieve the next batch of objects, provide this token in your next request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `firewall_rule_groups`.
    ///
    /// To override the contents of this collection use [`set_firewall_rule_groups`](Self::set_firewall_rule_groups).
    ///
    /// <p>A list of your firewall rule groups.</p>
    /// <p>This might be a partial list of the rule groups that you have defined. For information, see <code>MaxResults</code>. </p>
    pub fn firewall_rule_groups(mut self, input: crate::types::FirewallRuleGroupMetadata) -> Self {
        let mut v = self.firewall_rule_groups.unwrap_or_default();
        v.push(input);
        self.firewall_rule_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of your firewall rule groups.</p>
    /// <p>This might be a partial list of the rule groups that you have defined. For information, see <code>MaxResults</code>. </p>
    pub fn set_firewall_rule_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FirewallRuleGroupMetadata>>,
    ) -> Self {
        self.firewall_rule_groups = input;
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
    /// Consumes the builder and constructs a [`ListFirewallRuleGroupsOutput`](crate::operation::list_firewall_rule_groups::ListFirewallRuleGroupsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_firewall_rule_groups::ListFirewallRuleGroupsOutput {
        crate::operation::list_firewall_rule_groups::ListFirewallRuleGroupsOutput {
            next_token: self.next_token,
            firewall_rule_groups: self.firewall_rule_groups,
            _request_id: self._request_id,
        }
    }
}
