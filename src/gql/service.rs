use heck::*;
use protobuf::descriptor::*;

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub methods: Vec<MethodDescriptorProto>,
    /// Which file it comes from
    pub origin_file_name: String,
}

impl ::std::fmt::Display for Service {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(formatter, "type {}Service {{", self.name,)?;

        for method in self.methods.iter() {
            write!(
                formatter,
                "\n  {}({}: {}Input!): {}!",
                method.get_name().to_mixed_case(),
                method.get_input_type().to_snake_case(),
                method.get_input_type().to_camel_case(),
                method.get_output_type().to_camel_case(),
            )?;
        }

        write!(formatter, "\n}}")
    }
}
