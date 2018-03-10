use protobuf::descriptor::*;
use heck::*;

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub methods: Vec<MethodDescriptorProto>,
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
