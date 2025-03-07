// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a recommendation for a CloudWatch alarm.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AlarmRecommendation {
    /// <p>The identifier of the alarm recommendation.</p>
    #[doc(hidden)]
    pub recommendation_id: ::std::option::Option<::std::string::String>,
    /// <p>The reference identifier of the alarm recommendation.</p>
    #[doc(hidden)]
    pub reference_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the alarm recommendation.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the recommendation.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The type of alarm recommendation.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::AlarmType>,
    /// <p>The Application Component for the CloudWatch alarm recommendation.</p>
    #[doc(hidden)]
    pub app_component_name: ::std::option::Option<::std::string::String>,
    /// <p>The list of CloudWatch alarm recommendations.</p>
    #[doc(hidden)]
    pub items: ::std::option::Option<::std::vec::Vec<crate::types::RecommendationItem>>,
    /// <p>The prerequisite for the alarm recommendation.</p>
    #[doc(hidden)]
    pub prerequisite: ::std::option::Option<::std::string::String>,
}
impl AlarmRecommendation {
    /// <p>The identifier of the alarm recommendation.</p>
    pub fn recommendation_id(&self) -> ::std::option::Option<&str> {
        self.recommendation_id.as_deref()
    }
    /// <p>The reference identifier of the alarm recommendation.</p>
    pub fn reference_id(&self) -> ::std::option::Option<&str> {
        self.reference_id.as_deref()
    }
    /// <p>The name of the alarm recommendation.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the recommendation.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The type of alarm recommendation.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::AlarmType> {
        self.r#type.as_ref()
    }
    /// <p>The Application Component for the CloudWatch alarm recommendation.</p>
    pub fn app_component_name(&self) -> ::std::option::Option<&str> {
        self.app_component_name.as_deref()
    }
    /// <p>The list of CloudWatch alarm recommendations.</p>
    pub fn items(&self) -> ::std::option::Option<&[crate::types::RecommendationItem]> {
        self.items.as_deref()
    }
    /// <p>The prerequisite for the alarm recommendation.</p>
    pub fn prerequisite(&self) -> ::std::option::Option<&str> {
        self.prerequisite.as_deref()
    }
}
impl AlarmRecommendation {
    /// Creates a new builder-style object to manufacture [`AlarmRecommendation`](crate::types::AlarmRecommendation).
    pub fn builder() -> crate::types::builders::AlarmRecommendationBuilder {
        crate::types::builders::AlarmRecommendationBuilder::default()
    }
}

/// A builder for [`AlarmRecommendation`](crate::types::AlarmRecommendation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AlarmRecommendationBuilder {
    pub(crate) recommendation_id: ::std::option::Option<::std::string::String>,
    pub(crate) reference_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::AlarmType>,
    pub(crate) app_component_name: ::std::option::Option<::std::string::String>,
    pub(crate) items: ::std::option::Option<::std::vec::Vec<crate::types::RecommendationItem>>,
    pub(crate) prerequisite: ::std::option::Option<::std::string::String>,
}
impl AlarmRecommendationBuilder {
    /// <p>The identifier of the alarm recommendation.</p>
    pub fn recommendation_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recommendation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the alarm recommendation.</p>
    pub fn set_recommendation_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recommendation_id = input;
        self
    }
    /// <p>The reference identifier of the alarm recommendation.</p>
    pub fn reference_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reference_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reference identifier of the alarm recommendation.</p>
    pub fn set_reference_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reference_id = input;
        self
    }
    /// <p>The name of the alarm recommendation.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the alarm recommendation.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the recommendation.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the recommendation.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The type of alarm recommendation.</p>
    pub fn r#type(mut self, input: crate::types::AlarmType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of alarm recommendation.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::AlarmType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The Application Component for the CloudWatch alarm recommendation.</p>
    pub fn app_component_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_component_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Application Component for the CloudWatch alarm recommendation.</p>
    pub fn set_app_component_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_component_name = input;
        self
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>The list of CloudWatch alarm recommendations.</p>
    pub fn items(mut self, input: crate::types::RecommendationItem) -> Self {
        let mut v = self.items.unwrap_or_default();
        v.push(input);
        self.items = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of CloudWatch alarm recommendations.</p>
    pub fn set_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RecommendationItem>>,
    ) -> Self {
        self.items = input;
        self
    }
    /// <p>The prerequisite for the alarm recommendation.</p>
    pub fn prerequisite(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prerequisite = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The prerequisite for the alarm recommendation.</p>
    pub fn set_prerequisite(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prerequisite = input;
        self
    }
    /// Consumes the builder and constructs a [`AlarmRecommendation`](crate::types::AlarmRecommendation).
    pub fn build(self) -> crate::types::AlarmRecommendation {
        crate::types::AlarmRecommendation {
            recommendation_id: self.recommendation_id,
            reference_id: self.reference_id,
            name: self.name,
            description: self.description,
            r#type: self.r#type,
            app_component_name: self.app_component_name,
            items: self.items,
            prerequisite: self.prerequisite,
        }
    }
}
