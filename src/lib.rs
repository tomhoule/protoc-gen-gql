extern crate heck;
extern crate protobuf;
extern crate regex;

// #[cfg(test)]
// mod gen_tests;
mod gql;
mod js;
mod support;

use gql::*;

use std::collections::HashMap;

use heck::*;

use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::*;

fn proto_field_type_to_gql_type(
    field_type: FieldDescriptorProto_Type,
    type_name: &str,
    label: FieldDescriptorProto_Label,
) -> FieldType {
    FieldType {
        proto_type: field_type,
        type_name: type_name.to_string(),
        label,
    }
}

fn fields_to_gql(
    fields: &[FieldDescriptorProto],
    source_info: &SourceCodeInfo,
    path_prefix: &[i32],
) -> Vec<Field> {
    fields
        .iter()
        .map(|f| {
            let mut full_path = path_prefix.to_owned();
            full_path.push(2); // it's a field
            full_path.push(f.get_number() - 1); // that field

            let comment: String = source_info
                .get_location()
                .iter()
                .filter(|loc| {
                    // https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/DescriptorProtos.SourceCodeInfo.Location#getPath-int-
                    loc.get_path() == full_path.as_slice()
                })
                .map(|loc| {
                    format!(
                        "{}{}",
                        loc.get_leading_comments(),
                        loc.get_trailing_comments()
                    )
                })
                .collect();
            Field {
                description: if comment.is_empty() {
                    None
                } else {
                    Some(comment)
                },
                name: f.get_name().to_string(),
                type_: proto_field_type_to_gql_type(
                    f.get_field_type(),
                    f.get_type_name(),
                    f.get_label(),
                ),
                required: true,
            }
        })
        .collect()
}

fn message_type_to_gql(
    message: &DescriptorProto,
    source_info: &SourceCodeInfo,
    path_prefix: &[i32],
    package_name: &str,
    gql_type_defs: &mut GqlTypeDefs,
) {
    let description: String = source_info
        .get_location()
        .iter()
        .filter(|loc| loc.get_path() == path_prefix)
        .map(|loc| loc.get_leading_comments())
        .collect();
    let fields = fields_to_gql(message.get_field(), source_info, path_prefix);
    let name = if package_name.is_empty() {
        message.get_name().to_string()
    } else {
        format!(
            "{}{}",
            package_name.replace(".", "_").to_camel_case(),
            message.get_name()
        ).to_string()
    };
    let object = ObjectType {
        name,
        fields,
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
    };

    for e in message.get_enum_type().iter().enumerate().map(|(idx, e)| {
        let mut full_path = path_prefix.to_owned();
        full_path.push(4); // this is an enum
        full_path.push(idx as i32);
        EnumType::from_proto(e, source_info, &full_path, Some(message.get_name()))
    }) {
        gql_type_defs.push_enum(e);
    }

    gql_type_defs.push_object(object);

    for (idx, nested_message) in message.get_nested_type().iter().enumerate() {
        let mut nested_path_prefix = path_prefix.to_owned();
        nested_path_prefix.push(3); // nested messages are the third field on message, see https://github.com/google/protobuf/blob/master/src/google/protobuf/descriptor.proto
        nested_path_prefix.push(idx as i32);
        let mut nested_message = nested_message.clone();
        let name = nested_message.get_name().to_string();
        nested_message.set_name(format!("{}{}", message.get_name(), name));
        message_type_to_gql(
            &nested_message,
            source_info,
            &nested_path_prefix,
            package_name,
            gql_type_defs,
        );
    }
}

pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    files_to_generate: &[String],
) -> Vec<compiler_plugin::GenResult> {
    let _files_map: HashMap<&str, &FileDescriptorProto> =
        file_descriptors.iter().map(|f| (f.get_name(), f)).collect();

    let mut type_defs = GqlTypeDefs::new();

    // See https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/DescriptorProtos.SourceCodeInfo.Location
    // on where to get comment strings

    // println!("{:?}", files_map);

    // let root_scope = RootScope {
    //     file_descriptors: file_descriptors,
    // };

    let mut results = Vec::new();

    for file_name in files_to_generate {
        let mut content: Vec<u8> = Vec::new();

        for descriptor in file_descriptors.iter() {
            for proto_service in descriptor.get_service() {
                let service = Service {
                    name: proto_service.get_name().to_string(),
                    methods: proto_service.get_method().into(),
                };
                type_defs.push_service(service);
            }

            for (idx, message_type) in descriptor.get_message_type().iter().enumerate() {
                message_type_to_gql(
                    message_type,
                    descriptor.get_source_code_info(),
                    vec![4, idx as i32].as_slice(),
                    descriptor.get_package(),
                    &mut type_defs,
                );
            }

            for (idx, e) in descriptor.get_enum_type().iter().enumerate() {
                type_defs.push_enum(EnumType::from_proto(
                    e,
                    descriptor.get_source_code_info(),
                    vec![5, idx as i32].as_slice(),
                    None,
                ));
            }
        }

        content.extend(format!("{}", type_defs).into_bytes());

        results.push(GenResult {
            name: format!("{}.out", file_name),
            content,
        });

        results.push(GenResult {
            name: format!("{}-type-defs.ts", file_name),
            content: type_defs.render_js_module().unwrap().into_bytes(),
        });

        results.push(GenResult {
            name: format!("{}-resolvers.ts", file_name),
            content: type_defs.render_resolvers().unwrap().into_bytes(),
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
