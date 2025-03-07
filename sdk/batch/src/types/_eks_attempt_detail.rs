// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the details of a job attempt for a job attempt by an Amazon EKS container.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EksAttemptDetail {
    /// <p>The details for the final status of the containers for this job attempt.</p>
    #[doc(hidden)]
    pub containers: ::std::option::Option<::std::vec::Vec<crate::types::EksAttemptContainerDetail>>,
    /// <p>The name of the pod for this job attempt.</p>
    #[doc(hidden)]
    pub pod_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the node for this job attempt.</p>
    #[doc(hidden)]
    pub node_name: ::std::option::Option<::std::string::String>,
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    #[doc(hidden)]
    pub started_at: ::std::option::Option<i64>,
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was stopped. This happens when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>.</p>
    #[doc(hidden)]
    pub stopped_at: ::std::option::Option<i64>,
    /// <p>A short, human-readable string to provide additional details for the current status of the job attempt.</p>
    #[doc(hidden)]
    pub status_reason: ::std::option::Option<::std::string::String>,
}
impl EksAttemptDetail {
    /// <p>The details for the final status of the containers for this job attempt.</p>
    pub fn containers(&self) -> ::std::option::Option<&[crate::types::EksAttemptContainerDetail]> {
        self.containers.as_deref()
    }
    /// <p>The name of the pod for this job attempt.</p>
    pub fn pod_name(&self) -> ::std::option::Option<&str> {
        self.pod_name.as_deref()
    }
    /// <p>The name of the node for this job attempt.</p>
    pub fn node_name(&self) -> ::std::option::Option<&str> {
        self.node_name.as_deref()
    }
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    pub fn started_at(&self) -> ::std::option::Option<i64> {
        self.started_at
    }
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was stopped. This happens when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>.</p>
    pub fn stopped_at(&self) -> ::std::option::Option<i64> {
        self.stopped_at
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job attempt.</p>
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
}
impl EksAttemptDetail {
    /// Creates a new builder-style object to manufacture [`EksAttemptDetail`](crate::types::EksAttemptDetail).
    pub fn builder() -> crate::types::builders::EksAttemptDetailBuilder {
        crate::types::builders::EksAttemptDetailBuilder::default()
    }
}

/// A builder for [`EksAttemptDetail`](crate::types::EksAttemptDetail).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EksAttemptDetailBuilder {
    pub(crate) containers:
        ::std::option::Option<::std::vec::Vec<crate::types::EksAttemptContainerDetail>>,
    pub(crate) pod_name: ::std::option::Option<::std::string::String>,
    pub(crate) node_name: ::std::option::Option<::std::string::String>,
    pub(crate) started_at: ::std::option::Option<i64>,
    pub(crate) stopped_at: ::std::option::Option<i64>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
}
impl EksAttemptDetailBuilder {
    /// Appends an item to `containers`.
    ///
    /// To override the contents of this collection use [`set_containers`](Self::set_containers).
    ///
    /// <p>The details for the final status of the containers for this job attempt.</p>
    pub fn containers(mut self, input: crate::types::EksAttemptContainerDetail) -> Self {
        let mut v = self.containers.unwrap_or_default();
        v.push(input);
        self.containers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The details for the final status of the containers for this job attempt.</p>
    pub fn set_containers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EksAttemptContainerDetail>>,
    ) -> Self {
        self.containers = input;
        self
    }
    /// <p>The name of the pod for this job attempt.</p>
    pub fn pod_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pod_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pod for this job attempt.</p>
    pub fn set_pod_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pod_name = input;
        self
    }
    /// <p>The name of the node for this job attempt.</p>
    pub fn node_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the node for this job attempt.</p>
    pub fn set_node_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_name = input;
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    pub fn started_at(mut self, input: i64) -> Self {
        self.started_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was started (when the attempt transitioned from the <code>STARTING</code> state to the <code>RUNNING</code> state).</p>
    pub fn set_started_at(mut self, input: ::std::option::Option<i64>) -> Self {
        self.started_at = input;
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was stopped. This happens when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>.</p>
    pub fn stopped_at(mut self, input: i64) -> Self {
        self.stopped_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the attempt was stopped. This happens when the attempt transitioned from the <code>RUNNING</code> state to a terminal state, such as <code>SUCCEEDED</code> or <code>FAILED</code>.</p>
    pub fn set_stopped_at(mut self, input: ::std::option::Option<i64>) -> Self {
        self.stopped_at = input;
        self
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job attempt.</p>
    pub fn status_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job attempt.</p>
    pub fn set_status_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_reason = input;
        self
    }
    /// Consumes the builder and constructs a [`EksAttemptDetail`](crate::types::EksAttemptDetail).
    pub fn build(self) -> crate::types::EksAttemptDetail {
        crate::types::EksAttemptDetail {
            containers: self.containers,
            pod_name: self.pod_name,
            node_name: self.node_name,
            started_at: self.started_at,
            stopped_at: self.stopped_at,
            status_reason: self.status_reason,
        }
    }
}
