// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDirectorySetup`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl ::std::convert::Into<String>)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::set_directory_id): <p> The identifier of the directory on which you want to perform the update. </p>
    ///   - [`update_type(UpdateType)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::update_type) / [`set_update_type(Option<UpdateType>)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::set_update_type): <p> The type of update that needs to be performed on the directory. For example, OS. </p>
    ///   - [`os_update_settings(OsUpdateSettings)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::os_update_settings) / [`set_os_update_settings(Option<OsUpdateSettings>)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::set_os_update_settings): <p> The settings for the OS update that needs to be performed on the directory. </p>
    ///   - [`create_snapshot_before_update(bool)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::create_snapshot_before_update) / [`set_create_snapshot_before_update(Option<bool>)`](crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::set_create_snapshot_before_update): <p> The boolean that specifies if a snapshot for the directory needs to be taken before updating the directory. </p>
    /// - On success, responds with [`UpdateDirectorySetupOutput`](crate::operation::update_directory_setup::UpdateDirectorySetupOutput)
    /// - On failure, responds with [`SdkError<UpdateDirectorySetupError>`](crate::operation::update_directory_setup::UpdateDirectorySetupError)
    pub fn update_directory_setup(
        &self,
    ) -> crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder {
        crate::operation::update_directory_setup::builders::UpdateDirectorySetupFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
