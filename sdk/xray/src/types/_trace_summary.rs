// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Metadata generated from the segment documents in a trace.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TraceSummary {
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    #[doc(hidden)]
    pub duration: ::std::option::Option<f64>,
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    #[doc(hidden)]
    pub response_time: ::std::option::Option<f64>,
    /// <p>The root segment document has a 500 series error.</p>
    #[doc(hidden)]
    pub has_fault: ::std::option::Option<bool>,
    /// <p>The root segment document has a 400 series error.</p>
    #[doc(hidden)]
    pub has_error: ::std::option::Option<bool>,
    /// <p>One or more of the segment documents has a 429 throttling error.</p>
    #[doc(hidden)]
    pub has_throttle: ::std::option::Option<bool>,
    /// <p>One or more of the segment documents is in progress.</p>
    #[doc(hidden)]
    pub is_partial: ::std::option::Option<bool>,
    /// <p>Information about the HTTP request served by the trace.</p>
    #[doc(hidden)]
    pub http: ::std::option::Option<crate::types::Http>,
    /// <p>Annotations from the trace's segment documents.</p>
    #[doc(hidden)]
    pub annotations: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<crate::types::ValueWithServiceIds>,
        >,
    >,
    /// <p>Users from the trace's segment documents.</p>
    #[doc(hidden)]
    pub users: ::std::option::Option<::std::vec::Vec<crate::types::TraceUser>>,
    /// <p>Service IDs from the trace's segment documents.</p>
    #[doc(hidden)]
    pub service_ids: ::std::option::Option<::std::vec::Vec<crate::types::ServiceId>>,
    /// <p>A list of resource ARNs for any resource corresponding to the trace segments.</p>
    #[doc(hidden)]
    pub resource_ar_ns: ::std::option::Option<::std::vec::Vec<crate::types::ResourceArnDetail>>,
    /// <p>A list of EC2 instance IDs for any instance corresponding to the trace segments.</p>
    #[doc(hidden)]
    pub instance_ids: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIdDetail>>,
    /// <p>A list of Availability Zones for any zone corresponding to the trace segments.</p>
    #[doc(hidden)]
    pub availability_zones:
        ::std::option::Option<::std::vec::Vec<crate::types::AvailabilityZoneDetail>>,
    /// <p>The root of a trace.</p>
    #[doc(hidden)]
    pub entry_point: ::std::option::Option<crate::types::ServiceId>,
    /// <p>A collection of FaultRootCause structures corresponding to the trace segments.</p>
    #[doc(hidden)]
    pub fault_root_causes: ::std::option::Option<::std::vec::Vec<crate::types::FaultRootCause>>,
    /// <p>A collection of ErrorRootCause structures corresponding to the trace segments.</p>
    #[doc(hidden)]
    pub error_root_causes: ::std::option::Option<::std::vec::Vec<crate::types::ErrorRootCause>>,
    /// <p>A collection of ResponseTimeRootCause structures corresponding to the trace segments.</p>
    #[doc(hidden)]
    pub response_time_root_causes:
        ::std::option::Option<::std::vec::Vec<crate::types::ResponseTimeRootCause>>,
    /// <p>The revision number of a trace.</p>
    #[doc(hidden)]
    pub revision: i32,
    /// <p>The matched time stamp of a defined event.</p>
    #[doc(hidden)]
    pub matched_event_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl TraceSummary {
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    pub fn duration(&self) -> ::std::option::Option<f64> {
        self.duration
    }
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    pub fn response_time(&self) -> ::std::option::Option<f64> {
        self.response_time
    }
    /// <p>The root segment document has a 500 series error.</p>
    pub fn has_fault(&self) -> ::std::option::Option<bool> {
        self.has_fault
    }
    /// <p>The root segment document has a 400 series error.</p>
    pub fn has_error(&self) -> ::std::option::Option<bool> {
        self.has_error
    }
    /// <p>One or more of the segment documents has a 429 throttling error.</p>
    pub fn has_throttle(&self) -> ::std::option::Option<bool> {
        self.has_throttle
    }
    /// <p>One or more of the segment documents is in progress.</p>
    pub fn is_partial(&self) -> ::std::option::Option<bool> {
        self.is_partial
    }
    /// <p>Information about the HTTP request served by the trace.</p>
    pub fn http(&self) -> ::std::option::Option<&crate::types::Http> {
        self.http.as_ref()
    }
    /// <p>Annotations from the trace's segment documents.</p>
    pub fn annotations(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<crate::types::ValueWithServiceIds>,
        >,
    > {
        self.annotations.as_ref()
    }
    /// <p>Users from the trace's segment documents.</p>
    pub fn users(&self) -> ::std::option::Option<&[crate::types::TraceUser]> {
        self.users.as_deref()
    }
    /// <p>Service IDs from the trace's segment documents.</p>
    pub fn service_ids(&self) -> ::std::option::Option<&[crate::types::ServiceId]> {
        self.service_ids.as_deref()
    }
    /// <p>A list of resource ARNs for any resource corresponding to the trace segments.</p>
    pub fn resource_ar_ns(&self) -> ::std::option::Option<&[crate::types::ResourceArnDetail]> {
        self.resource_ar_ns.as_deref()
    }
    /// <p>A list of EC2 instance IDs for any instance corresponding to the trace segments.</p>
    pub fn instance_ids(&self) -> ::std::option::Option<&[crate::types::InstanceIdDetail]> {
        self.instance_ids.as_deref()
    }
    /// <p>A list of Availability Zones for any zone corresponding to the trace segments.</p>
    pub fn availability_zones(
        &self,
    ) -> ::std::option::Option<&[crate::types::AvailabilityZoneDetail]> {
        self.availability_zones.as_deref()
    }
    /// <p>The root of a trace.</p>
    pub fn entry_point(&self) -> ::std::option::Option<&crate::types::ServiceId> {
        self.entry_point.as_ref()
    }
    /// <p>A collection of FaultRootCause structures corresponding to the trace segments.</p>
    pub fn fault_root_causes(&self) -> ::std::option::Option<&[crate::types::FaultRootCause]> {
        self.fault_root_causes.as_deref()
    }
    /// <p>A collection of ErrorRootCause structures corresponding to the trace segments.</p>
    pub fn error_root_causes(&self) -> ::std::option::Option<&[crate::types::ErrorRootCause]> {
        self.error_root_causes.as_deref()
    }
    /// <p>A collection of ResponseTimeRootCause structures corresponding to the trace segments.</p>
    pub fn response_time_root_causes(
        &self,
    ) -> ::std::option::Option<&[crate::types::ResponseTimeRootCause]> {
        self.response_time_root_causes.as_deref()
    }
    /// <p>The revision number of a trace.</p>
    pub fn revision(&self) -> i32 {
        self.revision
    }
    /// <p>The matched time stamp of a defined event.</p>
    pub fn matched_event_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.matched_event_time.as_ref()
    }
}
impl TraceSummary {
    /// Creates a new builder-style object to manufacture [`TraceSummary`](crate::types::TraceSummary).
    pub fn builder() -> crate::types::builders::TraceSummaryBuilder {
        crate::types::builders::TraceSummaryBuilder::default()
    }
}

/// A builder for [`TraceSummary`](crate::types::TraceSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TraceSummaryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) duration: ::std::option::Option<f64>,
    pub(crate) response_time: ::std::option::Option<f64>,
    pub(crate) has_fault: ::std::option::Option<bool>,
    pub(crate) has_error: ::std::option::Option<bool>,
    pub(crate) has_throttle: ::std::option::Option<bool>,
    pub(crate) is_partial: ::std::option::Option<bool>,
    pub(crate) http: ::std::option::Option<crate::types::Http>,
    pub(crate) annotations: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::vec::Vec<crate::types::ValueWithServiceIds>,
        >,
    >,
    pub(crate) users: ::std::option::Option<::std::vec::Vec<crate::types::TraceUser>>,
    pub(crate) service_ids: ::std::option::Option<::std::vec::Vec<crate::types::ServiceId>>,
    pub(crate) resource_ar_ns:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourceArnDetail>>,
    pub(crate) instance_ids: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIdDetail>>,
    pub(crate) availability_zones:
        ::std::option::Option<::std::vec::Vec<crate::types::AvailabilityZoneDetail>>,
    pub(crate) entry_point: ::std::option::Option<crate::types::ServiceId>,
    pub(crate) fault_root_causes:
        ::std::option::Option<::std::vec::Vec<crate::types::FaultRootCause>>,
    pub(crate) error_root_causes:
        ::std::option::Option<::std::vec::Vec<crate::types::ErrorRootCause>>,
    pub(crate) response_time_root_causes:
        ::std::option::Option<::std::vec::Vec<crate::types::ResponseTimeRootCause>>,
    pub(crate) revision: ::std::option::Option<i32>,
    pub(crate) matched_event_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl TraceSummaryBuilder {
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the request that generated the trace's segments and subsegments.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    pub fn duration(mut self, input: f64) -> Self {
        self.duration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The length of time in seconds between the start time of the root segment and the end time of the last segment that completed.</p>
    pub fn set_duration(mut self, input: ::std::option::Option<f64>) -> Self {
        self.duration = input;
        self
    }
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    pub fn response_time(mut self, input: f64) -> Self {
        self.response_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The length of time in seconds between the start and end times of the root segment. If the service performs work asynchronously, the response time measures the time before the response is sent to the user, while the duration measures the amount of time before the last traced activity completes.</p>
    pub fn set_response_time(mut self, input: ::std::option::Option<f64>) -> Self {
        self.response_time = input;
        self
    }
    /// <p>The root segment document has a 500 series error.</p>
    pub fn has_fault(mut self, input: bool) -> Self {
        self.has_fault = ::std::option::Option::Some(input);
        self
    }
    /// <p>The root segment document has a 500 series error.</p>
    pub fn set_has_fault(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_fault = input;
        self
    }
    /// <p>The root segment document has a 400 series error.</p>
    pub fn has_error(mut self, input: bool) -> Self {
        self.has_error = ::std::option::Option::Some(input);
        self
    }
    /// <p>The root segment document has a 400 series error.</p>
    pub fn set_has_error(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_error = input;
        self
    }
    /// <p>One or more of the segment documents has a 429 throttling error.</p>
    pub fn has_throttle(mut self, input: bool) -> Self {
        self.has_throttle = ::std::option::Option::Some(input);
        self
    }
    /// <p>One or more of the segment documents has a 429 throttling error.</p>
    pub fn set_has_throttle(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_throttle = input;
        self
    }
    /// <p>One or more of the segment documents is in progress.</p>
    pub fn is_partial(mut self, input: bool) -> Self {
        self.is_partial = ::std::option::Option::Some(input);
        self
    }
    /// <p>One or more of the segment documents is in progress.</p>
    pub fn set_is_partial(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_partial = input;
        self
    }
    /// <p>Information about the HTTP request served by the trace.</p>
    pub fn http(mut self, input: crate::types::Http) -> Self {
        self.http = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the HTTP request served by the trace.</p>
    pub fn set_http(mut self, input: ::std::option::Option<crate::types::Http>) -> Self {
        self.http = input;
        self
    }
    /// Adds a key-value pair to `annotations`.
    ///
    /// To override the contents of this collection use [`set_annotations`](Self::set_annotations).
    ///
    /// <p>Annotations from the trace's segment documents.</p>
    pub fn annotations(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: ::std::vec::Vec<crate::types::ValueWithServiceIds>,
    ) -> Self {
        let mut hash_map = self.annotations.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.annotations = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Annotations from the trace's segment documents.</p>
    pub fn set_annotations(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<crate::types::ValueWithServiceIds>,
            >,
        >,
    ) -> Self {
        self.annotations = input;
        self
    }
    /// Appends an item to `users`.
    ///
    /// To override the contents of this collection use [`set_users`](Self::set_users).
    ///
    /// <p>Users from the trace's segment documents.</p>
    pub fn users(mut self, input: crate::types::TraceUser) -> Self {
        let mut v = self.users.unwrap_or_default();
        v.push(input);
        self.users = ::std::option::Option::Some(v);
        self
    }
    /// <p>Users from the trace's segment documents.</p>
    pub fn set_users(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TraceUser>>,
    ) -> Self {
        self.users = input;
        self
    }
    /// Appends an item to `service_ids`.
    ///
    /// To override the contents of this collection use [`set_service_ids`](Self::set_service_ids).
    ///
    /// <p>Service IDs from the trace's segment documents.</p>
    pub fn service_ids(mut self, input: crate::types::ServiceId) -> Self {
        let mut v = self.service_ids.unwrap_or_default();
        v.push(input);
        self.service_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>Service IDs from the trace's segment documents.</p>
    pub fn set_service_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ServiceId>>,
    ) -> Self {
        self.service_ids = input;
        self
    }
    /// Appends an item to `resource_ar_ns`.
    ///
    /// To override the contents of this collection use [`set_resource_ar_ns`](Self::set_resource_ar_ns).
    ///
    /// <p>A list of resource ARNs for any resource corresponding to the trace segments.</p>
    pub fn resource_ar_ns(mut self, input: crate::types::ResourceArnDetail) -> Self {
        let mut v = self.resource_ar_ns.unwrap_or_default();
        v.push(input);
        self.resource_ar_ns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of resource ARNs for any resource corresponding to the trace segments.</p>
    pub fn set_resource_ar_ns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceArnDetail>>,
    ) -> Self {
        self.resource_ar_ns = input;
        self
    }
    /// Appends an item to `instance_ids`.
    ///
    /// To override the contents of this collection use [`set_instance_ids`](Self::set_instance_ids).
    ///
    /// <p>A list of EC2 instance IDs for any instance corresponding to the trace segments.</p>
    pub fn instance_ids(mut self, input: crate::types::InstanceIdDetail) -> Self {
        let mut v = self.instance_ids.unwrap_or_default();
        v.push(input);
        self.instance_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of EC2 instance IDs for any instance corresponding to the trace segments.</p>
    pub fn set_instance_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIdDetail>>,
    ) -> Self {
        self.instance_ids = input;
        self
    }
    /// Appends an item to `availability_zones`.
    ///
    /// To override the contents of this collection use [`set_availability_zones`](Self::set_availability_zones).
    ///
    /// <p>A list of Availability Zones for any zone corresponding to the trace segments.</p>
    pub fn availability_zones(mut self, input: crate::types::AvailabilityZoneDetail) -> Self {
        let mut v = self.availability_zones.unwrap_or_default();
        v.push(input);
        self.availability_zones = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of Availability Zones for any zone corresponding to the trace segments.</p>
    pub fn set_availability_zones(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AvailabilityZoneDetail>>,
    ) -> Self {
        self.availability_zones = input;
        self
    }
    /// <p>The root of a trace.</p>
    pub fn entry_point(mut self, input: crate::types::ServiceId) -> Self {
        self.entry_point = ::std::option::Option::Some(input);
        self
    }
    /// <p>The root of a trace.</p>
    pub fn set_entry_point(
        mut self,
        input: ::std::option::Option<crate::types::ServiceId>,
    ) -> Self {
        self.entry_point = input;
        self
    }
    /// Appends an item to `fault_root_causes`.
    ///
    /// To override the contents of this collection use [`set_fault_root_causes`](Self::set_fault_root_causes).
    ///
    /// <p>A collection of FaultRootCause structures corresponding to the trace segments.</p>
    pub fn fault_root_causes(mut self, input: crate::types::FaultRootCause) -> Self {
        let mut v = self.fault_root_causes.unwrap_or_default();
        v.push(input);
        self.fault_root_causes = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of FaultRootCause structures corresponding to the trace segments.</p>
    pub fn set_fault_root_causes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FaultRootCause>>,
    ) -> Self {
        self.fault_root_causes = input;
        self
    }
    /// Appends an item to `error_root_causes`.
    ///
    /// To override the contents of this collection use [`set_error_root_causes`](Self::set_error_root_causes).
    ///
    /// <p>A collection of ErrorRootCause structures corresponding to the trace segments.</p>
    pub fn error_root_causes(mut self, input: crate::types::ErrorRootCause) -> Self {
        let mut v = self.error_root_causes.unwrap_or_default();
        v.push(input);
        self.error_root_causes = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of ErrorRootCause structures corresponding to the trace segments.</p>
    pub fn set_error_root_causes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ErrorRootCause>>,
    ) -> Self {
        self.error_root_causes = input;
        self
    }
    /// Appends an item to `response_time_root_causes`.
    ///
    /// To override the contents of this collection use [`set_response_time_root_causes`](Self::set_response_time_root_causes).
    ///
    /// <p>A collection of ResponseTimeRootCause structures corresponding to the trace segments.</p>
    pub fn response_time_root_causes(mut self, input: crate::types::ResponseTimeRootCause) -> Self {
        let mut v = self.response_time_root_causes.unwrap_or_default();
        v.push(input);
        self.response_time_root_causes = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of ResponseTimeRootCause structures corresponding to the trace segments.</p>
    pub fn set_response_time_root_causes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResponseTimeRootCause>>,
    ) -> Self {
        self.response_time_root_causes = input;
        self
    }
    /// <p>The revision number of a trace.</p>
    pub fn revision(mut self, input: i32) -> Self {
        self.revision = ::std::option::Option::Some(input);
        self
    }
    /// <p>The revision number of a trace.</p>
    pub fn set_revision(mut self, input: ::std::option::Option<i32>) -> Self {
        self.revision = input;
        self
    }
    /// <p>The matched time stamp of a defined event.</p>
    pub fn matched_event_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.matched_event_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The matched time stamp of a defined event.</p>
    pub fn set_matched_event_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.matched_event_time = input;
        self
    }
    /// Consumes the builder and constructs a [`TraceSummary`](crate::types::TraceSummary).
    pub fn build(self) -> crate::types::TraceSummary {
        crate::types::TraceSummary {
            id: self.id,
            duration: self.duration,
            response_time: self.response_time,
            has_fault: self.has_fault,
            has_error: self.has_error,
            has_throttle: self.has_throttle,
            is_partial: self.is_partial,
            http: self.http,
            annotations: self.annotations,
            users: self.users,
            service_ids: self.service_ids,
            resource_ar_ns: self.resource_ar_ns,
            instance_ids: self.instance_ids,
            availability_zones: self.availability_zones,
            entry_point: self.entry_point,
            fault_root_causes: self.fault_root_causes,
            error_root_causes: self.error_root_causes,
            response_time_root_causes: self.response_time_root_causes,
            revision: self.revision.unwrap_or_default(),
            matched_event_time: self.matched_event_time,
        }
    }
}
