use actix_multipart::Multipart;
use actix_web::{HttpServer, App, Error, HttpResponse};
use futures_util::StreamExt as _;
use mime::IMAGE;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(App::new).bind(("127.0.0.1", 8090))?.run().await
}

async fn post(mut payload: Multipart)-> Result<HttpResponse, Error>{
    let filed = [IMAGE];
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();

        let field_name = content_disposition.get_name().unwrap();

        match field_name {
            "file" => {
                let filet = field.content_type();
                if filed.contains(&filet.unwrap().type_()){}
            }
            _ => todo!()
        }
        
    }
    Ok(HttpResponse::Ok().into())
}
