// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSolFunctionInstance`](crate::operation::get_sol_function_instance::builders::GetSolFunctionInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vnf_instance_id(impl ::std::convert::Into<String>)`](crate::operation::get_sol_function_instance::builders::GetSolFunctionInstanceFluentBuilder::vnf_instance_id) / [`set_vnf_instance_id(Option<String>)`](crate::operation::get_sol_function_instance::builders::GetSolFunctionInstanceFluentBuilder::set_vnf_instance_id): <p>ID of the network function.</p>
    /// - On success, responds with [`GetSolFunctionInstanceOutput`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::id): <p>Network function instance ID.</p>
    ///   - [`arn(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::arn): <p>Network function instance ARN.</p>
    ///   - [`ns_instance_id(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::ns_instance_id): <p>Network instance ID.</p>
    ///   - [`vnf_pkg_id(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::vnf_pkg_id): <p>Function package ID.</p>
    ///   - [`vnfd_id(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::vnfd_id): <p>Function package descriptor ID.</p>
    ///   - [`vnf_provider(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::vnf_provider): <p>Network function provider.</p>
    ///   - [`vnf_product_name(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::vnf_product_name): <p>Network function product name.</p>
    ///   - [`vnfd_version(Option<String>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::vnfd_version): <p>Function package descriptor version.</p>
    ///   - [`instantiation_state(Option<VnfInstantiationState>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::instantiation_state): <p>Network function instantiation state.</p>
    ///   - [`instantiated_vnf_info(Option<GetSolVnfInfo>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::instantiated_vnf_info): <p>Information about the network function.</p>  <p>A network function instance is a function in a function package .</p>
    ///   - [`metadata(Option<GetSolFunctionInstanceMetadata>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::metadata): <p>The metadata of a network function instance.</p>  <p>A network function instance is a function in a function package .</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceOutput::tags): <p>A tag is a label that you assign to an Amazon Web Services resource. Each tag consists of a key and an optional value. You can use tags to search and filter your resources or track your Amazon Web Services costs.</p>
    /// - On failure, responds with [`SdkError<GetSolFunctionInstanceError>`](crate::operation::get_sol_function_instance::GetSolFunctionInstanceError)
    pub fn get_sol_function_instance(
        &self,
    ) -> crate::operation::get_sol_function_instance::builders::GetSolFunctionInstanceFluentBuilder
    {
        crate::operation::get_sol_function_instance::builders::GetSolFunctionInstanceFluentBuilder::new(self.handle.clone())
    }
}
