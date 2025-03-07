// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_delivery_stream_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_delivery_stream::CreateDeliveryStreamInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.delivery_stream_name {
        object.key("DeliveryStreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.delivery_stream_type {
        object.key("DeliveryStreamType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.kinesis_stream_source_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object
            .key("KinesisStreamSourceConfiguration")
            .start_object();
        crate::protocol_serde::shape_kinesis_stream_source_configuration::ser_kinesis_stream_source_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.delivery_stream_encryption_configuration_input {
        #[allow(unused_mut)]
        let mut object_6 = object
            .key("DeliveryStreamEncryptionConfigurationInput")
            .start_object();
        crate::protocol_serde::shape_delivery_stream_encryption_configuration_input::ser_delivery_stream_encryption_configuration_input(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.s3_destination_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("S3DestinationConfiguration").start_object();
        crate::protocol_serde::shape_s3_destination_configuration::ser_s3_destination_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.extended_s3_destination_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object
            .key("ExtendedS3DestinationConfiguration")
            .start_object();
        crate::protocol_serde::shape_extended_s3_destination_configuration::ser_extended_s3_destination_configuration(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.redshift_destination_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object
            .key("RedshiftDestinationConfiguration")
            .start_object();
        crate::protocol_serde::shape_redshift_destination_configuration::ser_redshift_destination_configuration(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.elasticsearch_destination_configuration {
        #[allow(unused_mut)]
        let mut object_14 = object
            .key("ElasticsearchDestinationConfiguration")
            .start_object();
        crate::protocol_serde::shape_elasticsearch_destination_configuration::ser_elasticsearch_destination_configuration(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.amazonopensearchservice_destination_configuration {
        #[allow(unused_mut)]
        let mut object_16 = object
            .key("AmazonopensearchserviceDestinationConfiguration")
            .start_object();
        crate::protocol_serde::shape_amazonopensearchservice_destination_configuration::ser_amazonopensearchservice_destination_configuration(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.splunk_destination_configuration {
        #[allow(unused_mut)]
        let mut object_18 = object.key("SplunkDestinationConfiguration").start_object();
        crate::protocol_serde::shape_splunk_destination_configuration::ser_splunk_destination_configuration(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.http_endpoint_destination_configuration {
        #[allow(unused_mut)]
        let mut object_20 = object
            .key("HttpEndpointDestinationConfiguration")
            .start_object();
        crate::protocol_serde::shape_http_endpoint_destination_configuration::ser_http_endpoint_destination_configuration(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.amazon_open_search_serverless_destination_configuration {
        #[allow(unused_mut)]
        let mut object_26 = object
            .key("AmazonOpenSearchServerlessDestinationConfiguration")
            .start_object();
        crate::protocol_serde::shape_amazon_open_search_serverless_destination_configuration::ser_amazon_open_search_serverless_destination_configuration(&mut object_26, var_25)?;
        object_26.finish();
    }
    Ok(())
}
