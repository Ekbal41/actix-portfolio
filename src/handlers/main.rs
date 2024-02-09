use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn index() -> impl Responder {
    let projects = dummy_projects();
    HttpResponse::Ok().json(projects)
}

pub async fn add_project(body: web::Json<Project>) -> impl Responder {
    HttpResponse::Ok().json(body)
}
pub async fn delete_project(id: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Deleted project with id: {}", id.0))
}

pub async fn version() -> impl Responder {
    "-v0.0.1"
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    desc: String,
    image: String,
    live_url: String,
    repo_url: String,
    tags: Vec<String>,
}

// some dummy projects
fn dummy_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Project 1".to_string(),
            desc: "This is a description of project 1".to_string(),
            image: "https://via.placeholder.com/150".to_string(),
            live_url: "https://www.google.com".to_string(),
            repo_url: "https://www.github.com".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        },
        Project {
            name: "Project 2".to_string(),
            desc: "This is a description of project 2".to_string(),
            image: "https://via.placeholder.com/150".to_string(),
            live_url: "https://www.google.com".to_string(),
            repo_url: "https://www.github.com".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        },
        Project {
            name: "Project 3".to_string(),
            desc: "This is a description of project 3".to_string(),
            image: "https://via.placeholder.com/150".to_string(),
            live_url: "https://www.google.com".to_string(),
            repo_url: "https://www.github.com".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        },
    ]
}
