// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a rule.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Rule {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    #[doc(hidden)]
    pub rule_arn: ::std::option::Option<::std::string::String>,
    /// <p>The priority.</p>
    #[doc(hidden)]
    pub priority: ::std::option::Option<::std::string::String>,
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    #[doc(hidden)]
    pub conditions: ::std::option::Option<::std::vec::Vec<crate::types::RuleCondition>>,
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>redirect</code>, or <code>fixed-response</code>, and it must be the last action to be performed.</p>
    #[doc(hidden)]
    pub actions: ::std::option::Option<::std::vec::Vec<crate::types::Action>>,
    /// <p>Indicates whether this is the default rule.</p>
    #[doc(hidden)]
    pub is_default: bool,
}
impl Rule {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub fn rule_arn(&self) -> ::std::option::Option<&str> {
        self.rule_arn.as_deref()
    }
    /// <p>The priority.</p>
    pub fn priority(&self) -> ::std::option::Option<&str> {
        self.priority.as_deref()
    }
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    pub fn conditions(&self) -> ::std::option::Option<&[crate::types::RuleCondition]> {
        self.conditions.as_deref()
    }
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>redirect</code>, or <code>fixed-response</code>, and it must be the last action to be performed.</p>
    pub fn actions(&self) -> ::std::option::Option<&[crate::types::Action]> {
        self.actions.as_deref()
    }
    /// <p>Indicates whether this is the default rule.</p>
    pub fn is_default(&self) -> bool {
        self.is_default
    }
}
impl Rule {
    /// Creates a new builder-style object to manufacture [`Rule`](crate::types::Rule).
    pub fn builder() -> crate::types::builders::RuleBuilder {
        crate::types::builders::RuleBuilder::default()
    }
}

/// A builder for [`Rule`](crate::types::Rule).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RuleBuilder {
    pub(crate) rule_arn: ::std::option::Option<::std::string::String>,
    pub(crate) priority: ::std::option::Option<::std::string::String>,
    pub(crate) conditions: ::std::option::Option<::std::vec::Vec<crate::types::RuleCondition>>,
    pub(crate) actions: ::std::option::Option<::std::vec::Vec<crate::types::Action>>,
    pub(crate) is_default: ::std::option::Option<bool>,
}
impl RuleBuilder {
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub fn rule_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rule_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rule.</p>
    pub fn set_rule_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rule_arn = input;
        self
    }
    /// <p>The priority.</p>
    pub fn priority(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.priority = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The priority.</p>
    pub fn set_priority(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.priority = input;
        self
    }
    /// Appends an item to `conditions`.
    ///
    /// To override the contents of this collection use [`set_conditions`](Self::set_conditions).
    ///
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    pub fn conditions(mut self, input: crate::types::RuleCondition) -> Self {
        let mut v = self.conditions.unwrap_or_default();
        v.push(input);
        self.conditions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The conditions. Each rule can include zero or one of the following conditions: <code>http-request-method</code>, <code>host-header</code>, <code>path-pattern</code>, and <code>source-ip</code>, and zero or more of the following conditions: <code>http-header</code> and <code>query-string</code>.</p>
    pub fn set_conditions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RuleCondition>>,
    ) -> Self {
        self.conditions = input;
        self
    }
    /// Appends an item to `actions`.
    ///
    /// To override the contents of this collection use [`set_actions`](Self::set_actions).
    ///
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>redirect</code>, or <code>fixed-response</code>, and it must be the last action to be performed.</p>
    pub fn actions(mut self, input: crate::types::Action) -> Self {
        let mut v = self.actions.unwrap_or_default();
        v.push(input);
        self.actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The actions. Each rule must include exactly one of the following types of actions: <code>forward</code>, <code>redirect</code>, or <code>fixed-response</code>, and it must be the last action to be performed.</p>
    pub fn set_actions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Action>>,
    ) -> Self {
        self.actions = input;
        self
    }
    /// <p>Indicates whether this is the default rule.</p>
    pub fn is_default(mut self, input: bool) -> Self {
        self.is_default = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether this is the default rule.</p>
    pub fn set_is_default(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_default = input;
        self
    }
    /// Consumes the builder and constructs a [`Rule`](crate::types::Rule).
    pub fn build(self) -> crate::types::Rule {
        crate::types::Rule {
            rule_arn: self.rule_arn,
            priority: self.priority,
            conditions: self.conditions,
            actions: self.actions,
            is_default: self.is_default.unwrap_or_default(),
        }
    }
}
