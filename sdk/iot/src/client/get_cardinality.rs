// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCardinality`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`index_name(impl ::std::convert::Into<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::index_name) / [`set_index_name(Option<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::set_index_name): <p>The name of the index to search.</p>
    ///   - [`query_string(impl ::std::convert::Into<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::query_string) / [`set_query_string(Option<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::set_query_string): <p>The search query string.</p>
    ///   - [`aggregation_field(impl ::std::convert::Into<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::aggregation_field) / [`set_aggregation_field(Option<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::set_aggregation_field): <p>The field to aggregate.</p>
    ///   - [`query_version(impl ::std::convert::Into<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::query_version) / [`set_query_version(Option<String>)`](crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::set_query_version): <p>The query version.</p>
    /// - On success, responds with [`GetCardinalityOutput`](crate::operation::get_cardinality::GetCardinalityOutput) with field(s):
    ///   - [`cardinality(i32)`](crate::operation::get_cardinality::GetCardinalityOutput::cardinality): <p>The approximate count of unique values that match the query.</p>
    /// - On failure, responds with [`SdkError<GetCardinalityError>`](crate::operation::get_cardinality::GetCardinalityError)
    pub fn get_cardinality(
        &self,
    ) -> crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder {
        crate::operation::get_cardinality::builders::GetCardinalityFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
