// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that represents a filter used to select items for a topic.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TopicFilter {
    /// <p>A description of the filter used to select items for a topic.</p>
    #[doc(hidden)]
    pub filter_description: ::std::option::Option<::std::string::String>,
    /// <p>The class of the filter. Valid values for this structure are <code>ENFORCED_VALUE_FILTER</code>, <code>CONDITIONAL_VALUE_FILTER</code>, and <code>NAMED_VALUE_FILTER</code>.</p>
    #[doc(hidden)]
    pub filter_class: ::std::option::Option<crate::types::FilterClass>,
    /// <p>The name of the filter.</p>
    #[doc(hidden)]
    pub filter_name: ::std::option::Option<::std::string::String>,
    /// <p>The other names or aliases for the filter.</p>
    #[doc(hidden)]
    pub filter_synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The name of the field that the filter operates on.</p>
    #[doc(hidden)]
    pub operand_field_name: ::std::option::Option<::std::string::String>,
    /// <p>The type of the filter. Valid values for this structure are <code>CATEGORY_FILTER</code>, <code>NUMERIC_EQUALITY_FILTER</code>, <code>NUMERIC_RANGE_FILTER</code>, <code>DATE_RANGE_FILTER</code>, and <code>RELATIVE_DATE_FILTER</code>.</p>
    #[doc(hidden)]
    pub filter_type: ::std::option::Option<crate::types::NamedFilterType>,
    /// <p>The category filter that is associated with this filter.</p>
    #[doc(hidden)]
    pub category_filter: ::std::option::Option<crate::types::TopicCategoryFilter>,
    /// <p>The numeric equality filter.</p>
    #[doc(hidden)]
    pub numeric_equality_filter: ::std::option::Option<crate::types::TopicNumericEqualityFilter>,
    /// <p>The numeric range filter.</p>
    #[doc(hidden)]
    pub numeric_range_filter: ::std::option::Option<crate::types::TopicNumericRangeFilter>,
    /// <p>The date range filter.</p>
    #[doc(hidden)]
    pub date_range_filter: ::std::option::Option<crate::types::TopicDateRangeFilter>,
    /// <p>The relative date filter.</p>
    #[doc(hidden)]
    pub relative_date_filter: ::std::option::Option<crate::types::TopicRelativeDateFilter>,
}
impl TopicFilter {
    /// <p>A description of the filter used to select items for a topic.</p>
    pub fn filter_description(&self) -> ::std::option::Option<&str> {
        self.filter_description.as_deref()
    }
    /// <p>The class of the filter. Valid values for this structure are <code>ENFORCED_VALUE_FILTER</code>, <code>CONDITIONAL_VALUE_FILTER</code>, and <code>NAMED_VALUE_FILTER</code>.</p>
    pub fn filter_class(&self) -> ::std::option::Option<&crate::types::FilterClass> {
        self.filter_class.as_ref()
    }
    /// <p>The name of the filter.</p>
    pub fn filter_name(&self) -> ::std::option::Option<&str> {
        self.filter_name.as_deref()
    }
    /// <p>The other names or aliases for the filter.</p>
    pub fn filter_synonyms(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.filter_synonyms.as_deref()
    }
    /// <p>The name of the field that the filter operates on.</p>
    pub fn operand_field_name(&self) -> ::std::option::Option<&str> {
        self.operand_field_name.as_deref()
    }
    /// <p>The type of the filter. Valid values for this structure are <code>CATEGORY_FILTER</code>, <code>NUMERIC_EQUALITY_FILTER</code>, <code>NUMERIC_RANGE_FILTER</code>, <code>DATE_RANGE_FILTER</code>, and <code>RELATIVE_DATE_FILTER</code>.</p>
    pub fn filter_type(&self) -> ::std::option::Option<&crate::types::NamedFilterType> {
        self.filter_type.as_ref()
    }
    /// <p>The category filter that is associated with this filter.</p>
    pub fn category_filter(&self) -> ::std::option::Option<&crate::types::TopicCategoryFilter> {
        self.category_filter.as_ref()
    }
    /// <p>The numeric equality filter.</p>
    pub fn numeric_equality_filter(
        &self,
    ) -> ::std::option::Option<&crate::types::TopicNumericEqualityFilter> {
        self.numeric_equality_filter.as_ref()
    }
    /// <p>The numeric range filter.</p>
    pub fn numeric_range_filter(
        &self,
    ) -> ::std::option::Option<&crate::types::TopicNumericRangeFilter> {
        self.numeric_range_filter.as_ref()
    }
    /// <p>The date range filter.</p>
    pub fn date_range_filter(&self) -> ::std::option::Option<&crate::types::TopicDateRangeFilter> {
        self.date_range_filter.as_ref()
    }
    /// <p>The relative date filter.</p>
    pub fn relative_date_filter(
        &self,
    ) -> ::std::option::Option<&crate::types::TopicRelativeDateFilter> {
        self.relative_date_filter.as_ref()
    }
}
impl TopicFilter {
    /// Creates a new builder-style object to manufacture [`TopicFilter`](crate::types::TopicFilter).
    pub fn builder() -> crate::types::builders::TopicFilterBuilder {
        crate::types::builders::TopicFilterBuilder::default()
    }
}

/// A builder for [`TopicFilter`](crate::types::TopicFilter).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TopicFilterBuilder {
    pub(crate) filter_description: ::std::option::Option<::std::string::String>,
    pub(crate) filter_class: ::std::option::Option<crate::types::FilterClass>,
    pub(crate) filter_name: ::std::option::Option<::std::string::String>,
    pub(crate) filter_synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) operand_field_name: ::std::option::Option<::std::string::String>,
    pub(crate) filter_type: ::std::option::Option<crate::types::NamedFilterType>,
    pub(crate) category_filter: ::std::option::Option<crate::types::TopicCategoryFilter>,
    pub(crate) numeric_equality_filter:
        ::std::option::Option<crate::types::TopicNumericEqualityFilter>,
    pub(crate) numeric_range_filter: ::std::option::Option<crate::types::TopicNumericRangeFilter>,
    pub(crate) date_range_filter: ::std::option::Option<crate::types::TopicDateRangeFilter>,
    pub(crate) relative_date_filter: ::std::option::Option<crate::types::TopicRelativeDateFilter>,
}
impl TopicFilterBuilder {
    /// <p>A description of the filter used to select items for a topic.</p>
    pub fn filter_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.filter_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the filter used to select items for a topic.</p>
    pub fn set_filter_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.filter_description = input;
        self
    }
    /// <p>The class of the filter. Valid values for this structure are <code>ENFORCED_VALUE_FILTER</code>, <code>CONDITIONAL_VALUE_FILTER</code>, and <code>NAMED_VALUE_FILTER</code>.</p>
    pub fn filter_class(mut self, input: crate::types::FilterClass) -> Self {
        self.filter_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>The class of the filter. Valid values for this structure are <code>ENFORCED_VALUE_FILTER</code>, <code>CONDITIONAL_VALUE_FILTER</code>, and <code>NAMED_VALUE_FILTER</code>.</p>
    pub fn set_filter_class(
        mut self,
        input: ::std::option::Option<crate::types::FilterClass>,
    ) -> Self {
        self.filter_class = input;
        self
    }
    /// <p>The name of the filter.</p>
    pub fn filter_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filter_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the filter.</p>
    pub fn set_filter_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filter_name = input;
        self
    }
    /// Appends an item to `filter_synonyms`.
    ///
    /// To override the contents of this collection use [`set_filter_synonyms`](Self::set_filter_synonyms).
    ///
    /// <p>The other names or aliases for the filter.</p>
    pub fn filter_synonyms(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.filter_synonyms.unwrap_or_default();
        v.push(input.into());
        self.filter_synonyms = ::std::option::Option::Some(v);
        self
    }
    /// <p>The other names or aliases for the filter.</p>
    pub fn set_filter_synonyms(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.filter_synonyms = input;
        self
    }
    /// <p>The name of the field that the filter operates on.</p>
    pub fn operand_field_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.operand_field_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the field that the filter operates on.</p>
    pub fn set_operand_field_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.operand_field_name = input;
        self
    }
    /// <p>The type of the filter. Valid values for this structure are <code>CATEGORY_FILTER</code>, <code>NUMERIC_EQUALITY_FILTER</code>, <code>NUMERIC_RANGE_FILTER</code>, <code>DATE_RANGE_FILTER</code>, and <code>RELATIVE_DATE_FILTER</code>.</p>
    pub fn filter_type(mut self, input: crate::types::NamedFilterType) -> Self {
        self.filter_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the filter. Valid values for this structure are <code>CATEGORY_FILTER</code>, <code>NUMERIC_EQUALITY_FILTER</code>, <code>NUMERIC_RANGE_FILTER</code>, <code>DATE_RANGE_FILTER</code>, and <code>RELATIVE_DATE_FILTER</code>.</p>
    pub fn set_filter_type(
        mut self,
        input: ::std::option::Option<crate::types::NamedFilterType>,
    ) -> Self {
        self.filter_type = input;
        self
    }
    /// <p>The category filter that is associated with this filter.</p>
    pub fn category_filter(mut self, input: crate::types::TopicCategoryFilter) -> Self {
        self.category_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The category filter that is associated with this filter.</p>
    pub fn set_category_filter(
        mut self,
        input: ::std::option::Option<crate::types::TopicCategoryFilter>,
    ) -> Self {
        self.category_filter = input;
        self
    }
    /// <p>The numeric equality filter.</p>
    pub fn numeric_equality_filter(
        mut self,
        input: crate::types::TopicNumericEqualityFilter,
    ) -> Self {
        self.numeric_equality_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The numeric equality filter.</p>
    pub fn set_numeric_equality_filter(
        mut self,
        input: ::std::option::Option<crate::types::TopicNumericEqualityFilter>,
    ) -> Self {
        self.numeric_equality_filter = input;
        self
    }
    /// <p>The numeric range filter.</p>
    pub fn numeric_range_filter(mut self, input: crate::types::TopicNumericRangeFilter) -> Self {
        self.numeric_range_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The numeric range filter.</p>
    pub fn set_numeric_range_filter(
        mut self,
        input: ::std::option::Option<crate::types::TopicNumericRangeFilter>,
    ) -> Self {
        self.numeric_range_filter = input;
        self
    }
    /// <p>The date range filter.</p>
    pub fn date_range_filter(mut self, input: crate::types::TopicDateRangeFilter) -> Self {
        self.date_range_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date range filter.</p>
    pub fn set_date_range_filter(
        mut self,
        input: ::std::option::Option<crate::types::TopicDateRangeFilter>,
    ) -> Self {
        self.date_range_filter = input;
        self
    }
    /// <p>The relative date filter.</p>
    pub fn relative_date_filter(mut self, input: crate::types::TopicRelativeDateFilter) -> Self {
        self.relative_date_filter = ::std::option::Option::Some(input);
        self
    }
    /// <p>The relative date filter.</p>
    pub fn set_relative_date_filter(
        mut self,
        input: ::std::option::Option<crate::types::TopicRelativeDateFilter>,
    ) -> Self {
        self.relative_date_filter = input;
        self
    }
    /// Consumes the builder and constructs a [`TopicFilter`](crate::types::TopicFilter).
    pub fn build(self) -> crate::types::TopicFilter {
        crate::types::TopicFilter {
            filter_description: self.filter_description,
            filter_class: self.filter_class,
            filter_name: self.filter_name,
            filter_synonyms: self.filter_synonyms,
            operand_field_name: self.operand_field_name,
            filter_type: self.filter_type,
            category_filter: self.category_filter,
            numeric_equality_filter: self.numeric_equality_filter,
            numeric_range_filter: self.numeric_range_filter,
            date_range_filter: self.date_range_filter,
            relative_date_filter: self.relative_date_filter,
        }
    }
}
