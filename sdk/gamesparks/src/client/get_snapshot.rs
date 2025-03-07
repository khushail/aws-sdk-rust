// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSnapshot`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`game_name(impl ::std::convert::Into<String>)`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::game_name) / [`set_game_name(Option<String>)`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::set_game_name): <p>The name of the game.</p>
    ///   - [`snapshot_id(impl ::std::convert::Into<String>)`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::set_snapshot_id): <p>The identifier of the snapshot.</p>
    ///   - [`sections(Vec<String>)`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::sections) / [`set_sections(Option<Vec<String>>)`](crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::set_sections): <p>The list of game configuration sections to be described.</p>
    /// - On success, responds with [`GetSnapshotOutput`](crate::operation::get_snapshot::GetSnapshotOutput) with field(s):
    ///   - [`snapshot(Option<SnapshotDetails>)`](crate::operation::get_snapshot::GetSnapshotOutput::snapshot): <p>Properties that provide details of the snapshot.</p>
    /// - On failure, responds with [`SdkError<GetSnapshotError>`](crate::operation::get_snapshot::GetSnapshotError)
    pub fn get_snapshot(
        &self,
    ) -> crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder {
        crate::operation::get_snapshot::builders::GetSnapshotFluentBuilder::new(self.handle.clone())
    }
}
