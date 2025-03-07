// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateConnectivityInfo`](crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connectivity_info(Vec<ConnectivityInfo>)`](crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder::connectivity_info) / [`set_connectivity_info(Option<Vec<ConnectivityInfo>>)`](crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder::set_connectivity_info): A list of connectivity info.
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder::set_thing_name): The thing name.
    /// - On success, responds with [`UpdateConnectivityInfoOutput`](crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput) with field(s):
    ///   - [`message(Option<String>)`](crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput::message): A message about the connectivity info update request.
    ///   - [`version(Option<String>)`](crate::operation::update_connectivity_info::UpdateConnectivityInfoOutput::version): The new version of the connectivity info.
    /// - On failure, responds with [`SdkError<UpdateConnectivityInfoError>`](crate::operation::update_connectivity_info::UpdateConnectivityInfoError)
    pub fn update_connectivity_info(
        &self,
    ) -> crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder
    {
        crate::operation::update_connectivity_info::builders::UpdateConnectivityInfoFluentBuilder::new(self.handle.clone())
    }
}
