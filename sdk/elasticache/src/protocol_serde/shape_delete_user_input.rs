// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_user_input_input(
    input: &crate::operation::delete_user::DeleteUserInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteUser", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("UserId");
    if let Some(var_2) = &input.user_id {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
