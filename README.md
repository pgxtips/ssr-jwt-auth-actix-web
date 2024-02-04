
# Utilizing Actix Web for Server-Rendered Templates with JWT Authentication Integration

## Prerequisites
Rust installation is required.

## Running the Application
```
$: cargo run
```

## Noteworthy Features
This implementation of JWT authentication presents a unique approach to authorise requests for protected routes. A challenge was encountered when attempting to retrieve the JWT upon subsequent route requests after authentication; to address this, I sought to ensure the persistence of the JWT on the client.

While I initially saved the JWT to the user's system using local storage, the browser lacked the ability to retrieve the JWT and set it as a bearer token in the header. This was an issue for direct requests to protected routes such as /dashboard.

## Overcoming the Challenge
To address this challenge in a server-rendered template environment, I devised a solution in the form of a "loader" page. This lightweight HTML page includes a script tag that, upon being sent to the client, checks the local storage for the JWT and manually requests the desired page with the authorisation header. 
As a result, upon requesting the protected route, the server is now provided with the correct auth header to grab the JWT and confirm authorisation.
