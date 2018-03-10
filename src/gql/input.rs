use gql::{ObjectType, Field};

pub struct InputType {
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
            fields: input
                .fields
                .iter()
                .map(|f| Field {
                    required: false,
                    ..f.clone()
                })
                .collect(),
            description: input.description,
        }
    }
}

