// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSatellites`](crate::operation::list_satellites::builders::ListSatellitesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_satellites::builders::ListSatellitesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_satellites::builders::ListSatellitesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_satellites::builders::ListSatellitesFluentBuilder::set_max_results): <p>Maximum number of satellites returned.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_satellites::builders::ListSatellitesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_satellites::builders::ListSatellitesFluentBuilder::set_next_token): <p>Next token that can be supplied in the next call to get the next page of satellites.</p>
    /// - On success, responds with [`ListSatellitesOutput`](crate::operation::list_satellites::ListSatellitesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_satellites::ListSatellitesOutput::next_token): <p>Next token that can be supplied in the next call to get the next page of satellites.</p>
    ///   - [`satellites(Option<Vec<SatelliteListItem>>)`](crate::operation::list_satellites::ListSatellitesOutput::satellites): <p>List of satellites.</p>
    /// - On failure, responds with [`SdkError<ListSatellitesError>`](crate::operation::list_satellites::ListSatellitesError)
    pub fn list_satellites(
        &self,
    ) -> crate::operation::list_satellites::builders::ListSatellitesFluentBuilder {
        crate::operation::list_satellites::builders::ListSatellitesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
