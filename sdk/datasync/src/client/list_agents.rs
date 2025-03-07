// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAgents`](crate::operation::list_agents::builders::ListAgentsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_agents::builders::ListAgentsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_agents::builders::ListAgentsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_agents::builders::ListAgentsFluentBuilder::set_max_results): <p>Specifies the maximum number of DataSync agents to list in a response. By default, a response shows a maximum of 100 agents.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_agents::builders::ListAgentsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_agents::builders::ListAgentsFluentBuilder::set_next_token): <p>Specifies an opaque string that indicates the position to begin the next list of results in the response.</p>
    /// - On success, responds with [`ListAgentsOutput`](crate::operation::list_agents::ListAgentsOutput) with field(s):
    ///   - [`agents(Option<Vec<AgentListEntry>>)`](crate::operation::list_agents::ListAgentsOutput::agents): <p>A list of DataSync agents in your Amazon Web Services account in the Amazon Web Services Region specified in the request. The list is ordered by the agents' Amazon Resource Names (ARNs).</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_agents::ListAgentsOutput::next_token): <p>The opaque string that indicates the position to begin the next list of results in the response.</p>
    /// - On failure, responds with [`SdkError<ListAgentsError>`](crate::operation::list_agents::ListAgentsError)
    pub fn list_agents(&self) -> crate::operation::list_agents::builders::ListAgentsFluentBuilder {
        crate::operation::list_agents::builders::ListAgentsFluentBuilder::new(self.handle.clone())
    }
}
