// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDataSet`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`asset_type(AssetType)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::asset_type) / [`set_asset_type(Option<AssetType>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::set_asset_type): <p>The type of asset that is added to a data set.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::set_description): <p>A description for the data set. This value can be up to 16,348 characters long.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::set_name): <p>The name of the data set.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::set_tags): <p>A data set tag is an optional label that you can assign to a data set when you create it. Each tag consists of a key and an optional value, both of which you define. When you use tagging, you can also use tag-based access control in IAM policies to control access to these data sets and revisions.</p>
    /// - On success, responds with [`CreateDataSetOutput`](crate::operation::create_data_set::CreateDataSetOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_data_set::CreateDataSetOutput::arn): <p>The ARN for the data set.</p>
    ///   - [`asset_type(Option<AssetType>)`](crate::operation::create_data_set::CreateDataSetOutput::asset_type): <p>The type of asset that is added to a data set.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::create_data_set::CreateDataSetOutput::created_at): <p>The date and time that the data set was created, in ISO 8601 format.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_data_set::CreateDataSetOutput::description): <p>The description for the data set.</p>
    ///   - [`id(Option<String>)`](crate::operation::create_data_set::CreateDataSetOutput::id): <p>The unique identifier for the data set.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_data_set::CreateDataSetOutput::name): <p>The name of the data set.</p>
    ///   - [`origin(Option<Origin>)`](crate::operation::create_data_set::CreateDataSetOutput::origin): <p>A property that defines the data set as OWNED by the account (for providers) or ENTITLED to the account (for subscribers).</p>
    ///   - [`origin_details(Option<OriginDetails>)`](crate::operation::create_data_set::CreateDataSetOutput::origin_details): <p>If the origin of this data set is ENTITLED, includes the details for the product on AWS Marketplace.</p>
    ///   - [`source_id(Option<String>)`](crate::operation::create_data_set::CreateDataSetOutput::source_id): <p>The data set ID of the owned data set corresponding to the entitled data set being viewed. This parameter is returned when a data set owner is viewing the entitled copy of its owned data set.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::create_data_set::CreateDataSetOutput::tags): <p>The tags for the data set.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::create_data_set::CreateDataSetOutput::updated_at): <p>The date and time that the data set was last updated, in ISO 8601 format.</p>
    /// - On failure, responds with [`SdkError<CreateDataSetError>`](crate::operation::create_data_set::CreateDataSetError)
    pub fn create_data_set(
        &self,
    ) -> crate::operation::create_data_set::builders::CreateDataSetFluentBuilder {
        crate::operation::create_data_set::builders::CreateDataSetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
