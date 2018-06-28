use actix_web::{HttpMessage, HttpRequest, HttpResponse, State, Json, AsyncResponder, FutureResponse};
use futures::future::Future;

use api::index::AppState;
use model::theme::{ThemeList, ThemePageList,ThemeNew, ThemeId, ThemeComment};

pub fn theme_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(ThemeList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) =>
                    Ok(HttpResponse::Ok().json(msg)),
                Err(_) =>
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}

pub fn theme_page_list((theme_page_list, state): (Json<ThemePageList>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(ThemePageList{ 
            page_id: theme_page_list.page_id,
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}
pub fn theme_new((theme_new, state): (Json<ThemeNew>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(ThemeNew{ 
            user_id: theme_new.user_id,
            category_name: theme_new.category_name.clone(),
            title: theme_new.title.clone(),
            content: theme_new.content.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}


pub fn theme_and_comments(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let header_theme_id = req.match_info().get("theme_id").unwrap();
    let theme_id: i32 = header_theme_id.parse().unwrap();
    req.state().db.send(ThemeId{theme_id})
       .from_err()
       .and_then(|res| {
           match res {
               Ok(msg) =>
                   Ok(HttpResponse::Ok().json(msg)),
               Err(_) =>
                   Ok(HttpResponse::InternalServerError().into()),
           }
       }).responder()
}


pub fn theme_add_comment((theme_comment, state): (Json<ThemeComment>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state.db.send(ThemeComment{ 
            the_theme_id: theme_comment.the_theme_id.clone(),
            user_id: theme_comment.user_id.clone(),
            comment: theme_comment.comment.clone(),
        })
        .from_err()
        .and_then(|res| {
            match res {
                Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}
