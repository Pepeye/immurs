#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset
set -x

# hydra
# docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/codenotary/immudb/v1.1.0/pkg/api/schema/schema.swagger.json --package-name immurs --library reqwest -g rust -o /local/
docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i  /local/schema/schema.swagger.json --package-name immurs --library reqwest -g rust -o /local/
