// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>If a service instance is manually updated, Proton wants to prevent accidentally overriding a manual change.</p>
/// <p>A blocker is created because of the manual update or deletion of a service instance. The summary describes the blocker as being active or resolved.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceSyncBlockerSummary {
    /// <p>The name of the service that you want to get the sync blocker summary for. If given a service instance name and a service name, it will return the blockers only applying to the instance that is blocked.</p>
    /// <p>If given only a service name, it will return the blockers that apply to all of the instances. In order to get the blockers for a single instance, you will need to make two distinct calls, one to get the sync blocker summary for the service and the other to get the sync blocker for the service instance.</p>
    #[doc(hidden)]
    pub service_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service instance that you want sync your service configuration with.</p>
    #[doc(hidden)]
    pub service_instance_name: ::std::option::Option<::std::string::String>,
    /// <p>The latest active blockers for the synced service.</p>
    #[doc(hidden)]
    pub latest_blockers: ::std::option::Option<::std::vec::Vec<crate::types::SyncBlocker>>,
}
impl ServiceSyncBlockerSummary {
    /// <p>The name of the service that you want to get the sync blocker summary for. If given a service instance name and a service name, it will return the blockers only applying to the instance that is blocked.</p>
    /// <p>If given only a service name, it will return the blockers that apply to all of the instances. In order to get the blockers for a single instance, you will need to make two distinct calls, one to get the sync blocker summary for the service and the other to get the sync blocker for the service instance.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
    /// <p>The name of the service instance that you want sync your service configuration with.</p>
    pub fn service_instance_name(&self) -> ::std::option::Option<&str> {
        self.service_instance_name.as_deref()
    }
    /// <p>The latest active blockers for the synced service.</p>
    pub fn latest_blockers(&self) -> ::std::option::Option<&[crate::types::SyncBlocker]> {
        self.latest_blockers.as_deref()
    }
}
impl ServiceSyncBlockerSummary {
    /// Creates a new builder-style object to manufacture [`ServiceSyncBlockerSummary`](crate::types::ServiceSyncBlockerSummary).
    pub fn builder() -> crate::types::builders::ServiceSyncBlockerSummaryBuilder {
        crate::types::builders::ServiceSyncBlockerSummaryBuilder::default()
    }
}

/// A builder for [`ServiceSyncBlockerSummary`](crate::types::ServiceSyncBlockerSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ServiceSyncBlockerSummaryBuilder {
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
    pub(crate) service_instance_name: ::std::option::Option<::std::string::String>,
    pub(crate) latest_blockers: ::std::option::Option<::std::vec::Vec<crate::types::SyncBlocker>>,
}
impl ServiceSyncBlockerSummaryBuilder {
    /// <p>The name of the service that you want to get the sync blocker summary for. If given a service instance name and a service name, it will return the blockers only applying to the instance that is blocked.</p>
    /// <p>If given only a service name, it will return the blockers that apply to all of the instances. In order to get the blockers for a single instance, you will need to make two distinct calls, one to get the sync blocker summary for the service and the other to get the sync blocker for the service instance.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service that you want to get the sync blocker summary for. If given a service instance name and a service name, it will return the blockers only applying to the instance that is blocked.</p>
    /// <p>If given only a service name, it will return the blockers that apply to all of the instances. In order to get the blockers for a single instance, you will need to make two distinct calls, one to get the sync blocker summary for the service and the other to get the sync blocker for the service instance.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// <p>The name of the service instance that you want sync your service configuration with.</p>
    pub fn service_instance_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.service_instance_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service instance that you want sync your service configuration with.</p>
    pub fn set_service_instance_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.service_instance_name = input;
        self
    }
    /// Appends an item to `latest_blockers`.
    ///
    /// To override the contents of this collection use [`set_latest_blockers`](Self::set_latest_blockers).
    ///
    /// <p>The latest active blockers for the synced service.</p>
    pub fn latest_blockers(mut self, input: crate::types::SyncBlocker) -> Self {
        let mut v = self.latest_blockers.unwrap_or_default();
        v.push(input);
        self.latest_blockers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The latest active blockers for the synced service.</p>
    pub fn set_latest_blockers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SyncBlocker>>,
    ) -> Self {
        self.latest_blockers = input;
        self
    }
    /// Consumes the builder and constructs a [`ServiceSyncBlockerSummary`](crate::types::ServiceSyncBlockerSummary).
    pub fn build(self) -> crate::types::ServiceSyncBlockerSummary {
        crate::types::ServiceSyncBlockerSummary {
            service_name: self.service_name,
            service_instance_name: self.service_instance_name,
            latest_blockers: self.latest_blockers,
        }
    }
}
