// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSafetyRules`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`control_panel_arn(impl ::std::convert::Into<String>)`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::control_panel_arn) / [`set_control_panel_arn(Option<String>)`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::set_control_panel_arn): <p>The Amazon Resource Name (ARN) of the control panel.</p>
    ///   - [`max_results(i32)`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::set_max_results): <p>The number of objects that you want to return with this call.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::set_next_token): <p>The token that identifies which batch of results you want to see.</p>
    /// - On success, responds with [`ListSafetyRulesOutput`](crate::operation::list_safety_rules::ListSafetyRulesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_safety_rules::ListSafetyRulesOutput::next_token): <p>The token that identifies which batch of results you want to see.</p>
    ///   - [`safety_rules(Option<Vec<Rule>>)`](crate::operation::list_safety_rules::ListSafetyRulesOutput::safety_rules): <p>The list of safety rules in a control panel.</p>
    /// - On failure, responds with [`SdkError<ListSafetyRulesError>`](crate::operation::list_safety_rules::ListSafetyRulesError)
    pub fn list_safety_rules(
        &self,
    ) -> crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder {
        crate::operation::list_safety_rules::builders::ListSafetyRulesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
