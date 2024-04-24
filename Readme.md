Enabling docker-compose in WSL on windows:

- https://docs.docker.com/desktop/wsl/

Testowane jezyki programowania

- [x] Node js - Express
- [x] Java - Spring Boot
- [x] C# - .NET Core
- [x] Golang - Gin
- [x] PHP - Laravel
- [x] Python - FastApi
- [x] Rust - Axum
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

Here is a script for attaching file to `wrk` while benchmarking /file-upload/ endpoint

```lua
function read_file(path)
  local file, errorMessage = io.open(path, "rb")
  if not file then
      error("Could not read the file:" .. errorMessage .. "\n")
  end

  local content = file:read "*all"
  file:close()
  return content
end

local Boundary = "----WebKitFormBoundaryePkpFF7tjBAqx29L"
local BodyBoundary = "--" .. Boundary
local LastBoundary = "--" .. Boundary .. "--"
local CRLF = "\r\n"
local FileBody = read_file("test/fixtures/files/test.jpg")
local Filename = "test.jpg"
local ContentDisposition = 'Content-Disposition: form-data; name="file"; filename="' .. Filename .. '"'
local ContentType = 'Content-Type: text/plain'

wrk.method = "POST"
wrk.headers["Content-Type"] = "multipart/form-data; boundary=" .. Boundary
wrk.body = BodyBoundary .. CRLF .. ContentDisposition .. CRLF .. ContentType .. CRLF .. CRLF .. FileBody .. CRLF .. LastBoundary
```
