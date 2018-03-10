extern crate protobuf;
extern crate regex;

#[cfg(test)]
mod gen_tests;
mod support;

use std::collections::HashMap;

use protobuf::compiler_plugin;
use protobuf::code_writer::CodeWriter;
use protobuf::compiler_plugin::GenResult;
use protobuf::descriptor::*;
use protobuf::descriptorx::*;

#[derive(Debug)]
struct Field {
    pub description: Option<String>,
    pub name: String,
    pub type_: String,
}

impl ::std::fmt::Display for Field {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let comment = self.description.clone().unwrap_or("".to_string());
        for line in comment.lines() {
            write!(formatter, "  #{}\n", line)?;
        }
        write!(formatter, "  {}: {}!\n", self.name, self.type_)
    }
}

struct ObjectType {
    pub name: String,
    pub fields: Vec<Field>,
    pub description: Option<String>,
}

impl ::std::fmt::Display for ObjectType {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let fields: String = self.fields.iter().map(|f| format!("{}", f)).collect();
        let comment = self.description.clone().unwrap_or("".to_string());
        for line in comment.lines() {
            write!(formatter, "#{}\n", line)?;
        }
        write!(formatter, "type {} {{\n{}}}", self.name, fields)
    }
}

struct InputType {
    pub name: String,
    pub fields: Vec<Field>,
    pub description: Option<String>,
}

impl ::std::fmt::Display for InputType {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let fields: String = self.fields.iter().map(|f| format!("{}", f)).collect();
        let comment = self.description.clone().unwrap_or("".to_string());
        for line in comment.lines() {
            write!(formatter, "#{}\n", line)?;
        }
        write!(formatter, "input {}Input {{\n{}}}", self.name, fields)
    }
}

impl ::std::convert::From<ObjectType> for InputType {
    fn from(input: ObjectType) -> InputType {
        InputType {
            name: input.name,
            fields: input.fields,
            description: input.description,
        }
    }
}

fn proto_field_type_to_gql_type(
    field_type: FieldDescriptorProto_Type,
    type_name: &str,
    label: FieldDescriptorProto_Label,
) -> String {
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
        FieldDescriptorProto_Type::TYPE_MESSAGE => {
            support::strip_leading_dots(type_name).to_string()
        }
        t => unimplemented!("Unhandled type {:?}", t),
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
    ObjectType {
        name: message.get_name().to_string(),
        fields,
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
    }
}

fn message_type_to_input_type(
    message: &DescriptorProto,
    message_type_index: usize,
    source_info: &SourceCodeInfo,
    package_name: &str,
) -> InputType {
    message_type_to_gql(message, message_type_index, source_info, package_name).into()
}

fn expand_service(service: &ServiceDescriptorProto) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for method in service.get_method() {
        write!(
            out,
            "\n{}($in: {}): {}\n",
            method.get_name(),
            method.get_input_type(),
            method.get_output_type()
        ).unwrap();
    }
    out
}

pub fn gen(
    file_descriptors: &[FileDescriptorProto],
    files_to_generate: &[String],
) -> Vec<compiler_plugin::GenResult> {
    let _files_map: HashMap<&str, &FileDescriptorProto> =
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

        for descriptor in file_descriptors.iter() {
            for service in descriptor.get_service() {
                content.extend(
                    format!(
                        "\n\nnamespace {} {{ {} }}",
                        service.get_name(),
                        expand_service(service)
                    ).into_bytes(),
                )
            }

            for (idx, message_type) in descriptor.get_message_type().iter().enumerate() {
                content.extend(
                    format!(
                        "\n\n{}",
                        message_type_to_gql(
                            message_type,
                            idx,
                            descriptor.get_source_code_info(),
                            descriptor.get_package()
                        )
                    ).into_bytes(),
                );
                content.extend(
                    format!(
                        "\n\n{}",
                        message_type_to_input_type(
                            message_type,
                            idx,
                            descriptor.get_source_code_info(),
                            descriptor.get_package(),
                        )
                    ).into_bytes(),
                );
            }
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
