// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ApplicationSnapshotConfigurationDescription {
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    #[doc(hidden)]
    pub snapshots_enabled: ::std::option::Option<bool>,
}
impl ApplicationSnapshotConfigurationDescription {
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    pub fn snapshots_enabled(&self) -> ::std::option::Option<bool> {
        self.snapshots_enabled
    }
}
impl ApplicationSnapshotConfigurationDescription {
    /// Creates a new builder-style object to manufacture [`ApplicationSnapshotConfigurationDescription`](crate::types::ApplicationSnapshotConfigurationDescription).
    pub fn builder() -> crate::types::builders::ApplicationSnapshotConfigurationDescriptionBuilder {
        crate::types::builders::ApplicationSnapshotConfigurationDescriptionBuilder::default()
    }
}

/// A builder for [`ApplicationSnapshotConfigurationDescription`](crate::types::ApplicationSnapshotConfigurationDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ApplicationSnapshotConfigurationDescriptionBuilder {
    pub(crate) snapshots_enabled: ::std::option::Option<bool>,
}
impl ApplicationSnapshotConfigurationDescriptionBuilder {
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    pub fn snapshots_enabled(mut self, input: bool) -> Self {
        self.snapshots_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes whether snapshots are enabled for a Flink-based Kinesis Data Analytics application.</p>
    pub fn set_snapshots_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.snapshots_enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`ApplicationSnapshotConfigurationDescription`](crate::types::ApplicationSnapshotConfigurationDescription).
    pub fn build(self) -> crate::types::ApplicationSnapshotConfigurationDescription {
        crate::types::ApplicationSnapshotConfigurationDescription {
            snapshots_enabled: self.snapshots_enabled,
        }
    }
}
