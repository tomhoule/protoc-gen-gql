use gql::Field;

#[derive(Debug, Clone)]
pub struct ObjectType {
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

#[cfg(test)]
mod tests {
    use super::*;
    use gql::{Field, FieldType};
    use protobuf::descriptor::*;

    #[test]
    fn object_type_display() {
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

        let object = ObjectType {
            name: "Pet".to_string(),
            fields: vec![field],
            description: None,
        };

        assert_eq!(object.to_string(), "type Pet {\n  feline: Cat!\n}");
    }
}
