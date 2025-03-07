// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetInstanceSnapshots`](crate::operation::get_instance_snapshots::builders::GetInstanceSnapshotsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`page_token(impl ::std::convert::Into<String>)`](crate::operation::get_instance_snapshots::builders::GetInstanceSnapshotsFluentBuilder::page_token) / [`set_page_token(Option<String>)`](crate::operation::get_instance_snapshots::builders::GetInstanceSnapshotsFluentBuilder::set_page_token): <p>The token to advance to the next page of results from your request.</p>  <p>To get a page token, perform an initial <code>GetInstanceSnapshots</code> request. If your results are paginated, the response will return a next page token that you can specify as the page token in a subsequent request.</p>
    /// - On success, responds with [`GetInstanceSnapshotsOutput`](crate::operation::get_instance_snapshots::GetInstanceSnapshotsOutput) with field(s):
    ///   - [`instance_snapshots(Option<Vec<InstanceSnapshot>>)`](crate::operation::get_instance_snapshots::GetInstanceSnapshotsOutput::instance_snapshots): <p>An array of key-value pairs containing information about the results of your get instance snapshots request.</p>
    ///   - [`next_page_token(Option<String>)`](crate::operation::get_instance_snapshots::GetInstanceSnapshotsOutput::next_page_token): <p>The token to advance to the next page of results from your request.</p>  <p>A next page token is not returned if there are no more results to display.</p>  <p>To get the next page of results, perform another <code>GetInstanceSnapshots</code> request and specify the next page token using the <code>pageToken</code> parameter.</p>
    /// - On failure, responds with [`SdkError<GetInstanceSnapshotsError>`](crate::operation::get_instance_snapshots::GetInstanceSnapshotsError)
    pub fn get_instance_snapshots(
        &self,
    ) -> crate::operation::get_instance_snapshots::builders::GetInstanceSnapshotsFluentBuilder {
        crate::operation::get_instance_snapshots::builders::GetInstanceSnapshotsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
