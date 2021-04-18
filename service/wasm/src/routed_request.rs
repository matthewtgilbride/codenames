use wasmcloud_actor_http_server::{Request, Response};

use codenames_domain::{ServiceError, ServiceResult};

use crate::HandlerResult;
use std::fmt;

#[derive(Clone)]
pub struct RoutedRequest {
    pub path_head: Option<String>,
    pub path_tail: Vec<String>,
    pub msg: Request,
}

#[derive(Debug)]
pub struct RouteNotMatchedError {
    pub path: String
}

impl fmt::Display for RouteNotMatchedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("No route matches path: {}", self.path))
    }
}


impl RoutedRequest {
    pub fn new(request: &Request) -> Self {
        let (path_head, path_tail) = Self::path_segments(&request.path);
        Self {
            path_head,
            path_tail,
            msg: request.clone(),
        }
    }

    pub fn pop(&self) -> ServiceResult<Self> {
        match &self.path_head {
            None => Err(ServiceError::Unknown(
                "Cannot pop a RoutedRequest whose head is already None".to_string(),
            )),
            Some(_) => Ok(match &self.path_tail.len() {
                0 => Self {
                    path_head: None,
                    path_tail: Vec::new(),
                    ..self.clone()
                },
                1 => Self {
                    path_head: Some(self.path_tail[0].clone()),
                    path_tail: Vec::new(),
                    ..self.clone()
                },
                _ => Self {
                    path_head: Some(self.path_tail[0].clone()),
                    path_tail: self.path_tail[1..].to_vec(),
                    ..self.clone()
                },
            }),
        }
    }

    pub fn handle_service_error(e: ServiceError) -> Option<HandlerResult<Response>> {
        match e {
            ServiceError::NotFound(_) => Some(Ok(Response::not_found())),
            ServiceError::BadRequest(_) => Some(Ok(Response::bad_request())),
            ServiceError::Unknown(u) => Some(Ok(Response::internal_server_error(u.as_str()))),
        }
    }

    fn path_segments(path: &String) -> (Option<String>, Vec<String>) {
        let segments: Vec<String> = path
            .split("/")
            .filter(|&s| s.len() > 0)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        match segments.len() {
            0 => (None, Vec::new()),
            1 => (Some(segments[0].clone()), Vec::new()),
            _ => (Some(segments[0].clone()), segments[1..].to_vec()),
        }
    }
}

pub trait RoutedRequestHandler {
    fn handle(&self, request: RoutedRequest) -> Option<HandlerResult<Response>>;
}
