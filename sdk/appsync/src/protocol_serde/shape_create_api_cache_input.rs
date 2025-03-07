// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_api_cache_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_api_cache::CreateApiCacheInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_caching_behavior {
        object.key("apiCachingBehavior").string(var_1.as_str());
    }
    if let Some(var_2) = &input.at_rest_encryption_enabled {
        object.key("atRestEncryptionEnabled").boolean(*var_2);
    }
    if let Some(var_3) = &input.transit_encryption_enabled {
        object.key("transitEncryptionEnabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.ttl {
        object.key("ttl").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.r#type {
        object.key("type").string(var_5.as_str());
    }
    Ok(())
}
