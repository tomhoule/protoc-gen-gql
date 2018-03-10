use gql::Field;

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