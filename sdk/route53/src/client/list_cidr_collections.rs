// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCidrCollections`](crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder::set_next_token): <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>  <p>If no value is provided, the listing of results starts from the beginning.</p>
    ///   - [`max_results(i32)`](crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder::set_max_results): <p>The maximum number of CIDR collections to return in the response.</p>
    /// - On success, responds with [`ListCidrCollectionsOutput`](crate::operation::list_cidr_collections::ListCidrCollectionsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_cidr_collections::ListCidrCollectionsOutput::next_token): <p>An opaque pagination token to indicate where the service is to begin enumerating results.</p>  <p>If no value is provided, the listing of results starts from the beginning.</p>
    ///   - [`cidr_collections(Option<Vec<CollectionSummary>>)`](crate::operation::list_cidr_collections::ListCidrCollectionsOutput::cidr_collections): <p>A complex type with information about the CIDR collection.</p>
    /// - On failure, responds with [`SdkError<ListCidrCollectionsError>`](crate::operation::list_cidr_collections::ListCidrCollectionsError)
    pub fn list_cidr_collections(
        &self,
    ) -> crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder {
        crate::operation::list_cidr_collections::builders::ListCidrCollectionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
