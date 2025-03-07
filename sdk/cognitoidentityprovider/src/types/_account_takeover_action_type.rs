// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Account takeover action type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccountTakeoverActionType {
    /// <p>Flag specifying whether to send a notification.</p>
    #[doc(hidden)]
    pub notify: bool,
    /// <p>The action to take in response to the account takeover action. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>BLOCK</code> Choosing this action will block the request.</p> </li>
    /// <li> <p> <code>MFA_IF_CONFIGURED</code> Present an MFA challenge if user has configured it, else allow the request.</p> </li>
    /// <li> <p> <code>MFA_REQUIRED</code> Present an MFA challenge if user has configured it, else block the request.</p> </li>
    /// <li> <p> <code>NO_ACTION</code> Allow the user to sign in.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub event_action: ::std::option::Option<crate::types::AccountTakeoverEventActionType>,
}
impl AccountTakeoverActionType {
    /// <p>Flag specifying whether to send a notification.</p>
    pub fn notify(&self) -> bool {
        self.notify
    }
    /// <p>The action to take in response to the account takeover action. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>BLOCK</code> Choosing this action will block the request.</p> </li>
    /// <li> <p> <code>MFA_IF_CONFIGURED</code> Present an MFA challenge if user has configured it, else allow the request.</p> </li>
    /// <li> <p> <code>MFA_REQUIRED</code> Present an MFA challenge if user has configured it, else block the request.</p> </li>
    /// <li> <p> <code>NO_ACTION</code> Allow the user to sign in.</p> </li>
    /// </ul>
    pub fn event_action(
        &self,
    ) -> ::std::option::Option<&crate::types::AccountTakeoverEventActionType> {
        self.event_action.as_ref()
    }
}
impl AccountTakeoverActionType {
    /// Creates a new builder-style object to manufacture [`AccountTakeoverActionType`](crate::types::AccountTakeoverActionType).
    pub fn builder() -> crate::types::builders::AccountTakeoverActionTypeBuilder {
        crate::types::builders::AccountTakeoverActionTypeBuilder::default()
    }
}

/// A builder for [`AccountTakeoverActionType`](crate::types::AccountTakeoverActionType).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccountTakeoverActionTypeBuilder {
    pub(crate) notify: ::std::option::Option<bool>,
    pub(crate) event_action: ::std::option::Option<crate::types::AccountTakeoverEventActionType>,
}
impl AccountTakeoverActionTypeBuilder {
    /// <p>Flag specifying whether to send a notification.</p>
    pub fn notify(mut self, input: bool) -> Self {
        self.notify = ::std::option::Option::Some(input);
        self
    }
    /// <p>Flag specifying whether to send a notification.</p>
    pub fn set_notify(mut self, input: ::std::option::Option<bool>) -> Self {
        self.notify = input;
        self
    }
    /// <p>The action to take in response to the account takeover action. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>BLOCK</code> Choosing this action will block the request.</p> </li>
    /// <li> <p> <code>MFA_IF_CONFIGURED</code> Present an MFA challenge if user has configured it, else allow the request.</p> </li>
    /// <li> <p> <code>MFA_REQUIRED</code> Present an MFA challenge if user has configured it, else block the request.</p> </li>
    /// <li> <p> <code>NO_ACTION</code> Allow the user to sign in.</p> </li>
    /// </ul>
    pub fn event_action(mut self, input: crate::types::AccountTakeoverEventActionType) -> Self {
        self.event_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>The action to take in response to the account takeover action. Valid values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>BLOCK</code> Choosing this action will block the request.</p> </li>
    /// <li> <p> <code>MFA_IF_CONFIGURED</code> Present an MFA challenge if user has configured it, else allow the request.</p> </li>
    /// <li> <p> <code>MFA_REQUIRED</code> Present an MFA challenge if user has configured it, else block the request.</p> </li>
    /// <li> <p> <code>NO_ACTION</code> Allow the user to sign in.</p> </li>
    /// </ul>
    pub fn set_event_action(
        mut self,
        input: ::std::option::Option<crate::types::AccountTakeoverEventActionType>,
    ) -> Self {
        self.event_action = input;
        self
    }
    /// Consumes the builder and constructs a [`AccountTakeoverActionType`](crate::types::AccountTakeoverActionType).
    pub fn build(self) -> crate::types::AccountTakeoverActionType {
        crate::types::AccountTakeoverActionType {
            notify: self.notify.unwrap_or_default(),
            event_action: self.event_action,
        }
    }
}
