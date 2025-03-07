// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreSnapshotFromRecycleBinOutput {
    /// <p>The ID of the snapshot.</p>
    #[doc(hidden)]
    pub snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the Outpost on which the snapshot is stored. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    #[doc(hidden)]
    pub outpost_arn: ::std::option::Option<::std::string::String>,
    /// <p>The description for the snapshot.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the snapshot is encrypted.</p>
    #[doc(hidden)]
    pub encrypted: ::std::option::Option<bool>,
    /// <p>The ID of the Amazon Web Services account that owns the EBS snapshot.</p>
    #[doc(hidden)]
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The progress of the snapshot, as a percentage.</p>
    #[doc(hidden)]
    pub progress: ::std::option::Option<::std::string::String>,
    /// <p>The time stamp when the snapshot was initiated.</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The state of the snapshot.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::SnapshotState>,
    /// <p>The ID of the volume that was used to create the snapshot.</p>
    #[doc(hidden)]
    pub volume_id: ::std::option::Option<::std::string::String>,
    /// <p>The size of the volume, in GiB.</p>
    #[doc(hidden)]
    pub volume_size: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl RestoreSnapshotFromRecycleBinOutput {
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>The ARN of the Outpost on which the snapshot is stored. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn outpost_arn(&self) -> ::std::option::Option<&str> {
        self.outpost_arn.as_deref()
    }
    /// <p>The description for the snapshot.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Indicates whether the snapshot is encrypted.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
    }
    /// <p>The ID of the Amazon Web Services account that owns the EBS snapshot.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The progress of the snapshot, as a percentage.</p>
    pub fn progress(&self) -> ::std::option::Option<&str> {
        self.progress.as_deref()
    }
    /// <p>The time stamp when the snapshot was initiated.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The state of the snapshot.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::SnapshotState> {
        self.state.as_ref()
    }
    /// <p>The ID of the volume that was used to create the snapshot.</p>
    pub fn volume_id(&self) -> ::std::option::Option<&str> {
        self.volume_id.as_deref()
    }
    /// <p>The size of the volume, in GiB.</p>
    pub fn volume_size(&self) -> ::std::option::Option<i32> {
        self.volume_size
    }
}
impl ::aws_http::request_id::RequestId for RestoreSnapshotFromRecycleBinOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RestoreSnapshotFromRecycleBinOutput {
    /// Creates a new builder-style object to manufacture [`RestoreSnapshotFromRecycleBinOutput`](crate::operation::restore_snapshot_from_recycle_bin::RestoreSnapshotFromRecycleBinOutput).
    pub fn builder() -> crate::operation::restore_snapshot_from_recycle_bin::builders::RestoreSnapshotFromRecycleBinOutputBuilder{
        crate::operation::restore_snapshot_from_recycle_bin::builders::RestoreSnapshotFromRecycleBinOutputBuilder::default()
    }
}

/// A builder for [`RestoreSnapshotFromRecycleBinOutput`](crate::operation::restore_snapshot_from_recycle_bin::RestoreSnapshotFromRecycleBinOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RestoreSnapshotFromRecycleBinOutputBuilder {
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) outpost_arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) progress: ::std::option::Option<::std::string::String>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) state: ::std::option::Option<crate::types::SnapshotState>,
    pub(crate) volume_id: ::std::option::Option<::std::string::String>,
    pub(crate) volume_size: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl RestoreSnapshotFromRecycleBinOutputBuilder {
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
    /// <p>The ARN of the Outpost on which the snapshot is stored. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.outpost_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the Outpost on which the snapshot is stored. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.outpost_arn = input;
        self
    }
    /// <p>The description for the snapshot.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description for the snapshot.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Indicates whether the snapshot is encrypted.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the snapshot is encrypted.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the EBS snapshot.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the EBS snapshot.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The progress of the snapshot, as a percentage.</p>
    pub fn progress(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.progress = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The progress of the snapshot, as a percentage.</p>
    pub fn set_progress(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.progress = input;
        self
    }
    /// <p>The time stamp when the snapshot was initiated.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp when the snapshot was initiated.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The state of the snapshot.</p>
    pub fn state(mut self, input: crate::types::SnapshotState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the snapshot.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::SnapshotState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The ID of the volume that was used to create the snapshot.</p>
    pub fn volume_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.volume_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the volume that was used to create the snapshot.</p>
    pub fn set_volume_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.volume_id = input;
        self
    }
    /// <p>The size of the volume, in GiB.</p>
    pub fn volume_size(mut self, input: i32) -> Self {
        self.volume_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The size of the volume, in GiB.</p>
    pub fn set_volume_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.volume_size = input;
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
    /// Consumes the builder and constructs a [`RestoreSnapshotFromRecycleBinOutput`](crate::operation::restore_snapshot_from_recycle_bin::RestoreSnapshotFromRecycleBinOutput).
    pub fn build(
        self,
    ) -> crate::operation::restore_snapshot_from_recycle_bin::RestoreSnapshotFromRecycleBinOutput
    {
        crate::operation::restore_snapshot_from_recycle_bin::RestoreSnapshotFromRecycleBinOutput {
            snapshot_id: self.snapshot_id,
            outpost_arn: self.outpost_arn,
            description: self.description,
            encrypted: self.encrypted,
            owner_id: self.owner_id,
            progress: self.progress,
            start_time: self.start_time,
            state: self.state,
            volume_id: self.volume_id,
            volume_size: self.volume_size,
            _request_id: self._request_id,
        }
    }
}
