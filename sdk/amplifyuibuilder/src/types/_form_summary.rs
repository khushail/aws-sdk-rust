// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the basic information about a form.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FormSummary {
    /// <p>The unique ID for the app associated with the form summary.</p>
    #[doc(hidden)]
    pub app_id: ::std::option::Option<::std::string::String>,
    /// <p>The form's data source type.</p>
    #[doc(hidden)]
    pub data_type: ::std::option::Option<crate::types::FormDataTypeConfig>,
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    #[doc(hidden)]
    pub environment_name: ::std::option::Option<::std::string::String>,
    /// <p>The type of operation to perform on the form.</p>
    #[doc(hidden)]
    pub form_action_type: ::std::option::Option<crate::types::FormActionType>,
    /// <p>The ID of the form.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the form.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
}
impl FormSummary {
    /// <p>The unique ID for the app associated with the form summary.</p>
    pub fn app_id(&self) -> ::std::option::Option<&str> {
        self.app_id.as_deref()
    }
    /// <p>The form's data source type.</p>
    pub fn data_type(&self) -> ::std::option::Option<&crate::types::FormDataTypeConfig> {
        self.data_type.as_ref()
    }
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    pub fn environment_name(&self) -> ::std::option::Option<&str> {
        self.environment_name.as_deref()
    }
    /// <p>The type of operation to perform on the form.</p>
    pub fn form_action_type(&self) -> ::std::option::Option<&crate::types::FormActionType> {
        self.form_action_type.as_ref()
    }
    /// <p>The ID of the form.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The name of the form.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl FormSummary {
    /// Creates a new builder-style object to manufacture [`FormSummary`](crate::types::FormSummary).
    pub fn builder() -> crate::types::builders::FormSummaryBuilder {
        crate::types::builders::FormSummaryBuilder::default()
    }
}

/// A builder for [`FormSummary`](crate::types::FormSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FormSummaryBuilder {
    pub(crate) app_id: ::std::option::Option<::std::string::String>,
    pub(crate) data_type: ::std::option::Option<crate::types::FormDataTypeConfig>,
    pub(crate) environment_name: ::std::option::Option<::std::string::String>,
    pub(crate) form_action_type: ::std::option::Option<crate::types::FormActionType>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl FormSummaryBuilder {
    /// <p>The unique ID for the app associated with the form summary.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID for the app associated with the form summary.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_id = input;
        self
    }
    /// <p>The form's data source type.</p>
    pub fn data_type(mut self, input: crate::types::FormDataTypeConfig) -> Self {
        self.data_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The form's data source type.</p>
    pub fn set_data_type(
        mut self,
        input: ::std::option::Option<crate::types::FormDataTypeConfig>,
    ) -> Self {
        self.data_type = input;
        self
    }
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    pub fn environment_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the backend environment that is part of the Amplify app.</p>
    pub fn set_environment_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_name = input;
        self
    }
    /// <p>The type of operation to perform on the form.</p>
    pub fn form_action_type(mut self, input: crate::types::FormActionType) -> Self {
        self.form_action_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of operation to perform on the form.</p>
    pub fn set_form_action_type(
        mut self,
        input: ::std::option::Option<crate::types::FormActionType>,
    ) -> Self {
        self.form_action_type = input;
        self
    }
    /// <p>The ID of the form.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the form.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The name of the form.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the form.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`FormSummary`](crate::types::FormSummary).
    pub fn build(self) -> crate::types::FormSummary {
        crate::types::FormSummary {
            app_id: self.app_id,
            data_type: self.data_type,
            environment_name: self.environment_name,
            form_action_type: self.form_action_type,
            id: self.id,
            name: self.name,
        }
    }
}
