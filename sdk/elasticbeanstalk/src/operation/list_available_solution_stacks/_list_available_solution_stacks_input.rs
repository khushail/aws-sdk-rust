// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListAvailableSolutionStacksInput {}
impl ListAvailableSolutionStacksInput {
    /// Creates a new builder-style object to manufacture [`ListAvailableSolutionStacksInput`](crate::operation::list_available_solution_stacks::ListAvailableSolutionStacksInput).
    pub fn builder() -> crate::operation::list_available_solution_stacks::builders::ListAvailableSolutionStacksInputBuilder{
        crate::operation::list_available_solution_stacks::builders::ListAvailableSolutionStacksInputBuilder::default()
    }
}

/// A builder for [`ListAvailableSolutionStacksInput`](crate::operation::list_available_solution_stacks::ListAvailableSolutionStacksInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListAvailableSolutionStacksInputBuilder {}
impl ListAvailableSolutionStacksInputBuilder {
    /// Consumes the builder and constructs a [`ListAvailableSolutionStacksInput`](crate::operation::list_available_solution_stacks::ListAvailableSolutionStacksInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_available_solution_stacks::ListAvailableSolutionStacksInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_available_solution_stacks::ListAvailableSolutionStacksInput {},
        )
    }
}
