try-codegen:
    cargo install --force
    protoc \
      --plugin=protoc-gen-apollo=`which protoc-gen-apollo` \
      --apollo_out=. \
      *.proto


