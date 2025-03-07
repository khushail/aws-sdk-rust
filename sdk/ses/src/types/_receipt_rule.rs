// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Receipt rules enable you to specify which actions Amazon SES should take when it receives mail on behalf of one or more email addresses or domains that you own.</p>
/// <p>Each receipt rule defines a set of email addresses or domains that it applies to. If the email addresses or domains match at least one recipient address of the message, Amazon SES executes all of the receipt rule's actions on the message.</p>
/// <p>For information about setting up receipt rules, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-receipt-rules.html">Amazon SES Developer Guide</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReceiptRule {
    /// <p>The name of the receipt rule. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>If <code>true</code>, the receipt rule is active. The default value is <code>false</code>.</p>
    #[doc(hidden)]
    pub enabled: bool,
    /// <p>Specifies whether Amazon SES should require that incoming email is delivered over a connection encrypted with Transport Layer Security (TLS). If this parameter is set to <code>Require</code>, Amazon SES will bounce emails that are not received over TLS. The default is <code>Optional</code>.</p>
    #[doc(hidden)]
    pub tls_policy: ::std::option::Option<crate::types::TlsPolicy>,
    /// <p>The recipient domains and email addresses that the receipt rule applies to. If this field is not specified, this rule will match all recipients under all verified domains.</p>
    #[doc(hidden)]
    pub recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An ordered list of actions to perform on messages that match at least one of the recipient email addresses or domains specified in the receipt rule.</p>
    #[doc(hidden)]
    pub actions: ::std::option::Option<::std::vec::Vec<crate::types::ReceiptAction>>,
    /// <p>If <code>true</code>, then messages that this receipt rule applies to are scanned for spam and viruses. The default value is <code>false</code>.</p>
    #[doc(hidden)]
    pub scan_enabled: bool,
}
impl ReceiptRule {
    /// <p>The name of the receipt rule. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>If <code>true</code>, the receipt rule is active. The default value is <code>false</code>.</p>
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    /// <p>Specifies whether Amazon SES should require that incoming email is delivered over a connection encrypted with Transport Layer Security (TLS). If this parameter is set to <code>Require</code>, Amazon SES will bounce emails that are not received over TLS. The default is <code>Optional</code>.</p>
    pub fn tls_policy(&self) -> ::std::option::Option<&crate::types::TlsPolicy> {
        self.tls_policy.as_ref()
    }
    /// <p>The recipient domains and email addresses that the receipt rule applies to. If this field is not specified, this rule will match all recipients under all verified domains.</p>
    pub fn recipients(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.recipients.as_deref()
    }
    /// <p>An ordered list of actions to perform on messages that match at least one of the recipient email addresses or domains specified in the receipt rule.</p>
    pub fn actions(&self) -> ::std::option::Option<&[crate::types::ReceiptAction]> {
        self.actions.as_deref()
    }
    /// <p>If <code>true</code>, then messages that this receipt rule applies to are scanned for spam and viruses. The default value is <code>false</code>.</p>
    pub fn scan_enabled(&self) -> bool {
        self.scan_enabled
    }
}
impl ReceiptRule {
    /// Creates a new builder-style object to manufacture [`ReceiptRule`](crate::types::ReceiptRule).
    pub fn builder() -> crate::types::builders::ReceiptRuleBuilder {
        crate::types::builders::ReceiptRuleBuilder::default()
    }
}

/// A builder for [`ReceiptRule`](crate::types::ReceiptRule).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReceiptRuleBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) tls_policy: ::std::option::Option<crate::types::TlsPolicy>,
    pub(crate) recipients: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) actions: ::std::option::Option<::std::vec::Vec<crate::types::ReceiptAction>>,
    pub(crate) scan_enabled: ::std::option::Option<bool>,
}
impl ReceiptRuleBuilder {
    /// <p>The name of the receipt rule. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the receipt rule. The name must:</p>
    /// <ul>
    /// <li> <p>This value can only contain ASCII letters (a-z, A-Z), numbers (0-9), underscores (_), or dashes (-).</p> </li>
    /// <li> <p>Start and end with a letter or number.</p> </li>
    /// <li> <p>Contain less than 64 characters.</p> </li>
    /// </ul>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>If <code>true</code>, the receipt rule is active. The default value is <code>false</code>.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>If <code>true</code>, the receipt rule is active. The default value is <code>false</code>.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Specifies whether Amazon SES should require that incoming email is delivered over a connection encrypted with Transport Layer Security (TLS). If this parameter is set to <code>Require</code>, Amazon SES will bounce emails that are not received over TLS. The default is <code>Optional</code>.</p>
    pub fn tls_policy(mut self, input: crate::types::TlsPolicy) -> Self {
        self.tls_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon SES should require that incoming email is delivered over a connection encrypted with Transport Layer Security (TLS). If this parameter is set to <code>Require</code>, Amazon SES will bounce emails that are not received over TLS. The default is <code>Optional</code>.</p>
    pub fn set_tls_policy(mut self, input: ::std::option::Option<crate::types::TlsPolicy>) -> Self {
        self.tls_policy = input;
        self
    }
    /// Appends an item to `recipients`.
    ///
    /// To override the contents of this collection use [`set_recipients`](Self::set_recipients).
    ///
    /// <p>The recipient domains and email addresses that the receipt rule applies to. If this field is not specified, this rule will match all recipients under all verified domains.</p>
    pub fn recipients(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.recipients.unwrap_or_default();
        v.push(input.into());
        self.recipients = ::std::option::Option::Some(v);
        self
    }
    /// <p>The recipient domains and email addresses that the receipt rule applies to. If this field is not specified, this rule will match all recipients under all verified domains.</p>
    pub fn set_recipients(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.recipients = input;
        self
    }
    /// Appends an item to `actions`.
    ///
    /// To override the contents of this collection use [`set_actions`](Self::set_actions).
    ///
    /// <p>An ordered list of actions to perform on messages that match at least one of the recipient email addresses or domains specified in the receipt rule.</p>
    pub fn actions(mut self, input: crate::types::ReceiptAction) -> Self {
        let mut v = self.actions.unwrap_or_default();
        v.push(input);
        self.actions = ::std::option::Option::Some(v);
        self
    }
    /// <p>An ordered list of actions to perform on messages that match at least one of the recipient email addresses or domains specified in the receipt rule.</p>
    pub fn set_actions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ReceiptAction>>,
    ) -> Self {
        self.actions = input;
        self
    }
    /// <p>If <code>true</code>, then messages that this receipt rule applies to are scanned for spam and viruses. The default value is <code>false</code>.</p>
    pub fn scan_enabled(mut self, input: bool) -> Self {
        self.scan_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>If <code>true</code>, then messages that this receipt rule applies to are scanned for spam and viruses. The default value is <code>false</code>.</p>
    pub fn set_scan_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.scan_enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`ReceiptRule`](crate::types::ReceiptRule).
    pub fn build(self) -> crate::types::ReceiptRule {
        crate::types::ReceiptRule {
            name: self.name,
            enabled: self.enabled.unwrap_or_default(),
            tls_policy: self.tls_policy,
            recipients: self.recipients,
            actions: self.actions,
            scan_enabled: self.scan_enabled.unwrap_or_default(),
        }
    }
}
