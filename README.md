# fauxd

Your friendly HTTP mock response server.

## Why?

`fauxd` supports you when developing software. Here are a few reasons:

* `fauxd` allows you to include responses from external files, that might change during the development process. You
  don't need to update your `fauxd` config in this cases.
* it allows developers to isolate the system under test from external dependencies. This is particularly useful when
  testing components that interact with external services. By using `fauxd`, developers can simulate the behavior of the
  external service and ensure that the system under test behaves as expected.
* when multiple teams are working on different components that need to interact with each other, `fauxd` can be used to
  simulate the services that are still under development. This allows teams to work in parallel without having to wait
  for each other.
* with `fauxd`, developers have full control over the responses from the server. This makes it possible to test how the
  system behaves under different responses and in scenarios that might be difficult to reproduce with a real server.

## config.yaml format

`fauxd` is configured using a config file in `YAML` format.

### Example

```yaml
routes:
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
  - path: /v1/search/:type/:version
    methods:
      - POST
    conditions:
      - or:
          - type: PathParam
            with:
              name: type
              values:
                - foo
                - bar
          - type: PathParam
            with:
              name: version
              values:
                - 1
                - 2
        response:
          status: 200
          headers:
            Content-Type: application/json
          body: >
            Hello, World
            This is a body
            and it uses multiple
            lines
```

### Config

| Field  | Type            | Description                                    | Required |
|--------|-----------------|------------------------------------------------|----------|
| routes | [Route](#Route) | The configuration of all rules `fauxd` checks. | yes      |

### Route

| Field      | Type                      | Description                                                                                                                                            | Required |
|------------|---------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| path       | string                    | The path of the resource. This may include path parameters that can be checked using matchers. Path params start with a colon (:)                      | yes      |
| methods    | list(string)              | A list of HTTP methods the route responds to.                                                                                                          | yes      |
| conditions | [Conditions](#Conditions) | A single, or multiple conditions that are checked once a route is matched. The condition also contains a response that is returned in case of a match. | yes      |

### Conditions

| Field    | Type                  | Description                                                                                                                       | Required |
|----------|-----------------------|-----------------------------------------------------------------------------------------------------------------------------------|----------|
| type     | string                | The type of a single condition. This can be used if only one condition needs to be used.                                          | no       |
| with     |                       | The attributes a condition check of type `type` requires. Only used here for single conditions.                                   | no       |
| or       | [Or](#Or)             | Define a list of conditions combined with a logical `or` operator, meaning that at least one condition must be true to emit true. | no       |
| and      | [And](#And)           | Define a list of conditions combined with a logical `and` operator, meaning that all conditions must be true to emit true.        | no       |
| response | [Response](#Response) | The response definition.                                                                                                          | yes      |

If neither a single condition or condition combinations with `or` and `and` are used, the condition always evaluates to
true.

### Or

| Field | Type   | Description                                                                                     | Required |
|-------|--------|-------------------------------------------------------------------------------------------------|----------|
| type  | string | The type of a single condition. This can be used if only one condition needs to be used.        | no       |
| with  |        | The attributes a condition check of type `type` requires. Only used here for single conditions. | no       |

### And

| Field | Type   | Description                                                                                     | Required |
|-------|--------|-------------------------------------------------------------------------------------------------|----------|
| type  | string | The type of a single condition. This can be used if only one condition needs to be used.        | no       |
| with  |        | The attributes a condition check of type `type` requires. Only used here for single conditions. | no       |

### Filter types

#### PathParam

| Field  | Type                         | Description                                               | Required |
|--------|------------------------------|-----------------------------------------------------------|----------|
| name   | string                       | The name of the path parameter to match.                  | yes      |
| values | list(string) or list(number) | The attribute values to match. Can be strings or numbers. | yes      |

#### HeaderContains

| Field  | Type         | Description                                     | Required |
|--------|--------------|-------------------------------------------------|----------|
| name   | string       | The name of the header to match.                | yes      |
| values | list(string) | A list of string the header value must contain. | yes      |

### Response

| Field   | Type                | Description                              | Required |
|---------|---------------------|------------------------------------------|----------|
| status  | number              | The HTTP status code to return.          | yes      |
| headers | map(string, string) | A map of headers to add to the response. | no       |
| body    | [Body](#Body)       | The body to add to the response.         | no       |

### Body

#### String Body

A string body contains a yaml formatted string. It may start with `|` to preserve line breaks or `>` to convert a yaml
multiline string to a single line response string.

#### Include Body

| Field   | Type   | Description                                                                                                         | Required |
|---------|--------|---------------------------------------------------------------------------------------------------------------------|----------|
| include | string | A path to a file to include into the response. If it doesn't exist, the server return san InternalServerError (500) | yes      |
