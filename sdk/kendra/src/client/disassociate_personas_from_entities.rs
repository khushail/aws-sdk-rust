// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociatePersonasFromEntities`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::set_id): <p>The identifier of your Amazon Kendra experience.</p>
    ///   - [`index_id(impl ::std::convert::Into<String>)`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::index_id) / [`set_index_id(Option<String>)`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::set_index_id): <p>The identifier of the index for your Amazon Kendra experience.</p>
    ///   - [`entity_ids(Vec<String>)`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::entity_ids) / [`set_entity_ids(Option<Vec<String>>)`](crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::set_entity_ids): <p>The identifiers of users or groups in your IAM Identity Center identity source. For example, user IDs could be user emails.</p>
    /// - On success, responds with [`DisassociatePersonasFromEntitiesOutput`](crate::operation::disassociate_personas_from_entities::DisassociatePersonasFromEntitiesOutput) with field(s):
    ///   - [`failed_entity_list(Option<Vec<FailedEntity>>)`](crate::operation::disassociate_personas_from_entities::DisassociatePersonasFromEntitiesOutput::failed_entity_list): <p>Lists the users or groups in your IAM Identity Center identity source that failed to properly remove access to your Amazon Kendra experience.</p>
    /// - On failure, responds with [`SdkError<DisassociatePersonasFromEntitiesError>`](crate::operation::disassociate_personas_from_entities::DisassociatePersonasFromEntitiesError)
    pub fn disassociate_personas_from_entities(&self) -> crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder{
        crate::operation::disassociate_personas_from_entities::builders::DisassociatePersonasFromEntitiesFluentBuilder::new(self.handle.clone())
    }
}
