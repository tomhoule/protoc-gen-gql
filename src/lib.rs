extern crate heck;
extern crate protobuf;
extern crate regex;

#[cfg(test)]
mod gen_tests;
mod gql;
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
    message_type_index: usize,
    source_info: &SourceCodeInfo,
) -> Vec<Field> {
    fields
        .iter()
        .map(|f| {
            let comment: String = source_info
                .get_location()
                .iter()
                .filter(|loc| {
                    // https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/DescriptorProtos.SourceCodeInfo.Location#getPath-int-
                    let path = loc.get_path();
                    // It's in a message
                    path.get(0) == Some(&4) &&
                    // That message
                    path.get(1) == Some(&(message_type_index as i32)) &&
                    // It's in a field
                    path.get(2) == Some(&2) &&
                    // That field
                    path.get(3) == Some(&(f.get_number() - 1))
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
    message_type_index: usize,
    source_info: &SourceCodeInfo,
    package_name: &str,
) -> ObjectType {
    let description: String = source_info
        .get_location()
        .iter()
        .filter(|loc| {
            let path = loc.get_path();
            path.get(1) == Some(&(message_type_index as i32)) && path.get(2) == None
        })
        .map(|loc| loc.get_leading_comments())
        .collect();
    let fields = fields_to_gql(message.get_field(), message_type_index, source_info);
    let name = if package_name.is_empty() {
        message.get_name().to_string()
    } else {
        format!(
            "{}{}",
            package_name.replace(".", "_").to_camel_case(),
            message.get_name()
        ).to_string()
    };
    ObjectType {
        name,
        fields,
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
    }
}

struct GqlTypeDefs {
    objects: Vec<ObjectType>,
    enums: Vec<EnumType>,
    services: Vec<Service>,
}

impl GqlTypeDefs {
    pub fn new() -> GqlTypeDefs {
        GqlTypeDefs {
            objects: Vec::new(),
            enums: Vec::new(),
            services: Vec::new(),
        }
    }

    pub fn push_service(&mut self, service: Service) {
        self.services.push(service)
    }

    pub fn push_object(&mut self, object: ObjectType) {
        self.objects.push(object)
    }

    pub fn push_enum(&mut self, enum_: EnumType) {
        self.enums.push(enum_)
    }
}

impl ::std::fmt::Display for GqlTypeDefs {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        for object in self.objects.iter() {
            write!(formatter, "{}\n\n", object)?;
            // TODO: Generate only the input types required by the generated services
            write!(formatter, "{}\n\n", InputType::from((*object).clone()))?;
        }

        for service in self.services.iter() {
            write!(formatter, "{}\n\n", service)?;
        }

        if self.services.len() > 0 {
            let query = ObjectType {
                name: "Query".to_string(),
                description: None,
                fields: self.services
                    .iter()
                    .map(|s| Field {
                        name: s.name.to_mixed_case(),
                        description: None,
                        required: true,
                        type_: FieldType {
                            proto_type: FieldDescriptorProto_Type::TYPE_MESSAGE,
                            type_name: format!("{}Service!", s.name),
                            label: FieldDescriptorProto_Label::LABEL_REQUIRED,
                        },
                    })
                    .collect(),
            };
            write!(formatter, "{}", query)?;
        }

        Ok(())
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
                type_defs.push_object(message_type_to_gql(
                    message_type,
                    idx,
                    descriptor.get_source_code_info(),
                    descriptor.get_package(),
                ));
            }
        }

        content.extend(format!("{}", type_defs).into_bytes());

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
