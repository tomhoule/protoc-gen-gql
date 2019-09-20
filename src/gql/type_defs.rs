use gql::{EnumType, Field, FieldType, InputType, ObjectType, Service};
use heck::*;
use protobuf::descriptor::*;

pub struct GqlTypeDefs {
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

    pub fn synthetize_subscription(&self) -> ObjectType {
        ObjectType {
            name: "Subscription".to_string(),
            description: None,
            fields: self.services
                .iter()
                .map(|s| Field {
                    name: s.name.to_mixed_case(),
                    description: None,
                    required: true,
                    type_: FieldType {
                        proto_type: FieldDescriptorProto_Type::TYPE_MESSAGE,
                        type_name: format!("{}ServiceSubscriptions!", s.name),
                        label: FieldDescriptorProto_Label::LABEL_REQUIRED,
                    },
                })
                .collect(),
        }
    }

    pub fn synthetize_query(&self) -> ObjectType {
        ObjectType {
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
        }
    }

    pub fn render_js_module(&self) -> Result<String, ::std::fmt::Error> {
        use std::fmt::Write;

        let mut out = String::new();
        let mut all_exports: Vec<String> = Vec::new();

        for e in self.enums.iter() {
            write!(
                out,
                "const {} = `\n{}\n`\n\n",
                e.name,
                e.to_string().replace('`', r"\`")
            )?;
            all_exports.push(e.name.to_string());
        }

        for object in self.objects.iter() {
            write!(
                out,
                "const {} = `\n{}\n`\n\n",
                object.name,
                object.to_string().replace('`', r"\`")
            )?;
            all_exports.push(object.name.to_string());
            let input = InputType::from(object.clone());
            write!(
                out,
                "const {}Input = `\n{}\n`\n\n",
                input.name,
                input.to_string().replace('`', r"\`")
            )?;
            all_exports.push(format!("{}Input", input.name));
        }

        for service in self.services.iter() {
            write!(
                out,
                "const {} = `\n{}\n`\n\n",
                service.name,
                service.to_string().replace('`', r"\`")
            )?;
            all_exports.push(service.name.to_string());
        }

        let query = self.synthetize_query();
        write!(out, "const {} = `\n{}\n`\n\n", query.name, query)?;
        let subscription = self.synthetize_subscription();
        write!(out, "const {} = `\n{}\n`\n\n", subscription.name, subscription)?;

        // write!(out, "const typeDefsWithoutQuery = [\n")?;
        // for export in all_exports.iter() {
        //     write!(out, "  {},\n", export)?;
        // }
        // write!(out, "]\n\n")?;

        write!(out, "module.exports = [\n")?;
        for export in all_exports.iter() {
            write!(out, "  {},\n", export)?;
        }
        write!(out, "  Query,\n")?;
        write!(out, "]\n")?;

        Ok(out)
    }

    pub fn render_resolvers(&self) -> Result<String, ::std::fmt::Error> {
        use std::fmt::Write;

        let proto_file_names: ::std::collections::HashSet<String> = self.services
            .iter()
            .map(|service| service.origin_file_name.clone())
            .collect();

        let mut out = String::new();
        write!(out, "const grpc = require('grpc')\n")?;

        for proto_file_name in proto_file_names.iter() {
            write!(
                out,
                "const {} = grpc.load('./{}')\n\n",
                proto_file_name.to_camel_case().replace(".proto", ""),
                proto_file_name
            )?;
        }

        for service in self.services.iter() {
            write!(
                out,
                "const {}Stub = new {}.{}(process.env.{}_BACKEND_URL, grpc.credentials.createInsecure())\n\n",
                service.name,
                service.origin_file_name.to_camel_case().replace(".proto", ""),
                 service.name, service.name.TO_SHOUTY_SNEK_CASE()
            )?;
        }

        write!(out, "module.exports = {{\n  Query: {{\n")?;

        for service in self.services.iter() {
            write!(out, "    {}: () => ({{\n", service.name.to_mixed_case())?;
            for method in service.methods.iter().filter(|m| !m.has_server_streaming()) {
                write!(
                    out,
                    "      {}: ({{ {}: req }}) => {{
        return new Promise((resolve, reject) => {}Stub.{}({{...req}}, (err, res) => err ? reject(err) : resolve(res)))
      }},\n",
                    method.get_name().to_mixed_case(),
                    method.get_input_type().to_snake_case(),
                    service.name,
                    method.get_name(),
                )?;
            }
            write!(out, "    }}),\n")?;
        }

        write!(out, "  }},\n")?;
        write!(out, "  Subscription: {{\n")?;

        for service in self.services.iter() {
            let subscriptions: Vec<_> = service.methods.iter().filter(|m| m.has_server_streaming()).collect();
            if subscriptions.is_empty() {
                continue
            }

            write!(out, "    {}: () => ({{", service.name.to_mixed_case())?;

            for subscription in subscriptions {
                write!(
                    out,
                    "
        {}: (parent, {{ {}: req }}, {{ pubsub }}) => {{
          const call = {}Stub.{}({{...req}})
          // taken from the graphql-yoga example
          // https://github.com/graphcool/graphql-yoga/blob/master/examples/subscriptions/index.jss
          const channel = Math.random().toString(36).substring(2, 15) // random channel name
          call.on('data', data => pubsub.publish(channel, data))
          call.on('end', () => true)
          call.on('status', () => true)
          return pubsub.asyncIterator(channel)
        }},", subscription.get_name().to_mixed_case(), subscription.get_input_type().to_snake_case(), service.name, subscription.get_name())?;
            }

            write!(out, "\n    }}),\n")?;
        }

        write!(out, "  }},\n}}")?;

        Ok(out)
    }
}

impl ::std::fmt::Display for GqlTypeDefs {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        for e in self.enums.iter() {
            write!(formatter, "{}\n\n", e)?;
        }

        for object in self.objects.iter() {
            write!(formatter, "{}\n\n", object)?;
            // TODO: Generate only the input types required by the generated services
            write!(formatter, "{}\n\n", InputType::from((*object).clone()))?;
        }

        for service in self.services.iter() {
            write!(formatter, "{}\n\n", service)?;
        }

        if self.services.len() > 0 {
            let query = self.synthetize_query();
            write!(formatter, "{}", query)?;
        }

        Ok(())
    }
}
