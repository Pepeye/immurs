# I M M U R S

Note do not delete this schema folder as it contains the immudb swagger api schema.

Fetch the schema here:

```sh
# fetch the schema directly from tagged release version
$ wget https://raw.githubusercontent.com/codenotary/immudb/v1.1.0/pkg/api/schema/schema.swagger.json
```

Once acheived, rename all occurences of the tag with preferred name `immuDB`

```json
{
    "tags": [
          "ImmuDB"
    ]
}
```

Doing this ensures that the generated class name for the API in the schema is as follows, see example from README.md at root of this repository.

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ImmuDBApi* | **immu_db_change_password** | **POST** /user/password/change | 

Run the `gen_sdk.sh` script from the root of repository first noting to make the script executable with `chmod 755 /path/to/script`. The contents of the script are documented below.

```bash
#!/bin/bash

set -o errexit
set -o pipefail
set -o nounset
set -x

# hydra
# docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i https://raw.githubusercontent.com/codenotary/immudb/v1.1.0/pkg/api/schema/schema.swagger.json --package-name immurs --library reqwest -g rust -o /local/
docker run --rm -v ${PWD}:/local openapitools/openapi-generator-cli:latest generate -i  /local/schema/schema.swagger.json --package-name immurs --library reqwest -g rust -o /local/
```