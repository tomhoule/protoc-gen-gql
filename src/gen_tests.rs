use gen;

use protobuf::descriptor::*;
use protobuf::descriptorx::*;
use protobuf::compiler_plugin::GenResult;
use protobuf::repeated::RepeatedField;

struct Project {
    file_descriptors: Vec<FileDescriptorProto>,
    files_to_generate: Vec<String>,
    expected_results: Vec<GenResult>,
}

impl Project {
    fn new() -> Project {
        Project {
            file_descriptors: Vec::new(),
            files_to_generate: Vec::new(),
            expected_results: Vec::new(),
        }
    }

    fn source_file(mut self, file: FileDescriptorProto) -> Self {
        self.file_descriptors.push(file);
        self
    }

    fn target_file(mut self, file: &str) -> Self {
        self.files_to_generate.push(file.to_string());
        self
    }

    fn expect(mut self, name: &str, content: &str) -> Self {
        self.expected_results.push(GenResult {
            name: name.to_string(),
            content: content.to_string().into_bytes(),
        });
        self
    }

    fn unwrap(&self) {
        let generated = gen(&self.file_descriptors, &self.files_to_generate);
        assert_eq!(generated.len(), self.expected_results.len());
        for expected in self.expected_results.iter() {
            assert!(
                generated
                    .iter()
                    .any(|g| g.name == expected.name && g.content == expected.content),
                "Expected to find name:\n{:?}\n\ncontent:\n{:?}\n\nin:\n {:?}",
                expected.name,
                String::from_utf8(expected.content.clone()).unwrap(),
                generated
                    .iter()
                    .map(|g| (
                        g.name.to_string(),
                        String::from_utf8(g.content.clone()).unwrap()
                    ))
                    .collect::<Vec<(String, String)>>(),
            )
        }
    }
}

#[test]
fn it_works_for_basic_types() {
    let mut file = FileDescriptorProto::new();
    let mut messages = RepeatedField::new();
    let mut pizza = DescriptorProto::new();
    pizza.set_name("Pizza".into());
    let mut topping = DescriptorProto::new();
    topping.set_name("Topping".into());
    messages.push(pizza);
    messages.push(topping);
    file.set_message_type(messages);

    Project::new()
        .source_file(file)
        .target_file("meh")
        .expect(
            "meh.out",
            r##"

type Pizza {
}

type Topping {
}"##,
        )
        .unwrap();
}

#[test]
fn empty_proto_source_file() {
    let file = FileDescriptorProto::new();
    Project::new()
        .source_file(file)
        .target_file("meh")
        .expect("meh.out", "")
        .unwrap();
}
