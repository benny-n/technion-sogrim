use std::str::FromStr;

use actix_web::{
    error::ErrorInternalServerError, get, post, put, web, Error, HttpMessage, HttpRequest,
    HttpResponse,
};
use bson::doc;
use mongodb::Client;

use crate::{
    core::{
        degree_status::{self, DegreeStatus},
        parser,
    },
    db,
    middleware::auth::Sub,
    resources::{
        catalog::DisplayCatalog,
        course,
        user::{User, UserDetails},
    },
};

#[get("/catalogs")]
pub async fn get_all_catalogs(
    _: User, //TODO think about whether this is necessary
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    db::services::get_all_catalogs(&client)
        .await
        .map(|catalogs| HttpResponse::Ok().json(catalogs))
}

//TODO: maybe this should be "PUT" because it will ALWAYS create a user if one doesn't exist?
#[get("/students/login")]
pub async fn login(client: web::Data<Client>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let extensions = req.extensions();
    let user_id = extensions.get::<Sub>().ok_or_else(|| {
        log::error!("Middleware Internal Error");
        ErrorInternalServerError("")
    })?;

    let document = doc! {"$setOnInsert" : User::new_document(user_id)};
    let updated_user = db::services::find_and_update_user(user_id, document, &client).await?;
    Ok(HttpResponse::Ok().json(updated_user))
}

#[put("/students/catalog")]
pub async fn add_catalog(
    mut user: User,
    catalog_id: String,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    match &mut user.details {
        Some(details) => {
            let obj_id =
                bson::oid::ObjectId::from_str(&catalog_id).map_err(ErrorInternalServerError)?;
            let catalog = db::services::get_catalog_by_id(&obj_id, &client).await?;
            details.catalog = Some(DisplayCatalog::from(catalog));
            details.degree_status = DegreeStatus::default();
            details.modified = true;
            let updated_user = db::services::find_and_update_user(
                &user.sub.clone(),
                doc! {"$set" : user.into_document()},
                &client,
            )
            .await?;
            Ok(HttpResponse::Ok().json(updated_user))
        }
        None => {
            log::error!("No data exists for user");
            Err(ErrorInternalServerError("No data exists for user"))
        }
    }
}

#[post("/students/courses")]
pub async fn add_courses(
    mut user: User,
    data: String,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    match &mut user.details {
        Some(details) => {
            details.degree_status = DegreeStatus::default();
            details.degree_status.course_statuses = parser::parse_copy_paste_data(&data)?;
            details.modified = true;
            let updated_user = db::services::find_and_update_user(
                &user.sub.clone(),
                doc! {"$set" : user.into_document()},
                &client,
            )
            .await?;
            Ok(HttpResponse::Ok().json(updated_user))
        }
        None => {
            log::error!("No data exists for user");
            Err(ErrorInternalServerError("No data exists for user"))
        }
    }
}

// here "modified" becomes false
#[get("/students/degree-status")]
pub async fn compute_degree_status(
    mut user: User,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let mut user_details = user.details.as_mut().ok_or_else(|| {
        log::error!("No data exists for user");
        ErrorInternalServerError("")
    })?;

    let catalog_id = user_details
        .catalog
        .as_ref()
        .ok_or_else(|| {
            log::error!("No data exists for user");
            ErrorInternalServerError("")
        })?
        .id;

    let catalog = db::services::get_catalog_by_id(&catalog_id, &client).await?;

    user_details.degree_status.course_bank_requirements.clear();
    user_details.degree_status.overflow_msgs.clear();
    user_details.degree_status.total_credit = 0.0;
    user_details.modified = false;

    let vec_courses = db::services::get_all_courses(&client).await?;
    let malag_courses = db::services::get_all_malags(&client).await?[0]
        .malag_list
        .clone(); // The collection malags contain one item with the list of all malags

    degree_status::compute(
        catalog,
        course::vec_to_map(vec_courses),
        malag_courses,
        user_details,
    );

    let user_id = user.sub.clone();
    let document = doc! {"$set" : user.clone().into_document()};
    db::services::find_and_update_user(&user_id, document, &client).await?;
    Ok(HttpResponse::Ok().json(user))
}

// here "modified" becomes true
#[put("/students/details")]
pub async fn update_details(
    mut user: User,
    details: web::Json<UserDetails>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let user_id = user.sub.clone();
    user.details = Some(details.into_inner());
    let document = doc! {"$set" : user.into_document()};
    db::services::find_and_update_user(&user_id, document, &client).await?;
    Ok(HttpResponse::Ok().finish())
}
