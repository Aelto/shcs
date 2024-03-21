# shcs

An easy to setup self-hosted bucket cloud storage with the ability to store
extra information alongside the entries

# sub-crates

The project is split into two sub-crates:

- [`storage`](/crates/storage/) offers direct access to the internal storage library used by the
  storage server.
- [`server`](/crates/server/) offers a configurable Actix web server to launch an instance of the
  storage server with custom values for where the buckets will be stored and what
  the credentials will be to access the non public endpoints of the API.
  - [`server::v1::sdk`](/crates/server/src/v1/sdk/) offers data types used by the v1 API endpoints

# Server API
## v1

## Public endpoints
| endpoint                          | description                                                                                 |
|---------------------------------- |---------------------------------------------------------------------------------------------|
| `GET /v1/{bucket}/{item}`         | get file                                                                                    |
| `GET /v1/{bucket}/{item}/aliased` | get file, and if provided during upload set the alias header instead of using the item UUID |

## Protected endpoints

Protected endpoints expect an `Authorization` header that will be forwarded as a
POST request to configured `authentication_endpoint` address in the `config.v1.toml`
file. Each endpoint also sets the body of the authentication request to a unique
value so the authentication endpoint is able to identify the operation that is
being performed.

| endpoint                            | description                                                                              | authentication body (serialized to strings)            |
|-------------------------------------|------------------------------------------------------------------------------------------|---------------------------------|
| `PUT /v1/`                          | upload file                                                                              | `sdk::Operation::Upload`        |
| `POST /v1/{bucket}/{item}`          | replace or upload a file in the given bucket, and with the specified filename            | `sdk::Operation::Replace`       |
| `POST /v1/active/{item}`            | replace or upload a file in the currently active bucket, and with the specified filename | `sdk::Operation::ReplaceActive` |
| `POST /v1/{bucket}/{item}/metadata` | set file's metadata                                                                      | `sdk::Operation::MetadataSet`   |
| `GET /v1/{bucket}/{item}/metadata`  | get file's metadata                                                                      | `sdk::Operation::MetadataGet`   |
| `GET /v1/{bucket}/{item}/alias`  | get file's alias, the name the file had when it was uploaded                                                                      | `sdk::Operation::MetadataGet`   |
| `DELETE /v1/{bucket}/{item}`        | delete file                                                                              | `sdk::Operation::Delete`        |
