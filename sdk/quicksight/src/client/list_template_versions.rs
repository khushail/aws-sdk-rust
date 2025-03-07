// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTemplateVersions`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the templates that you're listing.</p>
    ///   - [`template_id(impl ::std::convert::Into<String>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::template_id) / [`set_template_id(Option<String>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::set_template_id): <p>The ID for the template.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::set_max_results): <p>The maximum number of results to be returned per request.</p>
    /// - On success, responds with [`ListTemplateVersionsOutput`](crate::operation::list_template_versions::ListTemplateVersionsOutput) with field(s):
    ///   - [`template_version_summary_list(Option<Vec<TemplateVersionSummary>>)`](crate::operation::list_template_versions::ListTemplateVersionsOutput::template_version_summary_list): <p>A structure containing a list of all the versions of the specified template.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_template_versions::ListTemplateVersionsOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`status(i32)`](crate::operation::list_template_versions::ListTemplateVersionsOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::list_template_versions::ListTemplateVersionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<ListTemplateVersionsError>`](crate::operation::list_template_versions::ListTemplateVersionsError)
    pub fn list_template_versions(
        &self,
    ) -> crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder {
        crate::operation::list_template_versions::builders::ListTemplateVersionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
