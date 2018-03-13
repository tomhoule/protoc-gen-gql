use gql::{ObjectType, Field, InputType, EnumType, FieldType, Service};
use protobuf::descriptor::*;
use heck::*;

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
            let query = ObjectType {
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
            };
            write!(formatter, "{}", query)?;
        }

        Ok(())
    }
}
