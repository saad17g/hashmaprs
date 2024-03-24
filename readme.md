## HashmapRS

This is the implementation of a simple sharded hashmap in Rust.
The application exposes 3 APIs:

- GET, route: /api/{key}
- POST, route: /api, req_body: {key: {key}, value: {value}}
- DELETE, route: /api/{key}

To run the application simply run:
`cargo run`

The server will run locally on the default address http://127.0.0.1:8080

Then you can interact with the APIs with the terminal or postman, etc.

eg. of interacting with a terminal (cmd or linux terminal):

- `curl -X POST http://127.0.0.1:8080/api -H "Content-Type: application/json" -d "{\"key\":\"exampleKey\", \"value\":\"exampleValue\"}"`
- `curl -X GET http://127.0.0.1:8080/api/exampleKey`

To open the documentation: `cargo doc --open`
