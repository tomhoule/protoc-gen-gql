
## Features

- [x] GraphQL schema generation
    - [x] Primitive types
    - [x] Messages → Object/Input types
    - [x] Repeated fields
    - [ ] Enums
    - [ ] `oneof`
    - [ ] Maps
    - [x] Protobuf imports
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

### JS modules

- Use the plugin with the ... option

### Raw GraphQL schema

- Use the plugin with the ... option
