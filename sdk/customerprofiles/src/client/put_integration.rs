// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutIntegration`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::set_domain_name): <p>The unique name of the domain.</p>
    ///   - [`uri(impl ::std::convert::Into<String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::uri) / [`set_uri(Option<String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::set_uri): <p>The URI of the S3 bucket or any other type of data source.</p>
    ///   - [`object_type_name(impl ::std::convert::Into<String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::object_type_name) / [`set_object_type_name(Option<String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::set_object_type_name): <p>The name of the profile object type.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::set_tags): <p>The tags used to organize, track, or control access for this resource.</p>
    ///   - [`flow_definition(FlowDefinition)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::flow_definition) / [`set_flow_definition(Option<FlowDefinition>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::set_flow_definition): <p>The configuration that controls how Customer Profiles retrieves data from the source.</p>
    ///   - [`object_type_names(HashMap<String, String>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::object_type_names) / [`set_object_type_names(Option<HashMap<String, String>>)`](crate::operation::put_integration::builders::PutIntegrationFluentBuilder::set_object_type_names): <p>A map in which each key is an event type from an external application such as Segment or Shopify, and each value is an <code>ObjectTypeName</code> (template) used to ingest the event. It supports the following event types: <code>SegmentIdentify</code>, <code>ShopifyCreateCustomers</code>, <code>ShopifyUpdateCustomers</code>, <code>ShopifyCreateDraftOrders</code>, <code>ShopifyUpdateDraftOrders</code>, <code>ShopifyCreateOrders</code>, and <code>ShopifyUpdatedOrders</code>.</p>
    /// - On success, responds with [`PutIntegrationOutput`](crate::operation::put_integration::PutIntegrationOutput) with field(s):
    ///   - [`domain_name(Option<String>)`](crate::operation::put_integration::PutIntegrationOutput::domain_name): <p>The unique name of the domain.</p>
    ///   - [`uri(Option<String>)`](crate::operation::put_integration::PutIntegrationOutput::uri): <p>The URI of the S3 bucket or any other type of data source.</p>
    ///   - [`object_type_name(Option<String>)`](crate::operation::put_integration::PutIntegrationOutput::object_type_name): <p>The name of the profile object type.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::put_integration::PutIntegrationOutput::created_at): <p>The timestamp of when the domain was created.</p>
    ///   - [`last_updated_at(Option<DateTime>)`](crate::operation::put_integration::PutIntegrationOutput::last_updated_at): <p>The timestamp of when the domain was most recently edited.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::put_integration::PutIntegrationOutput::tags): <p>The tags used to organize, track, or control access for this resource.</p>
    ///   - [`object_type_names(Option<HashMap<String, String>>)`](crate::operation::put_integration::PutIntegrationOutput::object_type_names): <p>A map in which each key is an event type from an external application such as Segment or Shopify, and each value is an <code>ObjectTypeName</code> (template) used to ingest the event. It supports the following event types: <code>SegmentIdentify</code>, <code>ShopifyCreateCustomers</code>, <code>ShopifyUpdateCustomers</code>, <code>ShopifyCreateDraftOrders</code>, <code>ShopifyUpdateDraftOrders</code>, <code>ShopifyCreateOrders</code>, and <code>ShopifyUpdatedOrders</code>.</p>
    ///   - [`workflow_id(Option<String>)`](crate::operation::put_integration::PutIntegrationOutput::workflow_id): <p>Unique identifier for the workflow.</p>
    ///   - [`is_unstructured(Option<bool>)`](crate::operation::put_integration::PutIntegrationOutput::is_unstructured): <p>Boolean that shows if the Flow that's associated with the Integration is created in Amazon Appflow, or with ObjectTypeName equals _unstructured via API/CLI in flowDefinition.</p>
    /// - On failure, responds with [`SdkError<PutIntegrationError>`](crate::operation::put_integration::PutIntegrationError)
    pub fn put_integration(
        &self,
    ) -> crate::operation::put_integration::builders::PutIntegrationFluentBuilder {
        crate::operation::put_integration::builders::PutIntegrationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
