// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateCreatedArtifact`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`progress_update_stream(impl ::std::convert::Into<String>)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::progress_update_stream) / [`set_progress_update_stream(Option<String>)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::set_progress_update_stream): <p>The name of the ProgressUpdateStream. </p>
    ///   - [`migration_task_name(impl ::std::convert::Into<String>)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::migration_task_name) / [`set_migration_task_name(Option<String>)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::set_migration_task_name): <p>Unique identifier that references the migration task to be disassociated with the artifact. <i>Do not store personal data in this field.</i> </p>
    ///   - [`created_artifact_name(impl ::std::convert::Into<String>)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::created_artifact_name) / [`set_created_artifact_name(Option<String>)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::set_created_artifact_name): <p>An ARN of the AWS resource related to the migration (e.g., AMI, EC2 instance, RDS instance, etc.)</p>
    ///   - [`dry_run(bool)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::dry_run) / [`set_dry_run(bool)`](crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::set_dry_run): <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    /// - On success, responds with [`DisassociateCreatedArtifactOutput`](crate::operation::disassociate_created_artifact::DisassociateCreatedArtifactOutput)
    /// - On failure, responds with [`SdkError<DisassociateCreatedArtifactError>`](crate::operation::disassociate_created_artifact::DisassociateCreatedArtifactError)
    pub fn disassociate_created_artifact(&self) -> crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder{
        crate::operation::disassociate_created_artifact::builders::DisassociateCreatedArtifactFluentBuilder::new(self.handle.clone())
    }
}
