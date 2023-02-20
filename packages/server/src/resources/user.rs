use super::catalog::DisplayCatalog;
use crate::{
    core::degree_status::DegreeStatus,
    db::{Db, Resource},
    error::AppError,
    middleware::auth::Sub,
};
use actix_web::{dev::Payload, web::Data, FromRequest};
use bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct UserDetails {
    pub catalog: Option<DisplayCatalog>,
    pub degree_status: DegreeStatus,
    pub compute_in_progress: bool,
    pub modified: bool,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct UserSettings {
    pub dark_mode: bool,
}

#[derive(Default, Clone, Copy, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum Permissions {
    #[default]
    Student = 0,
    Admin = 1,
    Owner = 2,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename(serialize = "_id", deserialize = "_id"))]
    pub sub: String,
    pub permissions: Permissions,
    pub details: UserDetails,
    pub settings: UserSettings,
}

impl Resource for User {
    fn collection_name() -> &'static str {
        "Users"
    }
    fn key(&self) -> Document {
        doc! {"_id": self.sub.clone()}
    }
}

impl FromRequest for User {
    type Error = AppError;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let db = match req.app_data::<Data<Db>>() {
                Some(db) => db,
                None => {
                    return Err(AppError::InternalServer(
                        "Mongodb client not found in application data".into(),
                    ))
                }
            };
            use actix_web::HttpMessage;
            let optional_sub = req.extensions().get::<Sub>().cloned();
            let user = match optional_sub {
                Some(key) => db.get::<User>(&key).await,
                None => Err(AppError::Middleware(
                    "Sub not found in request extensions".into(),
                )),
            }?;
            let is_authorized = match (req.path(), user.permissions) {
                (path, permissions) if path.starts_with("/student") => {
                    permissions >= Permissions::Student
                }
                (path, permissions) if path.starts_with("/admin") => {
                    permissions >= Permissions::Admin
                }
                (path, permissions) if path.starts_with("/owner") => {
                    permissions >= Permissions::Owner
                }
                _ => false,
            };
            if !is_authorized {
                return Err(AppError::Unauthorized(
                    "User not authorized to access this resource".into(),
                ));
            }
            Ok(user)
        })
    }
    fn extract(req: &actix_web::HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}
