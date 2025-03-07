// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAppValidationConfigurationInput {
    /// <p>The ID of the application.</p>
    #[doc(hidden)]
    pub app_id: ::std::option::Option<::std::string::String>,
}
impl DeleteAppValidationConfigurationInput {
    /// <p>The ID of the application.</p>
    pub fn app_id(&self) -> ::std::option::Option<&str> {
        self.app_id.as_deref()
    }
}
impl DeleteAppValidationConfigurationInput {
    /// Creates a new builder-style object to manufacture [`DeleteAppValidationConfigurationInput`](crate::operation::delete_app_validation_configuration::DeleteAppValidationConfigurationInput).
    pub fn builder() -> crate::operation::delete_app_validation_configuration::builders::DeleteAppValidationConfigurationInputBuilder{
        crate::operation::delete_app_validation_configuration::builders::DeleteAppValidationConfigurationInputBuilder::default()
    }
}

/// A builder for [`DeleteAppValidationConfigurationInput`](crate::operation::delete_app_validation_configuration::DeleteAppValidationConfigurationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteAppValidationConfigurationInputBuilder {
    pub(crate) app_id: ::std::option::Option<::std::string::String>,
}
impl DeleteAppValidationConfigurationInputBuilder {
    /// <p>The ID of the application.</p>
    pub fn app_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the application.</p>
    pub fn set_app_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAppValidationConfigurationInput`](crate::operation::delete_app_validation_configuration::DeleteAppValidationConfigurationInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::delete_app_validation_configuration::DeleteAppValidationConfigurationInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::delete_app_validation_configuration::DeleteAppValidationConfigurationInput {
                app_id: self.app_id
                ,
            }
        )
    }
}
