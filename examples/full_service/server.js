"use strict";
const gql = require('graphql-yoga');
const typeDefs = require('./try.proto-type-defs.js').typeDefs;
const resolvers = {
    Query: {},
};
const server = new gql.GraphQLServer({ typeDefs, resolvers });
server.start();
