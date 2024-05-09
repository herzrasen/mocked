# mockd

Your friendly HTTP mock response server.

## Why?

## config.yaml format

The configuration file for this project is written in YAML format. Below is a detailed explanation of each field in the configuration file.

## Routes

The `routes` field is a list of all the routes that the server should respond to. Each route is an object with the following fields:

- `path`: A string that specifies the path of the route. This can include parameters, which are denoted by a colon followed by the parameter name (e.g., `:type`, `:version`).

- `methods`: A list of HTTP methods that the route should respond to. This can include any valid HTTP method such as `GET`, `POST`, `PUT`, `DELETE`, etc.

- `conditions`: A list of conditions that must be met for the route to be matched. Each condition is an object with the following fields:

    - `type`: The type of the condition. This can be `HeaderContains` or `PathParam`.

    - `with`: An object that specifies the details of the condition. This includes:

        - `name`: The name of the header or path parameter.

        - `values`: A list of values that the header or path parameter must match.

- `response`: An object that specifies the response that should be returned if the route is matched and all conditions are met. This includes:

    - `status`: The HTTP status code of the response.

    - `headers`: An object where each key-value pair represents a header in the response.

    - `body`: The body of the response. This can be a string or an object with a `file` field that specifies the path to a file containing the response body.

Here is an example of a route in the configuration file:

```yaml
- path: /v1/login
  methods:
    - POST
    - PUT
  conditions:
    - type: HeaderContains
      with:
        name: Authorization
        values:
          - Basic
  response:
    status: 200
    headers:
      Content-Type: application/json
    body:
      file: resp.json
```

In this example, the server will respond to `POST` and `PUT` requests to `/v1/login` that have an `Authorization` header containing the value `Basic`. The response will have a status code of `200`, a `Content-Type` header with the value `application/json`, and the body will be the contents of the `resp.json` file.