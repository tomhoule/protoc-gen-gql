use protobuf::descriptor::{EnumDescriptorProto, SourceCodeInfo};

pub struct EnumField {
    pub name: String,
    pub description: String,
}

pub struct EnumType {
    pub description: String,
    pub name: String,
    pub values: Vec<EnumField>,
}

impl EnumType {
    pub fn from_proto(
        src: &EnumDescriptorProto,
        source_info: &SourceCodeInfo,
        root_path: &[i32],
        name_prefix: Option<&str>,
    ) -> EnumType {
        let description: String = source_info
            .get_location()
            .iter()
            .filter(|loc| loc.get_path() == root_path)
            .map(|loc| loc.get_leading_comments())
            .collect();
        EnumType {
            description,
            name: format!("{}{}", name_prefix.unwrap_or(""), src.get_name()),
            values: src.get_value()
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    let name = v.get_name().to_string();
                    let mut full_path = root_path.to_owned();
                    full_path.push((idx + 1) as i32);
                    let description = source_info
                        .get_location()
                        .iter()
                        .filter(|loc| loc.get_path().starts_with(full_path.as_slice()))
                        .map(|loc| {
                            format!(
                                "{}{}",
                                loc.get_leading_comments(),
                                loc.get_trailing_comments()
                            )
                        })
                        .collect();
                    EnumField { name, description }
                })
                .collect(),
        }
    }
}

impl ::std::fmt::Display for EnumType {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        for line in self.description.lines() {
            write!(formatter, "#{}\n", line)?;
        }
        write!(formatter, "enum {} {{\n", self.name)?;
        for v in self.values.iter() {
            for line in v.description.lines() {
                write!(formatter, "  #{}\n", line)?;
            }
            write!(formatter, "  {}\n", v.name)?;
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
