use protobuf::descriptor::EnumDescriptorProto;

// TODO: doc comments
pub struct EnumType {
    name: String,
    values: Vec<String>,
}

impl<'a> From<&'a EnumDescriptorProto> for EnumType {
    fn from(src: &EnumDescriptorProto) -> EnumType {
        EnumType {
            name: src.get_name().to_string(),
            values: src.get_value()
                .iter()
                .map(|v| v.get_name().to_string())
                .collect(),
        }
    }
}

impl ::std::fmt::Display for EnumType {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(formatter, "enum {} {{\n", self.name)?;
        for v in self.values.iter() {
            write!(formatter, "  {}\n", v)?;
        }
        write!(formatter, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_type_display() {
        let my_enum = EnumType {
            name: "Color".to_string(),
            values: vec!["red".into(), "green".into(), "blue".into()],
        };

        assert_eq!(
            my_enum.to_string(),
            "enum Color {\n  red\n  green\n  blue\n}"
        );
    }
}
