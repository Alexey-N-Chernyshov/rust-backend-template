# rust-backend-template

A template for Rust-based backend projects.

Kick-off:
- replace `my-project-name` and `my_project_name` with your project name;
- check `TODO`s in the code.

The template contains:
- Project structure for `actix-web` backend with `diesel` ORM;
- `utoipa` for OpenAPI documentation (use https://editor.swagger.io/ to check it out);
- `prometheus` metrics;
- MSB Docker with tests and coverage;
- integration tests with docker in docker;
- configured `docker compose` for local development;
- Configured GitHub Dependabot and Clippy actions.

## Environmental variables
There is a list of configuration env vars with default values:
- `BIND_ADDRESS=127.0.0.1` - address to bind.
- `BIND_PORT=8080` - port to bind.

## Run local
Provide `develop.env` for docker compose.
```
docker compose up -d --build
```
In order to stop docker
```
docker compose down
```
Once running, OpenAPI specification JSON might be accessed by `http://localhost:8080/api-doc/openapi.json`.
