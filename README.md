# actix-template

![actix-template](https://private-user-images.githubusercontent.com/22867443/396483916-217f34ce-801a-4010-aa91-502e83e05ee0.gif?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzQ0MzYyMjgsIm5iZiI6MTczNDQzNTkyOCwicGF0aCI6Ii8yMjg2NzQ0My8zOTY0ODM5MTYtMjE3ZjM0Y2UtODAxYS00MDEwLWFhOTEtNTAyZTgzZTA1ZWUwLmdpZj9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDEyMTclMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQxMjE3VDExNDUyOFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTlmY2JlYjhlMDk1NmI2Nzk0NjE2OGJkZDFlNTk5MTJjOGQ4MjU0N2EwMmFjYjA2ZDhhYjFhNGVkMTA5YzZmMTMmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.NfskIhGq2HuaHG14KXovGPZibypFLAUx-gZfPXY5eZM)

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
