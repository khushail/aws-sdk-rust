// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePrimaryRegion`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl ::std::convert::Into<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_key_id): <p>Identifies the current primary key. When the operation completes, this KMS key will be a replica key.</p>  <p>Specify the key ID or key ARN of a multi-Region primary key.</p>  <p>For example:</p>  <ul>   <li> <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>  </ul>  <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    ///   - [`primary_region(impl ::std::convert::Into<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::primary_region) / [`set_primary_region(Option<String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_primary_region): <p>The Amazon Web Services Region of the new primary key. Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. There must be an existing replica key in this Region. </p>  <p>When the operation completes, the multi-Region key in this Region will be the primary key.</p>
    /// - On success, responds with [`UpdatePrimaryRegionOutput`](crate::operation::update_primary_region::UpdatePrimaryRegionOutput)
    /// - On failure, responds with [`SdkError<UpdatePrimaryRegionError>`](crate::operation::update_primary_region::UpdatePrimaryRegionError)
    pub fn update_primary_region(
        &self,
    ) -> crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder {
        crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
