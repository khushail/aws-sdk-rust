// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains an optional backup plan display name and an array of <code>BackupRule</code> objects, each of which specifies a backup rule. Each rule in a backup plan is a separate scheduled task. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BackupPlanInput {
    /// <p>The display name of a backup plan. Must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    #[doc(hidden)]
    pub backup_plan_name: ::std::option::Option<::std::string::String>,
    /// <p>An array of <code>BackupRule</code> objects, each of which specifies a scheduled task that is used to back up a selection of resources.</p>
    #[doc(hidden)]
    pub rules: ::std::option::Option<::std::vec::Vec<crate::types::BackupRuleInput>>,
    /// <p>Specifies a list of <code>BackupOptions</code> for each resource type. These settings are only available for Windows Volume Shadow Copy Service (VSS) backup jobs.</p>
    #[doc(hidden)]
    pub advanced_backup_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::AdvancedBackupSetting>>,
}
impl BackupPlanInput {
    /// <p>The display name of a backup plan. Must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    pub fn backup_plan_name(&self) -> ::std::option::Option<&str> {
        self.backup_plan_name.as_deref()
    }
    /// <p>An array of <code>BackupRule</code> objects, each of which specifies a scheduled task that is used to back up a selection of resources.</p>
    pub fn rules(&self) -> ::std::option::Option<&[crate::types::BackupRuleInput]> {
        self.rules.as_deref()
    }
    /// <p>Specifies a list of <code>BackupOptions</code> for each resource type. These settings are only available for Windows Volume Shadow Copy Service (VSS) backup jobs.</p>
    pub fn advanced_backup_settings(
        &self,
    ) -> ::std::option::Option<&[crate::types::AdvancedBackupSetting]> {
        self.advanced_backup_settings.as_deref()
    }
}
impl BackupPlanInput {
    /// Creates a new builder-style object to manufacture [`BackupPlanInput`](crate::types::BackupPlanInput).
    pub fn builder() -> crate::types::builders::BackupPlanInputBuilder {
        crate::types::builders::BackupPlanInputBuilder::default()
    }
}

/// A builder for [`BackupPlanInput`](crate::types::BackupPlanInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BackupPlanInputBuilder {
    pub(crate) backup_plan_name: ::std::option::Option<::std::string::String>,
    pub(crate) rules: ::std::option::Option<::std::vec::Vec<crate::types::BackupRuleInput>>,
    pub(crate) advanced_backup_settings:
        ::std::option::Option<::std::vec::Vec<crate::types::AdvancedBackupSetting>>,
}
impl BackupPlanInputBuilder {
    /// <p>The display name of a backup plan. Must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    pub fn backup_plan_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.backup_plan_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of a backup plan. Must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    pub fn set_backup_plan_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.backup_plan_name = input;
        self
    }
    /// Appends an item to `rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>An array of <code>BackupRule</code> objects, each of which specifies a scheduled task that is used to back up a selection of resources.</p>
    pub fn rules(mut self, input: crate::types::BackupRuleInput) -> Self {
        let mut v = self.rules.unwrap_or_default();
        v.push(input);
        self.rules = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of <code>BackupRule</code> objects, each of which specifies a scheduled task that is used to back up a selection of resources.</p>
    pub fn set_rules(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BackupRuleInput>>,
    ) -> Self {
        self.rules = input;
        self
    }
    /// Appends an item to `advanced_backup_settings`.
    ///
    /// To override the contents of this collection use [`set_advanced_backup_settings`](Self::set_advanced_backup_settings).
    ///
    /// <p>Specifies a list of <code>BackupOptions</code> for each resource type. These settings are only available for Windows Volume Shadow Copy Service (VSS) backup jobs.</p>
    pub fn advanced_backup_settings(mut self, input: crate::types::AdvancedBackupSetting) -> Self {
        let mut v = self.advanced_backup_settings.unwrap_or_default();
        v.push(input);
        self.advanced_backup_settings = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies a list of <code>BackupOptions</code> for each resource type. These settings are only available for Windows Volume Shadow Copy Service (VSS) backup jobs.</p>
    pub fn set_advanced_backup_settings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AdvancedBackupSetting>>,
    ) -> Self {
        self.advanced_backup_settings = input;
        self
    }
    /// Consumes the builder and constructs a [`BackupPlanInput`](crate::types::BackupPlanInput).
    pub fn build(self) -> crate::types::BackupPlanInput {
        crate::types::BackupPlanInput {
            backup_plan_name: self.backup_plan_name,
            rules: self.rules,
            advanced_backup_settings: self.advanced_backup_settings,
        }
    }
}
