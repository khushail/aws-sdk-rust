// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetObjectAttributes`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl ::std::convert::Into<String>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::directory_arn) / [`set_directory_arn(Option<String>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::set_directory_arn): <p>The Amazon Resource Name (ARN) that is associated with the <code>Directory</code> where the object resides.</p>
    ///   - [`object_reference(ObjectReference)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::object_reference) / [`set_object_reference(Option<ObjectReference>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::set_object_reference): <p>Reference that identifies the object whose attributes will be retrieved.</p>
    ///   - [`consistency_level(ConsistencyLevel)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::consistency_level) / [`set_consistency_level(Option<ConsistencyLevel>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::set_consistency_level): <p>The consistency level at which to retrieve the attributes on an object.</p>
    ///   - [`schema_facet(SchemaFacet)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::schema_facet) / [`set_schema_facet(Option<SchemaFacet>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::set_schema_facet): <p>Identifier for the facet whose attributes will be retrieved. See <code>SchemaFacet</code> for details.</p>
    ///   - [`attribute_names(Vec<String>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::attribute_names) / [`set_attribute_names(Option<Vec<String>>)`](crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::set_attribute_names): <p>List of attribute names whose values will be retrieved.</p>
    /// - On success, responds with [`GetObjectAttributesOutput`](crate::operation::get_object_attributes::GetObjectAttributesOutput) with field(s):
    ///   - [`attributes(Option<Vec<AttributeKeyAndValue>>)`](crate::operation::get_object_attributes::GetObjectAttributesOutput::attributes): <p>The attributes that are associated with the object.</p>
    /// - On failure, responds with [`SdkError<GetObjectAttributesError>`](crate::operation::get_object_attributes::GetObjectAttributesError)
    pub fn get_object_attributes(
        &self,
    ) -> crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder {
        crate::operation::get_object_attributes::builders::GetObjectAttributesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
