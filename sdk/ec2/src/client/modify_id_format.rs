// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyIdFormat`](crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource(impl ::std::convert::Into<String>)`](crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder::resource) / [`set_resource(Option<String>)`](crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder::set_resource): <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>  <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    ///   - [`use_long_ids(bool)`](crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder::use_long_ids) / [`set_use_long_ids(Option<bool>)`](crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder::set_use_long_ids): <p>Indicate whether the resource should use longer IDs (17-character IDs).</p>
    /// - On success, responds with [`ModifyIdFormatOutput`](crate::operation::modify_id_format::ModifyIdFormatOutput)
    /// - On failure, responds with [`SdkError<ModifyIdFormatError>`](crate::operation::modify_id_format::ModifyIdFormatError)
    pub fn modify_id_format(
        &self,
    ) -> crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder {
        crate::operation::modify_id_format::builders::ModifyIdFormatFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
