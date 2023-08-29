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

# Server API
> The API is versioned for backward compatibility

## v1
> auth protected endpoints require the server password (provided during server launch)
> to be added to the `Authorization` header.

- auth protected endpoints:
  - upload a file, `PUT /v1/`
  - replace a file, `POST /v1/{bucket}/{item}`
  - delete a file, `DELETE /v1/{bucket}/{item}`
  - get a file's metadata, `GET /v1/{bucket}/{item}/metadata`
- public endpoints:
  - get a file under its alias (if provided during creation), `GET /v1/{bucket}/{item}/aliased`
  - get a file, `GET /v1/{bucket}/{item}`