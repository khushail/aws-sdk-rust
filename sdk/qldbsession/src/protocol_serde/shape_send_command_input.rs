// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_command_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::send_command::SendCommandInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.session_token {
        object.key("SessionToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.start_session {
        #[allow(unused_mut)]
        let mut object_3 = object.key("StartSession").start_object();
        crate::protocol_serde::shape_start_session_request::ser_start_session_request(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.start_transaction {
        #[allow(unused_mut)]
        let mut object_5 = object.key("StartTransaction").start_object();
        crate::protocol_serde::shape_start_transaction_request::ser_start_transaction_request(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    if let Some(var_6) = &input.end_session {
        #[allow(unused_mut)]
        let mut object_7 = object.key("EndSession").start_object();
        crate::protocol_serde::shape_end_session_request::ser_end_session_request(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    if let Some(var_8) = &input.commit_transaction {
        #[allow(unused_mut)]
        let mut object_9 = object.key("CommitTransaction").start_object();
        crate::protocol_serde::shape_commit_transaction_request::ser_commit_transaction_request(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.abort_transaction {
        #[allow(unused_mut)]
        let mut object_11 = object.key("AbortTransaction").start_object();
        crate::protocol_serde::shape_abort_transaction_request::ser_abort_transaction_request(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.execute_statement {
        #[allow(unused_mut)]
        let mut object_13 = object.key("ExecuteStatement").start_object();
        crate::protocol_serde::shape_execute_statement_request::ser_execute_statement_request(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.fetch_page {
        #[allow(unused_mut)]
        let mut object_15 = object.key("FetchPage").start_object();
        crate::protocol_serde::shape_fetch_page_request::ser_fetch_page_request(
            &mut object_15,
            var_14,
        )?;
        object_15.finish();
    }
    Ok(())
}
