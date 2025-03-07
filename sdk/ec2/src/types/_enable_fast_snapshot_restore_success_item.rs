// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes fast snapshot restores that were successfully enabled.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableFastSnapshotRestoreSuccessItem {
    /// <p>The ID of the snapshot.</p>
    #[doc(hidden)]
    pub snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>The Availability Zone.</p>
    #[doc(hidden)]
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>The state of fast snapshot restores.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::FastSnapshotRestoreStateCode>,
    /// <p>The reason for the state transition. The possible values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>Client.UserInitiated</code> - The state successfully transitioned to <code>enabling</code> or <code>disabling</code>.</p> </li>
    /// <li> <p> <code>Client.UserInitiated - Lifecycle state transition</code> - The state successfully transitioned to <code>optimizing</code>, <code>enabled</code>, or <code>disabled</code>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub state_transition_reason: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that enabled fast snapshot restores on the snapshot.</p>
    #[doc(hidden)]
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services owner alias that enabled fast snapshot restores on the snapshot. This is intended for future use.</p>
    #[doc(hidden)]
    pub owner_alias: ::std::option::Option<::std::string::String>,
    /// <p>The time at which fast snapshot restores entered the <code>enabling</code> state.</p>
    #[doc(hidden)]
    pub enabling_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which fast snapshot restores entered the <code>optimizing</code> state.</p>
    #[doc(hidden)]
    pub optimizing_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which fast snapshot restores entered the <code>enabled</code> state.</p>
    #[doc(hidden)]
    pub enabled_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which fast snapshot restores entered the <code>disabling</code> state.</p>
    #[doc(hidden)]
    pub disabling_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time at which fast snapshot restores entered the <code>disabled</code> state.</p>
    #[doc(hidden)]
    pub disabled_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl EnableFastSnapshotRestoreSuccessItem {
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>The Availability Zone.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The state of fast snapshot restores.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::FastSnapshotRestoreStateCode> {
        self.state.as_ref()
    }
    /// <p>The reason for the state transition. The possible values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>Client.UserInitiated</code> - The state successfully transitioned to <code>enabling</code> or <code>disabling</code>.</p> </li>
    /// <li> <p> <code>Client.UserInitiated - Lifecycle state transition</code> - The state successfully transitioned to <code>optimizing</code>, <code>enabled</code>, or <code>disabled</code>.</p> </li>
    /// </ul>
    pub fn state_transition_reason(&self) -> ::std::option::Option<&str> {
        self.state_transition_reason.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that enabled fast snapshot restores on the snapshot.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The Amazon Web Services owner alias that enabled fast snapshot restores on the snapshot. This is intended for future use.</p>
    pub fn owner_alias(&self) -> ::std::option::Option<&str> {
        self.owner_alias.as_deref()
    }
    /// <p>The time at which fast snapshot restores entered the <code>enabling</code> state.</p>
    pub fn enabling_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.enabling_time.as_ref()
    }
    /// <p>The time at which fast snapshot restores entered the <code>optimizing</code> state.</p>
    pub fn optimizing_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.optimizing_time.as_ref()
    }
    /// <p>The time at which fast snapshot restores entered the <code>enabled</code> state.</p>
    pub fn enabled_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.enabled_time.as_ref()
    }
    /// <p>The time at which fast snapshot restores entered the <code>disabling</code> state.</p>
    pub fn disabling_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.disabling_time.as_ref()
    }
    /// <p>The time at which fast snapshot restores entered the <code>disabled</code> state.</p>
    pub fn disabled_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.disabled_time.as_ref()
    }
}
impl EnableFastSnapshotRestoreSuccessItem {
    /// Creates a new builder-style object to manufacture [`EnableFastSnapshotRestoreSuccessItem`](crate::types::EnableFastSnapshotRestoreSuccessItem).
    pub fn builder() -> crate::types::builders::EnableFastSnapshotRestoreSuccessItemBuilder {
        crate::types::builders::EnableFastSnapshotRestoreSuccessItemBuilder::default()
    }
}

/// A builder for [`EnableFastSnapshotRestoreSuccessItem`](crate::types::EnableFastSnapshotRestoreSuccessItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnableFastSnapshotRestoreSuccessItemBuilder {
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::FastSnapshotRestoreStateCode>,
    pub(crate) state_transition_reason: ::std::option::Option<::std::string::String>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) owner_alias: ::std::option::Option<::std::string::String>,
    pub(crate) enabling_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) optimizing_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) enabled_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) disabling_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) disabled_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl EnableFastSnapshotRestoreSuccessItemBuilder {
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the snapshot.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// <p>The Availability Zone.</p>
    pub fn availability_zone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone.</p>
    pub fn set_availability_zone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The state of fast snapshot restores.</p>
    pub fn state(mut self, input: crate::types::FastSnapshotRestoreStateCode) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of fast snapshot restores.</p>
    pub fn set_state(
        mut self,
        input: ::std::option::Option<crate::types::FastSnapshotRestoreStateCode>,
    ) -> Self {
        self.state = input;
        self
    }
    /// <p>The reason for the state transition. The possible values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>Client.UserInitiated</code> - The state successfully transitioned to <code>enabling</code> or <code>disabling</code>.</p> </li>
    /// <li> <p> <code>Client.UserInitiated - Lifecycle state transition</code> - The state successfully transitioned to <code>optimizing</code>, <code>enabled</code>, or <code>disabled</code>.</p> </li>
    /// </ul>
    pub fn state_transition_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.state_transition_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for the state transition. The possible values are as follows:</p>
    /// <ul>
    /// <li> <p> <code>Client.UserInitiated</code> - The state successfully transitioned to <code>enabling</code> or <code>disabling</code>.</p> </li>
    /// <li> <p> <code>Client.UserInitiated - Lifecycle state transition</code> - The state successfully transitioned to <code>optimizing</code>, <code>enabled</code>, or <code>disabled</code>.</p> </li>
    /// </ul>
    pub fn set_state_transition_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.state_transition_reason = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that enabled fast snapshot restores on the snapshot.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that enabled fast snapshot restores on the snapshot.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The Amazon Web Services owner alias that enabled fast snapshot restores on the snapshot. This is intended for future use.</p>
    pub fn owner_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services owner alias that enabled fast snapshot restores on the snapshot. This is intended for future use.</p>
    pub fn set_owner_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_alias = input;
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>enabling</code> state.</p>
    pub fn enabling_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.enabling_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>enabling</code> state.</p>
    pub fn set_enabling_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.enabling_time = input;
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>optimizing</code> state.</p>
    pub fn optimizing_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.optimizing_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>optimizing</code> state.</p>
    pub fn set_optimizing_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.optimizing_time = input;
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>enabled</code> state.</p>
    pub fn enabled_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.enabled_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>enabled</code> state.</p>
    pub fn set_enabled_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.enabled_time = input;
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>disabling</code> state.</p>
    pub fn disabling_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.disabling_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>disabling</code> state.</p>
    pub fn set_disabling_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.disabling_time = input;
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>disabled</code> state.</p>
    pub fn disabled_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.disabled_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time at which fast snapshot restores entered the <code>disabled</code> state.</p>
    pub fn set_disabled_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.disabled_time = input;
        self
    }
    /// Consumes the builder and constructs a [`EnableFastSnapshotRestoreSuccessItem`](crate::types::EnableFastSnapshotRestoreSuccessItem).
    pub fn build(self) -> crate::types::EnableFastSnapshotRestoreSuccessItem {
        crate::types::EnableFastSnapshotRestoreSuccessItem {
            snapshot_id: self.snapshot_id,
            availability_zone: self.availability_zone,
            state: self.state,
            state_transition_reason: self.state_transition_reason,
            owner_id: self.owner_id,
            owner_alias: self.owner_alias,
            enabling_time: self.enabling_time,
            optimizing_time: self.optimizing_time,
            enabled_time: self.enabled_time,
            disabling_time: self.disabling_time,
            disabled_time: self.disabled_time,
        }
    }
}
