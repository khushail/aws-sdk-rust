// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AttachTypedLink`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_arn(impl ::std::convert::Into<String>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::directory_arn) / [`set_directory_arn(Option<String>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::set_directory_arn): <p>The Amazon Resource Name (ARN) of the directory where you want to attach the typed link.</p>
    ///   - [`source_object_reference(ObjectReference)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::source_object_reference) / [`set_source_object_reference(Option<ObjectReference>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::set_source_object_reference): <p>Identifies the source object that the typed link will attach to.</p>
    ///   - [`target_object_reference(ObjectReference)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::target_object_reference) / [`set_target_object_reference(Option<ObjectReference>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::set_target_object_reference): <p>Identifies the target object that the typed link will attach to.</p>
    ///   - [`typed_link_facet(TypedLinkSchemaAndFacetName)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::typed_link_facet) / [`set_typed_link_facet(Option<TypedLinkSchemaAndFacetName>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::set_typed_link_facet): <p>Identifies the typed link facet that is associated with the typed link.</p>
    ///   - [`attributes(Vec<AttributeNameAndValue>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::attributes) / [`set_attributes(Option<Vec<AttributeNameAndValue>>)`](crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::set_attributes): <p>A set of attributes that are associated with the typed link.</p>
    /// - On success, responds with [`AttachTypedLinkOutput`](crate::operation::attach_typed_link::AttachTypedLinkOutput) with field(s):
    ///   - [`typed_link_specifier(Option<TypedLinkSpecifier>)`](crate::operation::attach_typed_link::AttachTypedLinkOutput::typed_link_specifier): <p>Returns a typed link specifier as output.</p>
    /// - On failure, responds with [`SdkError<AttachTypedLinkError>`](crate::operation::attach_typed_link::AttachTypedLinkError)
    pub fn attach_typed_link(
        &self,
    ) -> crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder {
        crate::operation::attach_typed_link::builders::AttachTypedLinkFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
