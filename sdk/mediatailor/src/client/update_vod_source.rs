// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateVodSource`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`http_package_configurations(Vec<HttpPackageConfiguration>)`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::http_package_configurations) / [`set_http_package_configurations(Option<Vec<HttpPackageConfiguration>>)`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::set_http_package_configurations): <p>A list of HTTP package configurations for the VOD source on this account.</p>
    ///   - [`source_location_name(impl ::std::convert::Into<String>)`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::source_location_name) / [`set_source_location_name(Option<String>)`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::set_source_location_name): <p>The name of the source location associated with this VOD Source.</p>
    ///   - [`vod_source_name(impl ::std::convert::Into<String>)`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::vod_source_name) / [`set_vod_source_name(Option<String>)`](crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::set_vod_source_name): <p>The name of the VOD source.</p>
    /// - On success, responds with [`UpdateVodSourceOutput`](crate::operation::update_vod_source::UpdateVodSourceOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::arn): <p>The Amazon Resource Name (ARN) associated with the VOD source.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::creation_time): <p>The timestamp that indicates when the VOD source was created.</p>
    ///   - [`http_package_configurations(Option<Vec<HttpPackageConfiguration>>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::http_package_configurations): <p>A list of HTTP package configurations for the VOD source on this account.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::last_modified_time): <p>The timestamp that indicates when the VOD source was last modified.</p>
    ///   - [`source_location_name(Option<String>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::source_location_name): <p>The name of the source location associated with the VOD source.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::tags): <p>The tags to assign to the VOD source. Tags are key-value pairs that you can associate with Amazon resources to help with organization, access control, and cost tracking. For more information, see <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/tagging.html">Tagging AWS Elemental MediaTailor Resources</a>.</p>
    ///   - [`vod_source_name(Option<String>)`](crate::operation::update_vod_source::UpdateVodSourceOutput::vod_source_name): <p>The name of the VOD source.</p>
    /// - On failure, responds with [`SdkError<UpdateVodSourceError>`](crate::operation::update_vod_source::UpdateVodSourceError)
    pub fn update_vod_source(
        &self,
    ) -> crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder {
        crate::operation::update_vod_source::builders::UpdateVodSourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
