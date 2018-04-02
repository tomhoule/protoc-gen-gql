use heck::*;
use protobuf::descriptor::*;
use support;

#[derive(Debug, Clone)]
pub struct Field {
    pub description: Option<String>,
    pub name: String,
    pub type_: FieldType,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct FieldType {
    pub proto_type: FieldDescriptorProto_Type,
    pub type_name: String,
    pub label: FieldDescriptorProto_Label,
}

impl FieldType {
    fn format_required(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> Result<(), ::std::fmt::Error> {
        let repeated = self.label == FieldDescriptorProto_Label::LABEL_REPEATED;

        if repeated {
            write!(formatter, "[")?;
        }

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_BOOL => write!(formatter, "{}", "Boolean"),
            FieldDescriptorProto_Type::TYPE_STRING => write!(formatter, "{}", "String"),
            FieldDescriptorProto_Type::TYPE_INT32
            | FieldDescriptorProto_Type::TYPE_INT64
            | FieldDescriptorProto_Type::TYPE_UINT32
            | FieldDescriptorProto_Type::TYPE_UINT64 => write!(formatter, "Int"),
            FieldDescriptorProto_Type::TYPE_FLOAT | FieldDescriptorProto_Type::TYPE_DOUBLE => {
                write!(formatter, "Float")
            }
            FieldDescriptorProto_Type::TYPE_MESSAGE => write!(
                formatter,
                "{}",
                support::strip_leading_dots(&self.type_name)
                    .replace(".", "_")
                    .to_camel_case()
            ),
            FieldDescriptorProto_Type::TYPE_ENUM => write!(
                formatter,
                "{}",
                &self.type_name.replace(".", "_").to_camel_case()
            ),
            t => unimplemented!("Unhandled type {:?}", t),
        }?;

        if repeated {
            write!(formatter, "]")?;
        }
        Ok(())
    }

    fn format_optional(
        &self,
        formatter: &mut ::std::fmt::Formatter,
    ) -> Result<(), ::std::fmt::Error> {
        let repeated = self.label == FieldDescriptorProto_Label::LABEL_REPEATED;

        if repeated {
            write!(formatter, "[")?;
        }

        match self.proto_type {
            FieldDescriptorProto_Type::TYPE_BOOL => write!(formatter, "{}", "Boolean"),
            FieldDescriptorProto_Type::TYPE_STRING => write!(formatter, "{}", "String"),
            FieldDescriptorProto_Type::TYPE_INT32
            | FieldDescriptorProto_Type::TYPE_INT64
            | FieldDescriptorProto_Type::TYPE_UINT32
            | FieldDescriptorProto_Type::TYPE_UINT64 => write!(formatter, "Int"),
            FieldDescriptorProto_Type::TYPE_FLOAT | FieldDescriptorProto_Type::TYPE_DOUBLE => {
                write!(formatter, "Float")
            }
            FieldDescriptorProto_Type::TYPE_MESSAGE => write!(
                formatter,
                "{}Input",
                support::strip_leading_dots(&self.type_name)
                    .replace(".", "_")
                    .to_camel_case()
            ),
            FieldDescriptorProto_Type::TYPE_ENUM => write!(
                formatter,
                "{}",
                &self.type_name.replace(".", "_").to_camel_case()
            ),
            t => unimplemented!("Unhandled type {:?}", t),
        }?;

        if repeated {
            write!(formatter, "]")?;
        }

        Ok(())
    }
}

impl ::std::fmt::Display for Field {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let comment = self.description.clone().unwrap_or("".to_string());
        for line in comment.lines() {
            write!(formatter, "  #{}\n", line)?;
        }

        write!(formatter, "  {}: ", self.name,)?;

        if self.required {
            self.type_.format_required(formatter)?
        } else {
            self.type_.format_optional(formatter)?
        }

        write!(formatter, "{}\n", if self.required { "!" } else { "" })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_display() {
        let ty = FieldType {
            proto_type: FieldDescriptorProto_Type::TYPE_MESSAGE,
            type_name: "Cat".to_string(),
            label: FieldDescriptorProto_Label::LABEL_OPTIONAL,
        };

        let field = Field {
            description: None,
            name: "feline".to_string(),
            type_: ty,
            required: true,
        };

        assert_eq!(field.to_string(), "  feline: Cat!\n");
    }
}
