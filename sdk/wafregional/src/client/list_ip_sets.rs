// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListIPSets`](crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_marker(impl ::std::convert::Into<String>)`](crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder::next_marker) / [`set_next_marker(Option<String>)`](crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder::set_next_marker): <p>AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>IPSets</code>. For the second and subsequent <code>ListIPSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>IPSets</code>.</p>
    ///   - [`limit(i32)`](crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder::limit) / [`set_limit(i32)`](crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder::set_limit): <p>Specifies the number of <code>IPSet</code> objects that you want AWS WAF to return for this request. If you have more <code>IPSet</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>IPSet</code> objects.</p>
    /// - On success, responds with [`ListIpSetsOutput`](crate::operation::list_ip_sets::ListIpSetsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::operation::list_ip_sets::ListIpSetsOutput::next_marker): <p>To list more <code>IPSet</code> objects, submit another <code>ListIPSets</code> request, and in the next request use the <code>NextMarker</code> response value as the <code>NextMarker</code> value.</p>
    ///   - [`ip_sets(Option<Vec<IpSetSummary>>)`](crate::operation::list_ip_sets::ListIpSetsOutput::ip_sets): <p>An array of <code>IPSetSummary</code> objects.</p>
    /// - On failure, responds with [`SdkError<ListIPSetsError>`](crate::operation::list_ip_sets::ListIPSetsError)
    pub fn list_ip_sets(
        &self,
    ) -> crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder {
        crate::operation::list_ip_sets::builders::ListIPSetsFluentBuilder::new(self.handle.clone())
    }
}
