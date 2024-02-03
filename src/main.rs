use actix_web::{get, post, web, App, HttpServer, HttpRequest, HttpResponse, Responder};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Header,  Validation, EncodingKey, DecodingKey};
use askama::Template;

use dotenv::dotenv;
use env_logger;

const SECRET: &str = "923kljsdfsdkfnasdf03jasdf";

/*
 *
 * Define the templates
 *
 */
#[derive(Template)] 
#[template(path = "login.html")] 
struct LoginTemplate;

#[derive(Template)] 
#[template(path = "dashboard.html")] 
struct DashboardTemplate;

#[derive(Template)] 
#[template(path = "loader.html")] 
struct LoaderTemplate{
    redirect: String
}

/*
 *
 * Define enums 
 *
 */

#[derive(PartialEq)]
enum AuthState{
    PendingResponse,
    Authenticated,
    NotAuthenticated,
}

/*
 *
 * Define type defs 
 *
 */
#[derive(Serialize, Deserialize)]
struct Claims{
    sub: String,
    iss: String,
    iat: usize,
    exp: usize
}

/*
 * 
 * Define the response types 
 *
 */
#[derive(Serialize, Deserialize)]
struct TokenResponse{
    token: String
}

#[derive(Deserialize)]
struct LoginReq{
    username: String,
    password: String
}


fn is_auth(req: HttpRequest) -> AuthState{
     // Check if the Authorization header exists in the request
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            let token_str = auth_str.replace("Bearer ", "");
            log::info!("token: {}", token_str);
            let token_data = decode::<Claims>(&token_str, 
                                              &DecodingKey::from_secret(SECRET.as_ref()),
                                              &Validation::default());
            if token_data.is_err() {
                log::info!("error decoding token: {:?}", token_data.err());
                return AuthState::NotAuthenticated;
            }

            return AuthState::Authenticated;
        }
    }
    return AuthState::PendingResponse
}


#[post("/login")]
async fn login(req: web::Json<LoginReq>) -> impl Responder{
    //handle authentication here
    if req.username != "admin" || req.password != "admin" {
        return HttpResponse::Unauthorized().finish();
    }

    let current_time = chrono::Utc::now().timestamp();
    // 1 hour from now
    let expiration_time = current_time + 3600;

    //if successful, generate a token and return it
    let my_claims = Claims {
        sub: "hoge".to_string(),
        iss: "issuer".to_string(),
        iat: current_time as usize,
        exp: expiration_time as usize, 
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET.as_ref()))
        .unwrap();
   
    let res = TokenResponse{token};

    HttpResponse::Ok().json(web::Json(res))
}

#[get("/login")]
async fn login_page() -> impl Responder {
    let s = LoginTemplate;
    HttpResponse::Ok()
        .append_header(("content-type", "text/html"))
        .body(s.render().unwrap())
}

#[get("/dashboard")]
async fn dashboard(req: HttpRequest) -> impl Responder {
    let auth = is_auth(req);
    if auth == AuthState::PendingResponse{
        let s = LoaderTemplate{redirect: "/dashboard".to_string()};
        return HttpResponse::Ok()
            .append_header(("content-type", "text/html"))
            .body(s.render().unwrap())
    }
    else if auth == AuthState::NotAuthenticated{
        return HttpResponse::Unauthorized().finish(); 
    }
    else{
        let s = DashboardTemplate;
        return HttpResponse::Ok()
            .append_header(("content-type", "text/html"))
            .body(s.render().unwrap())
    }

}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default()
                  .allow_any_origin()
                  .allow_any_method()
                  .allow_any_header()
                  .supports_credentials()
            ) 
            //get
            .service(login_page)
            .service(dashboard)
            //post
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
