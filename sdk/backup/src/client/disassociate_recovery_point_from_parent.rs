// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateRecoveryPointFromParent`](crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_vault_name(impl ::std::convert::Into<String>)`](crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder::backup_vault_name) / [`set_backup_vault_name(Option<String>)`](crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder::set_backup_vault_name): <p>This is the name of a logical container where the child (nested) recovery point is stored. Backup vaults are identified by names that are unique to the account used to create them and the Amazon Web Services Region where they are created. They consist of lowercase letters, numbers, and hyphens.</p>
    ///   - [`recovery_point_arn(impl ::std::convert::Into<String>)`](crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder::recovery_point_arn) / [`set_recovery_point_arn(Option<String>)`](crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder::set_recovery_point_arn): <p>This is the Amazon Resource Name (ARN) that uniquely identifies the child (nested) recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45.</code> </p>
    /// - On success, responds with [`DisassociateRecoveryPointFromParentOutput`](crate::operation::disassociate_recovery_point_from_parent::DisassociateRecoveryPointFromParentOutput)
    /// - On failure, responds with [`SdkError<DisassociateRecoveryPointFromParentError>`](crate::operation::disassociate_recovery_point_from_parent::DisassociateRecoveryPointFromParentError)
    pub fn disassociate_recovery_point_from_parent(&self) -> crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder{
        crate::operation::disassociate_recovery_point_from_parent::builders::DisassociateRecoveryPointFromParentFluentBuilder::new(self.handle.clone())
    }
}
