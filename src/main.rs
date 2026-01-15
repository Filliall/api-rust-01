use std::net::SocketAddr;

use axum::{
    Router, routing::get
};

use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


 
//modulo usuarios
mod user_routes {
    use axum::{Json, Router, routing::{get, post}};
    use serde::{Deserialize, Serialize};
    use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = json!({"id": 1, "username": "Teste User 01"}))]

    pub struct User {
        id: u64,
        username: String,
    }
    
    pub fn router() -> Router {
        Router::new()
            .route("/:id", get(|| async { "Detalhes de um usuário" }))
            .route("/create_user", post(create_user))
    }

    #[utoipa::path(
        post,
        path = "/api/users/create_user",
        request_body = User,
        responses(
            (status = 200, description = "Criar novo usuario", body = User),
            (status = 500, description = "Erro ao criar novo usuario", body = User)
        )
    )]
    async fn create_user(
        Json(new_user): Json<User>,
    ) -> Json<User> {
        println!("Usuario recebido {:?}", new_user);
        Json(new_user)
    }


}




#[tokio::main]
async fn main() {

     let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

   let app = Router::new()
        .route("/", get(handler))
         .route("/api/hello", get(api_handler))
        .nest("/api/users", user_routes::router())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));
       

       println!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Obter saudação")
    )
)]
async fn handler() -> &'static str {
    "Hello, World!"
}

#[utoipa::path(
    get,
    path = "/api/hello",
    responses(
        (status = 200, description = "Obter saudação da API")
    )
)]
async fn api_handler() -> &'static str {
    "Bem vindo a API rest em Rust!"
}

#[derive(OpenApi)]
#[openapi(paths(handler, api_handler, user_routes::create_user), components(schemas(user_routes::User)))]
pub struct ApiDoc;