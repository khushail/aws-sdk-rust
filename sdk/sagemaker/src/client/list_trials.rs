// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTrials`](crate::operation::list_trials::builders::ListTrialsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`experiment_name(impl ::std::convert::Into<String>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::experiment_name) / [`set_experiment_name(Option<String>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_experiment_name): <p>A filter that returns only trials that are part of the specified experiment.</p>
    ///   - [`trial_component_name(impl ::std::convert::Into<String>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::trial_component_name) / [`set_trial_component_name(Option<String>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_trial_component_name): <p>A filter that returns only trials that are associated with the specified trial component.</p>
    ///   - [`created_after(DateTime)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::created_after) / [`set_created_after(Option<DateTime>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_created_after): <p>A filter that returns only trials created after the specified time.</p>
    ///   - [`created_before(DateTime)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::created_before) / [`set_created_before(Option<DateTime>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_created_before): <p>A filter that returns only trials created before the specified time.</p>
    ///   - [`sort_by(SortTrialsBy)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::sort_by) / [`set_sort_by(Option<SortTrialsBy>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_sort_by): <p>The property used to sort results. The default value is <code>CreationTime</code>.</p>
    ///   - [`sort_order(SortOrder)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_sort_order): <p>The sort order. The default value is <code>Descending</code>.</p>
    ///   - [`max_results(i32)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_max_results): <p>The maximum number of trials to return in the response. The default value is 10.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_trials::builders::ListTrialsFluentBuilder::set_next_token): <p>If the previous call to <code>ListTrials</code> didn't return the full set of trials, the call returns a token for getting the next set of trials.</p>
    /// - On success, responds with [`ListTrialsOutput`](crate::operation::list_trials::ListTrialsOutput) with field(s):
    ///   - [`trial_summaries(Option<Vec<TrialSummary>>)`](crate::operation::list_trials::ListTrialsOutput::trial_summaries): <p>A list of the summaries of your trials.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_trials::ListTrialsOutput::next_token): <p>A token for getting the next set of trials, if there are any.</p>
    /// - On failure, responds with [`SdkError<ListTrialsError>`](crate::operation::list_trials::ListTrialsError)
    pub fn list_trials(&self) -> crate::operation::list_trials::builders::ListTrialsFluentBuilder {
        crate::operation::list_trials::builders::ListTrialsFluentBuilder::new(self.handle.clone())
    }
}
