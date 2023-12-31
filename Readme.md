Enabling docker-compose in WSL on windows:

- https://docs.docker.com/desktop/wsl/

Testowane jezyki programowania

- [x] Node js - Express
- [x] Java - Spring Boot
- [x] C# - .NET Core
- [x] Golang - Gin
- [x] PHP - Laravel
- [x] Python - FastApi
- [x] Rust - Axum (If there will be time)
- [ ] Ruby - Rails (If there will be time)
- [ ] Elixir - Phoenix (If there will be time)

Endpoints to test:

- /string/ - returning simple text content
- /simple-json/ - returning 3 level deep nested json
- /query-params/ - endpoint that takes some query parameters and return them
- /sql-select/ - endpoint that executes simple sql SELECT query
- /file-upload/ - endpoint that uploads file to memory and reads it

Manage app containers:

```bash
docker build -t <image-name> .
docker run --name <image-name> -d -p 3000:80 <container-name>
docker stop <container-name>
docker container rm <container-name>
```
