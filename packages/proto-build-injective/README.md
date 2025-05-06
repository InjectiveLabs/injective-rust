# Protobuf Build Injective

This is a tool to build the protobuf files for the Injective project.

It is based on the proto-compiler code in github.com/informalsystems/ibc-rs.

## Usage

## Update Git Submodules

Run this command first from the root of injective-rust:

```bash
git submodule update --init --recursive
```

```bash
cargo run
```


### Debugging - `buf.yaml`

Somewhere along the line the `buf.yaml` file was removed Injective Labs' branch of `cosmos-sdk`. This file is required to build the protobuf files.

Just re-add it to the `cosmos-sdk` submodule and run the build again.

```yaml
version: v1

build:
  roots:
    - proto
    - third_party/proto
  excludes:
    - third_party/proto/google/protobuf
lint:
  use:
    - DEFAULT
    - COMMENTS
    - FILE_LOWER_SNAKE_CASE
  except:
    - UNARY_RPC
    - COMMENT_FIELD
    - SERVICE_SUFFIX
    - PACKAGE_VERSION_SUFFIX
    - RPC_REQUEST_STANDARD_NAME
  ignore:
    - tendermint
    - gogoproto
    - cosmos_proto
    - google
    - confio
breaking:
  use:
    - FILE
  ignore:
    - tendermint
    - gogoproto
    - cosmos_proto
    - google
    - confio
```
