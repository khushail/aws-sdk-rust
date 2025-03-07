// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateMissionProfile`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`mission_profile_id(impl ::std::convert::Into<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::mission_profile_id) / [`set_mission_profile_id(Option<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_mission_profile_id): <p>UUID of a mission profile.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_name): <p>Name of a mission profile.</p>
    ///   - [`contact_pre_pass_duration_seconds(i32)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::contact_pre_pass_duration_seconds) / [`set_contact_pre_pass_duration_seconds(Option<i32>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_contact_pre_pass_duration_seconds): <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    ///   - [`contact_post_pass_duration_seconds(i32)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::contact_post_pass_duration_seconds) / [`set_contact_post_pass_duration_seconds(Option<i32>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_contact_post_pass_duration_seconds): <p>Amount of time after a contact ends that you’d like to receive a CloudWatch event indicating the pass has finished.</p>
    ///   - [`minimum_viable_contact_duration_seconds(i32)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::minimum_viable_contact_duration_seconds) / [`set_minimum_viable_contact_duration_seconds(Option<i32>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_minimum_viable_contact_duration_seconds): <p>Smallest amount of time in seconds that you’d like to see for an available contact. AWS Ground Station will not present you with contacts shorter than this duration.</p>
    ///   - [`dataflow_edges(Vec<Vec<String>>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::dataflow_edges) / [`set_dataflow_edges(Option<Vec<Vec<String>>>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_dataflow_edges): <p>A list of lists of ARNs. Each list of ARNs is an edge, with a <i>from</i> <code>Config</code> and a <i>to</i> <code>Config</code>.</p>
    ///   - [`tracking_config_arn(impl ::std::convert::Into<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::tracking_config_arn) / [`set_tracking_config_arn(Option<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_tracking_config_arn): <p>ARN of a tracking <code>Config</code>.</p>
    ///   - [`streams_kms_key(KmsKey)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::streams_kms_key) / [`set_streams_kms_key(Option<KmsKey>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_streams_kms_key): <p>KMS key to use for encrypting streams.</p>
    ///   - [`streams_kms_role(impl ::std::convert::Into<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::streams_kms_role) / [`set_streams_kms_role(Option<String>)`](crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::set_streams_kms_role): <p>Role to use for encrypting streams with KMS key.</p>
    /// - On success, responds with [`UpdateMissionProfileOutput`](crate::operation::update_mission_profile::UpdateMissionProfileOutput) with field(s):
    ///   - [`mission_profile_id(Option<String>)`](crate::operation::update_mission_profile::UpdateMissionProfileOutput::mission_profile_id): <p>UUID of a mission profile.</p>
    /// - On failure, responds with [`SdkError<UpdateMissionProfileError>`](crate::operation::update_mission_profile::UpdateMissionProfileError)
    pub fn update_mission_profile(
        &self,
    ) -> crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder {
        crate::operation::update_mission_profile::builders::UpdateMissionProfileFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
