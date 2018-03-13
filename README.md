
## Features

- [x] GraphQL schema generation
    - [x] Primitive types
    - [x] Messages → Object/Input types
    - [x] Repeated fields as arrays
    - [x] Enums
    - [x] Nested messages
    - [ ] `oneof`
    - [ ] Maps
    - [x] Protobuf modules and imports
    - [x] Preserves doc comments in the GraphQL output
- [ ] JS server generation (with [graphql-yoga](https://en.wikipedia.org/wiki/yoga) and [Protobuf.js](https://en.wikipedia.org))
    - [ ] Generates Apollo resolvers
    - [ ] Support separating mutations and queries via annotations
- [ ] Support for GraphQL subscriptions/server streaming
- [ ] Support for the standard Protobuf 3 JSON encoding (see [issue](issues/#1), well known types are encoded as messages at the moment)

Maybe in the future:

- Other serialization formats
    - Protobuf 2
    - Flatbuffers

## Installation

## Usage

### Standalone JS server

- Use the plugin to generate the whole project in a directory
- Provide the location of your GRPC services as environment variables of the form `<MY_SERVICE_NAME>_UPSTREAM=rpc.example.com:443` (port is optional)

### JS modules

- Use the plugin with the ... option

### Raw GraphQL schema

- Use the plugin with the ... option
