// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCustomKeyStore`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl ::std::convert::Into<String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::set_custom_key_store_id): <p>Enter the ID of the custom key store you want to delete. To find the ID of a custom key store, use the <code>DescribeCustomKeyStores</code> operation.</p>
    /// - On success, responds with [`DeleteCustomKeyStoreOutput`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreOutput)
    /// - On failure, responds with [`SdkError<DeleteCustomKeyStoreError>`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError)
    pub fn delete_custom_key_store(
        &self,
    ) -> crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder
    {
        crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
