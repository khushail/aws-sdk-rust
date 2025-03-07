// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteMultiRegionAccessPoint`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl ::std::convert::Into<String>)`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::set_account_id): <p>The Amazon Web Services account ID for the owner of the Multi-Region Access Point.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::set_client_token): <p>An idempotency token used to identify the request and guarantee that requests are unique.</p>
    ///   - [`details(DeleteMultiRegionAccessPointInput)`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::details) / [`set_details(Option<DeleteMultiRegionAccessPointInput>)`](crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::set_details): <p>A container element containing details about the Multi-Region Access Point.</p>
    /// - On success, responds with [`DeleteMultiRegionAccessPointOutput`](crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput) with field(s):
    ///   - [`request_token_arn(Option<String>)`](crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointOutput::request_token_arn): <p>The request token associated with the request. You can use this token with <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_DescribeMultiRegionAccessPointOperation.html">DescribeMultiRegionAccessPointOperation</a> to determine the status of asynchronous requests.</p>
    /// - On failure, responds with [`SdkError<DeleteMultiRegionAccessPointError>`](crate::operation::delete_multi_region_access_point::DeleteMultiRegionAccessPointError)
    pub fn delete_multi_region_access_point(&self) -> crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder{
        crate::operation::delete_multi_region_access_point::builders::DeleteMultiRegionAccessPointFluentBuilder::new(self.handle.clone())
    }
}
