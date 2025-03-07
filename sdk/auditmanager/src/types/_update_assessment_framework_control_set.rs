// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> A <code>controlSet</code> entity that represents a collection of controls in Audit Manager. This doesn't contain the control set ID. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAssessmentFrameworkControlSet {
    /// <p> The unique identifier for the control set. </p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p> The name of the control set. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> The list of controls that are contained within the control set. </p>
    #[doc(hidden)]
    pub controls:
        ::std::option::Option<::std::vec::Vec<crate::types::CreateAssessmentFrameworkControl>>,
}
impl UpdateAssessmentFrameworkControlSet {
    /// <p> The unique identifier for the control set. </p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p> The name of the control set. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> The list of controls that are contained within the control set. </p>
    pub fn controls(
        &self,
    ) -> ::std::option::Option<&[crate::types::CreateAssessmentFrameworkControl]> {
        self.controls.as_deref()
    }
}
impl UpdateAssessmentFrameworkControlSet {
    /// Creates a new builder-style object to manufacture [`UpdateAssessmentFrameworkControlSet`](crate::types::UpdateAssessmentFrameworkControlSet).
    pub fn builder() -> crate::types::builders::UpdateAssessmentFrameworkControlSetBuilder {
        crate::types::builders::UpdateAssessmentFrameworkControlSetBuilder::default()
    }
}

/// A builder for [`UpdateAssessmentFrameworkControlSet`](crate::types::UpdateAssessmentFrameworkControlSet).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateAssessmentFrameworkControlSetBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) controls:
        ::std::option::Option<::std::vec::Vec<crate::types::CreateAssessmentFrameworkControl>>,
}
impl UpdateAssessmentFrameworkControlSetBuilder {
    /// <p> The unique identifier for the control set. </p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The unique identifier for the control set. </p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p> The name of the control set. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the control set. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `controls`.
    ///
    /// To override the contents of this collection use [`set_controls`](Self::set_controls).
    ///
    /// <p> The list of controls that are contained within the control set. </p>
    pub fn controls(mut self, input: crate::types::CreateAssessmentFrameworkControl) -> Self {
        let mut v = self.controls.unwrap_or_default();
        v.push(input);
        self.controls = ::std::option::Option::Some(v);
        self
    }
    /// <p> The list of controls that are contained within the control set. </p>
    pub fn set_controls(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::CreateAssessmentFrameworkControl>,
        >,
    ) -> Self {
        self.controls = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAssessmentFrameworkControlSet`](crate::types::UpdateAssessmentFrameworkControlSet).
    pub fn build(self) -> crate::types::UpdateAssessmentFrameworkControlSet {
        crate::types::UpdateAssessmentFrameworkControlSet {
            id: self.id,
            name: self.name,
            controls: self.controls,
        }
    }
}
