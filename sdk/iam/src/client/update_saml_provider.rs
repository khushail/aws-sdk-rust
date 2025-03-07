// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateSAMLProvider`](crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`saml_metadata_document(impl ::std::convert::Into<String>)`](crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder::saml_metadata_document) / [`set_saml_metadata_document(Option<String>)`](crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder::set_saml_metadata_document): <p>An XML document generated by an identity provider (IdP) that supports SAML 2.0. The document includes the issuer's name, expiration information, and keys that can be used to validate the SAML authentication response (assertions) that are received from the IdP. You must generate the metadata document using the identity management software that is used as your organization's IdP.</p>
    ///   - [`saml_provider_arn(impl ::std::convert::Into<String>)`](crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder::saml_provider_arn) / [`set_saml_provider_arn(Option<String>)`](crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder::set_saml_provider_arn): <p>The Amazon Resource Name (ARN) of the SAML provider to update.</p>  <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    /// - On success, responds with [`UpdateSamlProviderOutput`](crate::operation::update_saml_provider::UpdateSamlProviderOutput) with field(s):
    ///   - [`saml_provider_arn(Option<String>)`](crate::operation::update_saml_provider::UpdateSamlProviderOutput::saml_provider_arn): <p>The Amazon Resource Name (ARN) of the SAML provider that was updated.</p>
    /// - On failure, responds with [`SdkError<UpdateSAMLProviderError>`](crate::operation::update_saml_provider::UpdateSAMLProviderError)
    pub fn update_saml_provider(
        &self,
    ) -> crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder {
        crate::operation::update_saml_provider::builders::UpdateSAMLProviderFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
