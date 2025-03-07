// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateRuleOutput {
    /// <p>Information about the rule.</p>
    #[doc(hidden)]
    pub rules: ::std::option::Option<::std::vec::Vec<crate::types::Rule>>,
    _request_id: Option<String>,
}
impl CreateRuleOutput {
    /// <p>Information about the rule.</p>
    pub fn rules(&self) -> ::std::option::Option<&[crate::types::Rule]> {
        self.rules.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateRuleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateRuleOutput {
    /// Creates a new builder-style object to manufacture [`CreateRuleOutput`](crate::operation::create_rule::CreateRuleOutput).
    pub fn builder() -> crate::operation::create_rule::builders::CreateRuleOutputBuilder {
        crate::operation::create_rule::builders::CreateRuleOutputBuilder::default()
    }
}

/// A builder for [`CreateRuleOutput`](crate::operation::create_rule::CreateRuleOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRuleOutputBuilder {
    pub(crate) rules: ::std::option::Option<::std::vec::Vec<crate::types::Rule>>,
    _request_id: Option<String>,
}
impl CreateRuleOutputBuilder {
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>Information about the rule.</p>
    pub fn rules(mut self, input: crate::types::Rule) -> Self {
        let mut v = self.rules.unwrap_or_default();
        v.push(input);
        self.rules = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the rule.</p>
    pub fn set_rules(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Rule>>,
    ) -> Self {
        self.rules = input;
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
    /// Consumes the builder and constructs a [`CreateRuleOutput`](crate::operation::create_rule::CreateRuleOutput).
    pub fn build(self) -> crate::operation::create_rule::CreateRuleOutput {
        crate::operation::create_rule::CreateRuleOutput {
            rules: self.rules,
            _request_id: self._request_id,
        }
    }
}
