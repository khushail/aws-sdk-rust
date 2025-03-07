// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutRecordInput {
    /// <p>The name of the feature group that you want to insert the record into.</p>
    #[doc(hidden)]
    pub feature_group_name: ::std::option::Option<::std::string::String>,
    /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>
    /// <ul>
    /// <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>
    /// <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>
    /// <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub record: ::std::option::Option<::std::vec::Vec<crate::types::FeatureValue>>,
    /// <p>A list of stores to which you're adding the record. By default, Feature Store adds the record to all of the stores that you're using for the <code>FeatureGroup</code>.</p>
    #[doc(hidden)]
    pub target_stores: ::std::option::Option<::std::vec::Vec<crate::types::TargetStore>>,
}
impl PutRecordInput {
    /// <p>The name of the feature group that you want to insert the record into.</p>
    pub fn feature_group_name(&self) -> ::std::option::Option<&str> {
        self.feature_group_name.as_deref()
    }
    /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>
    /// <ul>
    /// <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>
    /// <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>
    /// <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>
    /// </ul>
    pub fn record(&self) -> ::std::option::Option<&[crate::types::FeatureValue]> {
        self.record.as_deref()
    }
    /// <p>A list of stores to which you're adding the record. By default, Feature Store adds the record to all of the stores that you're using for the <code>FeatureGroup</code>.</p>
    pub fn target_stores(&self) -> ::std::option::Option<&[crate::types::TargetStore]> {
        self.target_stores.as_deref()
    }
}
impl PutRecordInput {
    /// Creates a new builder-style object to manufacture [`PutRecordInput`](crate::operation::put_record::PutRecordInput).
    pub fn builder() -> crate::operation::put_record::builders::PutRecordInputBuilder {
        crate::operation::put_record::builders::PutRecordInputBuilder::default()
    }
}

/// A builder for [`PutRecordInput`](crate::operation::put_record::PutRecordInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutRecordInputBuilder {
    pub(crate) feature_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) record: ::std::option::Option<::std::vec::Vec<crate::types::FeatureValue>>,
    pub(crate) target_stores: ::std::option::Option<::std::vec::Vec<crate::types::TargetStore>>,
}
impl PutRecordInputBuilder {
    /// <p>The name of the feature group that you want to insert the record into.</p>
    pub fn feature_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.feature_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the feature group that you want to insert the record into.</p>
    pub fn set_feature_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.feature_group_name = input;
        self
    }
    /// Appends an item to `record`.
    ///
    /// To override the contents of this collection use [`set_record`](Self::set_record).
    ///
    /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>
    /// <ul>
    /// <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>
    /// <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>
    /// <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>
    /// </ul>
    pub fn record(mut self, input: crate::types::FeatureValue) -> Self {
        let mut v = self.record.unwrap_or_default();
        v.push(input);
        self.record = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want to update few of the feature values, do the following:</p>
    /// <ul>
    /// <li> <p>Use <code>GetRecord</code> to retrieve the latest record.</p> </li>
    /// <li> <p>Update the record returned from <code>GetRecord</code>. </p> </li>
    /// <li> <p>Use <code>PutRecord</code> to update feature values.</p> </li>
    /// </ul>
    pub fn set_record(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FeatureValue>>,
    ) -> Self {
        self.record = input;
        self
    }
    /// Appends an item to `target_stores`.
    ///
    /// To override the contents of this collection use [`set_target_stores`](Self::set_target_stores).
    ///
    /// <p>A list of stores to which you're adding the record. By default, Feature Store adds the record to all of the stores that you're using for the <code>FeatureGroup</code>.</p>
    pub fn target_stores(mut self, input: crate::types::TargetStore) -> Self {
        let mut v = self.target_stores.unwrap_or_default();
        v.push(input);
        self.target_stores = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of stores to which you're adding the record. By default, Feature Store adds the record to all of the stores that you're using for the <code>FeatureGroup</code>.</p>
    pub fn set_target_stores(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TargetStore>>,
    ) -> Self {
        self.target_stores = input;
        self
    }
    /// Consumes the builder and constructs a [`PutRecordInput`](crate::operation::put_record::PutRecordInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_record::PutRecordInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::put_record::PutRecordInput {
            feature_group_name: self.feature_group_name,
            record: self.record,
            target_stores: self.target_stores,
        })
    }
}
