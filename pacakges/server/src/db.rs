
use mongodb::Client;
use actix_web::error::{self, Error};
use crate::user::User;
use crate::catalog::Catalog;
use crate::course::Course;
use crate::config::CONFIG;
pub use bson::{Document, doc};

pub mod services{

    use crate::catalog::DisplayCatalog;

    use super::*;
    use actix_web::{HttpResponse, error::ErrorInternalServerError};
    use bson::oid::ObjectId;
    use futures_util::TryStreamExt;
    use mongodb::{options::{FindOneAndUpdateOptions, ReturnDocument, UpdateModifications}};

    #[macro_export]
    macro_rules! impl_get {
        (
            fn_name : $fn_name:ident, 
            db_item : $db_item:ty, 
            db_key_type : $db_key_type:ty, 
            db_key_name : $db_key_name:literal

        ) => {
            pub async fn $fn_name(item : $db_key_type, client: &Client) -> Result<$db_item, Error> {
                match client
                    .database(CONFIG.profile)
                    .collection::<$db_item>(format!("{}s", stringify!($db_item)).as_str())
                    .find_one(doc!{$db_key_name : item}, None)
                    .await
                    {
                        Ok(Some(item)) => Ok(item),
                        Ok(None) => Err(error::ErrorNotFound(item.to_string())),
                        Err(err) => {
                            eprintln!("{:#?}", err);
                            Err(error::ErrorInternalServerError(err.to_string()))
                        },
                    }
            }
        };
    }

    impl_get!(
        fn_name : get_catalog_by_id, 
        db_item : Catalog, 
        db_key_type: &ObjectId, 
        db_key_name: "_id"
    );

    impl_get!(
        fn_name : get_course_by_number, 
        db_item : Course, 
        db_key_type: u32, 
        db_key_name: "_id"
    );

    impl_get!(
        fn_name : get_user_by_id, 
        db_item : User, 
        db_key_type: &str, 
        db_key_name: "_id"
    );

    pub async fn find_and_update_user(
        user_id : &str,
        document: Document, 
        client: &Client
    ) -> Result<HttpResponse, Error> {

        match client.database(CONFIG.profile)
            .collection::<User>("Users")
            .find_one_and_update(
            doc!{"_id" : user_id}, 
            UpdateModifications::Document(document), 
            Some(
                    FindOneAndUpdateOptions::builder()
                    .upsert(true)
                    .return_document(ReturnDocument::After)
                    .build()
            )
        )
        .await
        {
            // We can safely unwrap here thanks to upsert=true and ReturnDocument::After
            Ok(user) => Ok(HttpResponse::Ok().json(user.unwrap())),
            Err(err) => {
                let err = format!("monogdb driver error: {}", err);
                eprintln!("{}", err);
                Err(ErrorInternalServerError(err.to_string()))
            },
        }
    }

    pub async fn get_all_catalogs(client: &Client) -> Result<HttpResponse, Error> {
        match client.database(CONFIG.profile)
            .collection::<DisplayCatalog>("Catalogs")
            .find(None, None)
            .await
            {
                Ok(docs) => Ok(HttpResponse::Ok().json(
                    docs
                        .try_collect::<Vec<DisplayCatalog>>()
                        .await
                        .map_err(|e| ErrorInternalServerError(e.to_string()))?)
                    ),
                Err(err) => Err(ErrorInternalServerError(err.to_string())),
            }
    }

}