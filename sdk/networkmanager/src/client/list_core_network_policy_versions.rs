// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCoreNetworkPolicyVersions`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`core_network_id(impl ::std::convert::Into<String>)`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::core_network_id) / [`set_core_network_id(Option<String>)`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::set_core_network_id): <p>The ID of a core network.</p>
    ///   - [`max_results(i32)`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::set_max_results): <p>The maximum number of results to return.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::set_next_token): <p>The token for the next page of results.</p>
    /// - On success, responds with [`ListCoreNetworkPolicyVersionsOutput`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput) with field(s):
    ///   - [`core_network_policy_versions(Option<Vec<CoreNetworkPolicyVersion>>)`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput::core_network_policy_versions): <p>Describes core network policy versions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsOutput::next_token): <p>The token for the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListCoreNetworkPolicyVersionsError>`](crate::operation::list_core_network_policy_versions::ListCoreNetworkPolicyVersionsError)
    pub fn list_core_network_policy_versions(&self) -> crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder{
        crate::operation::list_core_network_policy_versions::builders::ListCoreNetworkPolicyVersionsFluentBuilder::new(self.handle.clone())
    }
}
