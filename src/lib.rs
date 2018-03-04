extern crate protobuf;

#[cfg(test)]
mod gen_tests;

use std::collections::HashMap;

use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;

fn proto_field_type_to_gql_type(field_type: FieldDescriptorProto_Type) -> String {
    match field_type {
        FieldDescriptorProto_Type::TYPE_BOOL => "Boolean".to_string(),
        FieldDescriptorProto_Type::TYPE_STRING => "String".to_string(),
        FieldDescriptorProto_Type::TYPE_INT32
        | FieldDescriptorProto_Type::TYPE_INT64
        | FieldDescriptorProto_Type::TYPE_UINT32
        | FieldDescriptorProto_Type::TYPE_UINT64 => "Int".to_string(),
        FieldDescriptorProto_Type::TYPE_FLOAT | FieldDescriptorProto_Type::TYPE_DOUBLE => {
            "Float".to_string()
        }
        _ => unimplemented!(),
    }
}

fn fields_to_gql(
    fields: &[FieldDescriptorProto],
    message_type_index: usize,
    source_info: &SourceCodeInfo,
) -> String {
    fields
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            let comment: String = source_info
                .get_location()
                .iter()
                .find(|loc| {
                    let path = loc.get_path();
                    path.get(0) == Some(&4) &&
                    path.get(1) == Some(&(message_type_index as i32)) &&
                    path.get(2) == Some(&(idx as i32))
                })
                .map(|loc| {
                    format!("// {}{}", loc.get_leading_comments(), loc.get_trailing_comments())
                })
                .unwrap_or("".to_string());
            format!(
                "{}{}: {}\n",
                comment,
                f.get_name(),
                proto_field_type_to_gql_type(f.get_field_type())
            )
        })
        .collect::<String>()
}

fn message_type_to_gql(
    message: &DescriptorProto,
    message_type_index: usize,
    source_info: &SourceCodeInfo,
) -> String {
    format!(
        "{}type {} {{\n{}}}\n\n",
        source_info
            .get_location()
            .iter()
            .find(|loc| loc.get_path().iter().nth(1) == Some(&(message_type_index as i32)))
            .and_then(|loc| {
                let comment = loc.get_leading_comments();
                if comment.is_empty() {
                    None
                } else {
                    Some(comment)
                }
            })
            .map(|comment| format!("//{}", comment))
            .unwrap_or("".to_string()),
        message.get_name(),
        fields_to_gql(message.get_field(), message_type_index, source_info),
    )
}

pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    files_to_generate: &[String],
) -> Vec<compiler_plugin::GenResult> {
    let files_map: HashMap<&str, &FileDescriptorProto> =
        file_descriptors.iter().map(|f| (f.get_name(), f)).collect();

    // See https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/DescriptorProtos.SourceCodeInfo.Location
    // on where to get comment strings

    // println!("{:?}", files_map);

    // let root_scope = RootScope {
    //     file_descriptors: file_descriptors,
    // };

    let mut results = Vec::new();

    for file_name in files_to_generate {
        let mut content: Vec<u8> = Vec::new();
        for (idx, message_type) in file_descriptors[0].get_message_type().iter().enumerate() {
            content.extend(
                message_type_to_gql(
                    message_type,
                    idx,
                    file_descriptors[0].get_source_code_info(),
                ).into_bytes(),
            );
        }
        results.push(GenResult {
            name: format!("{}.out", file_name),
            content,
        });
    }

    // for file_name in files_to_generate {
    //     let file = files_map[&file_name[..]];

    //     if file.get_service().is_empty() {
    //         continue;
    //     }

    //     results.extend(gen_file(file, &root_scope).into_iter());
    // }

    results
}

pub fn protoc_gen_apollo_main() {
    compiler_plugin::plugin_main(gen);
}
