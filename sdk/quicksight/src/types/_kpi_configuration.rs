// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration of a KPI visual.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KpiConfiguration {
    /// <p>The field well configuration of a KPI visual.</p>
    #[doc(hidden)]
    pub field_wells: ::std::option::Option<crate::types::KpiFieldWells>,
    /// <p>The sort configuration of a KPI visual.</p>
    #[doc(hidden)]
    pub sort_configuration: ::std::option::Option<crate::types::KpiSortConfiguration>,
    /// <p>The options that determine the presentation of a KPI visual.</p>
    #[doc(hidden)]
    pub kpi_options: ::std::option::Option<crate::types::KpiOptions>,
}
impl KpiConfiguration {
    /// <p>The field well configuration of a KPI visual.</p>
    pub fn field_wells(&self) -> ::std::option::Option<&crate::types::KpiFieldWells> {
        self.field_wells.as_ref()
    }
    /// <p>The sort configuration of a KPI visual.</p>
    pub fn sort_configuration(&self) -> ::std::option::Option<&crate::types::KpiSortConfiguration> {
        self.sort_configuration.as_ref()
    }
    /// <p>The options that determine the presentation of a KPI visual.</p>
    pub fn kpi_options(&self) -> ::std::option::Option<&crate::types::KpiOptions> {
        self.kpi_options.as_ref()
    }
}
impl KpiConfiguration {
    /// Creates a new builder-style object to manufacture [`KpiConfiguration`](crate::types::KpiConfiguration).
    pub fn builder() -> crate::types::builders::KpiConfigurationBuilder {
        crate::types::builders::KpiConfigurationBuilder::default()
    }
}

/// A builder for [`KpiConfiguration`](crate::types::KpiConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KpiConfigurationBuilder {
    pub(crate) field_wells: ::std::option::Option<crate::types::KpiFieldWells>,
    pub(crate) sort_configuration: ::std::option::Option<crate::types::KpiSortConfiguration>,
    pub(crate) kpi_options: ::std::option::Option<crate::types::KpiOptions>,
}
impl KpiConfigurationBuilder {
    /// <p>The field well configuration of a KPI visual.</p>
    pub fn field_wells(mut self, input: crate::types::KpiFieldWells) -> Self {
        self.field_wells = ::std::option::Option::Some(input);
        self
    }
    /// <p>The field well configuration of a KPI visual.</p>
    pub fn set_field_wells(
        mut self,
        input: ::std::option::Option<crate::types::KpiFieldWells>,
    ) -> Self {
        self.field_wells = input;
        self
    }
    /// <p>The sort configuration of a KPI visual.</p>
    pub fn sort_configuration(mut self, input: crate::types::KpiSortConfiguration) -> Self {
        self.sort_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The sort configuration of a KPI visual.</p>
    pub fn set_sort_configuration(
        mut self,
        input: ::std::option::Option<crate::types::KpiSortConfiguration>,
    ) -> Self {
        self.sort_configuration = input;
        self
    }
    /// <p>The options that determine the presentation of a KPI visual.</p>
    pub fn kpi_options(mut self, input: crate::types::KpiOptions) -> Self {
        self.kpi_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options that determine the presentation of a KPI visual.</p>
    pub fn set_kpi_options(
        mut self,
        input: ::std::option::Option<crate::types::KpiOptions>,
    ) -> Self {
        self.kpi_options = input;
        self
    }
    /// Consumes the builder and constructs a [`KpiConfiguration`](crate::types::KpiConfiguration).
    pub fn build(self) -> crate::types::KpiConfiguration {
        crate::types::KpiConfiguration {
            field_wells: self.field_wells,
            sort_configuration: self.sort_configuration,
            kpi_options: self.kpi_options,
        }
    }
}
