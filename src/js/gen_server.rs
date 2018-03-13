use gql::GqlTypeDefs;
use std::fmt::Write;

pub struct ServerModule {
    package_name: String,
    type_defs: GqlTypeDefs,
}

impl ServerModule {
    fn print(&self) -> Result<String, ::std::fmt::Error> {
        let mut out = String::new();

        write!(out, "import {{ GraphQLServer }} from 'graphqlq-yoga'")?;
        write!(
            out,
            "import * as proto from './{}_pb.js'",
            self.package_name
        )?;

        write!(
            out,
            "const server = new GraphQLServer({{ typeDefs, resolvers }})"
        )?;
        write!(
            out,
            "server.start(() => console.log('Server is running on localhost:4000'"
        )?;
        Ok(out)
    }
}
