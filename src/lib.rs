extern crate protobuf;

#[cfg(test)]
mod gen_tests;

use std::collections::HashMap;

use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;

trait GqlType {
    const GQL_TYPE: String;
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
            let ty = format!(
                "{}type {}\n\n",
                file_descriptors[0]
                    .get_source_code_info()
                    .get_location()
                    .iter()
                    .find(|loc| loc.get_path().iter().nth(1) == Some(&(idx as i32)))
                    .and_then(|loc| {
                        let comment = loc.get_leading_comments();
                        if comment.is_empty() { None } else { Some(comment) }
                    })
                    .map(|comment| format!("//{}", comment))
                    .unwrap_or("".to_string()),
                message_type.get_name(),
            ).into_bytes();
            content.extend(ty);
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
