// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateApiMapping`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl ::std::convert::Into<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::set_api_id): <p>The API identifier.</p>
    ///   - [`api_mapping_id(impl ::std::convert::Into<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::api_mapping_id) / [`set_api_mapping_id(Option<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::set_api_mapping_id): <p>The API mapping identifier.</p>
    ///   - [`api_mapping_key(impl ::std::convert::Into<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::api_mapping_key) / [`set_api_mapping_key(Option<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::set_api_mapping_key): <p>The API mapping key.</p>
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::set_domain_name): <p>The domain name.</p>
    ///   - [`stage(impl ::std::convert::Into<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::stage) / [`set_stage(Option<String>)`](crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::set_stage): <p>The API stage.</p>
    /// - On success, responds with [`UpdateApiMappingOutput`](crate::operation::update_api_mapping::UpdateApiMappingOutput) with field(s):
    ///   - [`api_id(Option<String>)`](crate::operation::update_api_mapping::UpdateApiMappingOutput::api_id): <p>The API identifier.</p>
    ///   - [`api_mapping_id(Option<String>)`](crate::operation::update_api_mapping::UpdateApiMappingOutput::api_mapping_id): <p>The API mapping identifier.</p>
    ///   - [`api_mapping_key(Option<String>)`](crate::operation::update_api_mapping::UpdateApiMappingOutput::api_mapping_key): <p>The API mapping key.</p>
    ///   - [`stage(Option<String>)`](crate::operation::update_api_mapping::UpdateApiMappingOutput::stage): <p>The API stage.</p>
    /// - On failure, responds with [`SdkError<UpdateApiMappingError>`](crate::operation::update_api_mapping::UpdateApiMappingError)
    pub fn update_api_mapping(
        &self,
    ) -> crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder {
        crate::operation::update_api_mapping::builders::UpdateApiMappingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
