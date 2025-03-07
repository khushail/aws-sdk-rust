// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains detailed information about a report setting.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReportSetting {
    /// <p>Identifies the report template for the report. Reports are built using a report template. The report templates are:</p>
    /// <p> <code>RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT | COPY_JOB_REPORT | RESTORE_JOB_REPORT</code> </p>
    #[doc(hidden)]
    pub report_template: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Names (ARNs) of the frameworks a report covers.</p>
    #[doc(hidden)]
    pub framework_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The number of frameworks a report covers.</p>
    #[doc(hidden)]
    pub number_of_frameworks: i32,
    /// <p>These are the accounts to be included in the report.</p>
    #[doc(hidden)]
    pub accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>These are the Organizational Units to be included in the report.</p>
    #[doc(hidden)]
    pub organization_units: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>These are the Regions to be included in the report.</p>
    #[doc(hidden)]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ReportSetting {
    /// <p>Identifies the report template for the report. Reports are built using a report template. The report templates are:</p>
    /// <p> <code>RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT | COPY_JOB_REPORT | RESTORE_JOB_REPORT</code> </p>
    pub fn report_template(&self) -> ::std::option::Option<&str> {
        self.report_template.as_deref()
    }
    /// <p>The Amazon Resource Names (ARNs) of the frameworks a report covers.</p>
    pub fn framework_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.framework_arns.as_deref()
    }
    /// <p>The number of frameworks a report covers.</p>
    pub fn number_of_frameworks(&self) -> i32 {
        self.number_of_frameworks
    }
    /// <p>These are the accounts to be included in the report.</p>
    pub fn accounts(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.accounts.as_deref()
    }
    /// <p>These are the Organizational Units to be included in the report.</p>
    pub fn organization_units(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.organization_units.as_deref()
    }
    /// <p>These are the Regions to be included in the report.</p>
    pub fn regions(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.regions.as_deref()
    }
}
impl ReportSetting {
    /// Creates a new builder-style object to manufacture [`ReportSetting`](crate::types::ReportSetting).
    pub fn builder() -> crate::types::builders::ReportSettingBuilder {
        crate::types::builders::ReportSettingBuilder::default()
    }
}

/// A builder for [`ReportSetting`](crate::types::ReportSetting).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReportSettingBuilder {
    pub(crate) report_template: ::std::option::Option<::std::string::String>,
    pub(crate) framework_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) number_of_frameworks: ::std::option::Option<i32>,
    pub(crate) accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) organization_units: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ReportSettingBuilder {
    /// <p>Identifies the report template for the report. Reports are built using a report template. The report templates are:</p>
    /// <p> <code>RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT | COPY_JOB_REPORT | RESTORE_JOB_REPORT</code> </p>
    pub fn report_template(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.report_template = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Identifies the report template for the report. Reports are built using a report template. The report templates are:</p>
    /// <p> <code>RESOURCE_COMPLIANCE_REPORT | CONTROL_COMPLIANCE_REPORT | BACKUP_JOB_REPORT | COPY_JOB_REPORT | RESTORE_JOB_REPORT</code> </p>
    pub fn set_report_template(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.report_template = input;
        self
    }
    /// Appends an item to `framework_arns`.
    ///
    /// To override the contents of this collection use [`set_framework_arns`](Self::set_framework_arns).
    ///
    /// <p>The Amazon Resource Names (ARNs) of the frameworks a report covers.</p>
    pub fn framework_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.framework_arns.unwrap_or_default();
        v.push(input.into());
        self.framework_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Names (ARNs) of the frameworks a report covers.</p>
    pub fn set_framework_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.framework_arns = input;
        self
    }
    /// <p>The number of frameworks a report covers.</p>
    pub fn number_of_frameworks(mut self, input: i32) -> Self {
        self.number_of_frameworks = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of frameworks a report covers.</p>
    pub fn set_number_of_frameworks(mut self, input: ::std::option::Option<i32>) -> Self {
        self.number_of_frameworks = input;
        self
    }
    /// Appends an item to `accounts`.
    ///
    /// To override the contents of this collection use [`set_accounts`](Self::set_accounts).
    ///
    /// <p>These are the accounts to be included in the report.</p>
    pub fn accounts(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.accounts.unwrap_or_default();
        v.push(input.into());
        self.accounts = ::std::option::Option::Some(v);
        self
    }
    /// <p>These are the accounts to be included in the report.</p>
    pub fn set_accounts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.accounts = input;
        self
    }
    /// Appends an item to `organization_units`.
    ///
    /// To override the contents of this collection use [`set_organization_units`](Self::set_organization_units).
    ///
    /// <p>These are the Organizational Units to be included in the report.</p>
    pub fn organization_units(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.organization_units.unwrap_or_default();
        v.push(input.into());
        self.organization_units = ::std::option::Option::Some(v);
        self
    }
    /// <p>These are the Organizational Units to be included in the report.</p>
    pub fn set_organization_units(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.organization_units = input;
        self
    }
    /// Appends an item to `regions`.
    ///
    /// To override the contents of this collection use [`set_regions`](Self::set_regions).
    ///
    /// <p>These are the Regions to be included in the report.</p>
    pub fn regions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.regions.unwrap_or_default();
        v.push(input.into());
        self.regions = ::std::option::Option::Some(v);
        self
    }
    /// <p>These are the Regions to be included in the report.</p>
    pub fn set_regions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.regions = input;
        self
    }
    /// Consumes the builder and constructs a [`ReportSetting`](crate::types::ReportSetting).
    pub fn build(self) -> crate::types::ReportSetting {
        crate::types::ReportSetting {
            report_template: self.report_template,
            framework_arns: self.framework_arns,
            number_of_frameworks: self.number_of_frameworks.unwrap_or_default(),
            accounts: self.accounts,
            organization_units: self.organization_units,
            regions: self.regions,
        }
    }
}
