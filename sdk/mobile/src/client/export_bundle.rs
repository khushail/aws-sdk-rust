// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExportBundle`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bundle_id(impl ::std::convert::Into<String>)`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder::bundle_id) / [`set_bundle_id(Option<String>)`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder::set_bundle_id): <p> Unique bundle identifier. </p>
    ///   - [`project_id(impl ::std::convert::Into<String>)`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder::project_id) / [`set_project_id(Option<String>)`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder::set_project_id): <p> Unique project identifier. </p>
    ///   - [`platform(Platform)`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder::platform) / [`set_platform(Option<Platform>)`](crate::operation::export_bundle::builders::ExportBundleFluentBuilder::set_platform): <p> Developer desktop or target application platform. </p>
    /// - On success, responds with [`ExportBundleOutput`](crate::operation::export_bundle::ExportBundleOutput) with field(s):
    ///   - [`download_url(Option<String>)`](crate::operation::export_bundle::ExportBundleOutput::download_url): <p> URL which contains the custom-generated SDK and tool packages used to integrate the client mobile app or web app with the AWS resources created by the AWS Mobile Hub project. </p>
    /// - On failure, responds with [`SdkError<ExportBundleError>`](crate::operation::export_bundle::ExportBundleError)
    pub fn export_bundle(
        &self,
    ) -> crate::operation::export_bundle::builders::ExportBundleFluentBuilder {
        crate::operation::export_bundle::builders::ExportBundleFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
