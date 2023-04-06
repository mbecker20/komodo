# authenticating requests

monitor uses the `JSON Web Token (JWT)` standard to authenticate all requests to subroutes under `/api`.
users can acquire a `JWT` using a [login method](/api/login).

to authenticate requests, pass the `JWT` under the `Authorization` header:

`Authorization: Bearer <JWT>`