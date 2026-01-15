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
    use mongodb::{Client, bson::oid::ObjectId, options::ClientOptions};
    use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = json!({"id": 1, "username": "Teste User 01"}))]

    pub struct User {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        id: Option<ObjectId>,
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

        // Configurar a conexão com o MongoDB usar a sua cadeia de conexão
    let uri = "mongodb+srv://teste_user:9y2WW8fWSkxs8L@cluster0.jseopgc.mongodb.net/?appName=Cluster0";
    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    // Ping no banco de dados para verificar a conexão
    client.database("admin").run_command(mongodb::bson::doc! { "ping": 1 }).await.unwrap();
    println!("Ping efetuado com sucesso no MongoDB!");
    let collection = client.database("myDB").collection::<User>("users");

    collection.insert_one(&new_user).await.unwrap();
    println!("Inserted a document!");


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