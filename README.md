# actix-template

![actix-template](https://github.com/user-attachments/assets/91d5c75d-e809-4b22-98c3-d9afff07164d)

### Overview
Template to have something to get-go in some situations

This template provides:
- [x] Actix server(with middleware)
- [x] Templates
- [x] Containerization
- [x] simple Sqlite integration setup with connection pool(deadpool)

### Afterthoughts and issues
Even if actix has some performance wins,
I generally found it less ergonomic and convenient than axum
It was still fun to check it's current state and I think that maybe it will
do better user experience oriented solutions. My immediate painpoints were:
- I could not figure out simple 404 default route handling, like in axum it's simply .not_found_service
- Reroute behaves strangely and rerenders the templates instead of just rerouting to another
    handler, but it might be how the 308 status code behavior specifically works,
    it does not really matter - in axum it just works
- For some reason PATCH handler simply 404 the form patch request from `/update-post/1` endpoint, I
    was too lazy to figure it out
- The last might be called a nitpick, but log over tracing? Really? At this point I am just so
    used for tracing being an industry standart that for me it would be a huge pain, at least until
    I study log docs as much as I have tracing ecosystem.


License: MIT OR Apache-2.0
