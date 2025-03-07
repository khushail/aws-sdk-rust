// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCostCategoryDefinitions`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`effective_on(impl ::std::convert::Into<String>)`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::effective_on) / [`set_effective_on(Option<String>)`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::set_effective_on): <p>The date when the Cost Category was effective. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::set_next_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
    ///   - [`max_results(i32)`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::set_max_results): <p>The number of entries a paginated response contains. </p>
    /// - On success, responds with [`ListCostCategoryDefinitionsOutput`](crate::operation::list_cost_category_definitions::ListCostCategoryDefinitionsOutput) with field(s):
    ///   - [`cost_category_references(Option<Vec<CostCategoryReference>>)`](crate::operation::list_cost_category_definitions::ListCostCategoryDefinitionsOutput::cost_category_references): <p>A reference to a Cost Category that contains enough information to identify the Cost Category. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_cost_category_definitions::ListCostCategoryDefinitionsOutput::next_token): <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size. </p>
    /// - On failure, responds with [`SdkError<ListCostCategoryDefinitionsError>`](crate::operation::list_cost_category_definitions::ListCostCategoryDefinitionsError)
    pub fn list_cost_category_definitions(&self) -> crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder{
        crate::operation::list_cost_category_definitions::builders::ListCostCategoryDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
