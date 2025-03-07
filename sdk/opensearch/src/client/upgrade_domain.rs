// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpgradeDomain`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl ::std::convert::Into<String>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::set_domain_name): <p>Name of the OpenSearch Service domain that you want to upgrade.</p>
    ///   - [`target_version(impl ::std::convert::Into<String>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::target_version) / [`set_target_version(Option<String>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::set_target_version): <p>OpenSearch or Elasticsearch version to which you want to upgrade, in the format Opensearch_X.Y or Elasticsearch_X.Y.</p>
    ///   - [`perform_check_only(bool)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::perform_check_only) / [`set_perform_check_only(Option<bool>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::set_perform_check_only): <p>When true, indicates that an upgrade eligibility check needs to be performed. Does not actually perform the upgrade.</p>
    ///   - [`advanced_options(HashMap<String, String>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::advanced_options) / [`set_advanced_options(Option<HashMap<String, String>>)`](crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::set_advanced_options): <p>Only supports the <code>override_main_response_version</code> parameter and not other advanced options. You can only include this option when upgrading to an OpenSearch version. Specifies whether the domain reports its version as 7.10 so that it continues to work with Elasticsearch OSS clients and plugins.</p>
    /// - On success, responds with [`UpgradeDomainOutput`](crate::operation::upgrade_domain::UpgradeDomainOutput) with field(s):
    ///   - [`upgrade_id(Option<String>)`](crate::operation::upgrade_domain::UpgradeDomainOutput::upgrade_id): <p>The unique identifier of the domain upgrade.</p>
    ///   - [`domain_name(Option<String>)`](crate::operation::upgrade_domain::UpgradeDomainOutput::domain_name): <p>The name of the domain that was upgraded.</p>
    ///   - [`target_version(Option<String>)`](crate::operation::upgrade_domain::UpgradeDomainOutput::target_version): <p>OpenSearch or Elasticsearch version that the domain was upgraded to.</p>
    ///   - [`perform_check_only(Option<bool>)`](crate::operation::upgrade_domain::UpgradeDomainOutput::perform_check_only): <p>When true, indicates that an upgrade eligibility check was performed.</p>
    ///   - [`advanced_options(Option<HashMap<String, String>>)`](crate::operation::upgrade_domain::UpgradeDomainOutput::advanced_options): <p>The advanced options configuration for the domain.</p>
    ///   - [`change_progress_details(Option<ChangeProgressDetails>)`](crate::operation::upgrade_domain::UpgradeDomainOutput::change_progress_details): <p>Container for information about a configuration change happening on a domain.</p>
    /// - On failure, responds with [`SdkError<UpgradeDomainError>`](crate::operation::upgrade_domain::UpgradeDomainError)
    pub fn upgrade_domain(
        &self,
    ) -> crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder {
        crate::operation::upgrade_domain::builders::UpgradeDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
