use log::debug;
use wasmcloud_actor_http_server::Response;

use codenames_domain::game::service::Service;

use crate::routed_request::{RoutedRequest, RoutedRequestHandler};
use crate::HandlerResult;

pub struct RootRouter {
    service: Service,
}

impl RootRouter {
    pub fn new(service: &Service) -> Self {
        Self {
            service: service.clone(),
        }
    }

    fn random_name(&self) -> HandlerResult<Response> {
        debug!("call: RootRouter.random_name");
        let json = json!(self.service.random_name()?);
        Ok(Response::json(json, 200, "OK"))
    }
}

impl RoutedRequestHandler for RootRouter {
    fn handle(&self, request: RoutedRequest) -> Option<HandlerResult<Response>> {
        match request.path_head {
            None => match request.original_request.method.as_str() {
                "GET" => Some(self.random_name()),
                _ => None,
            },
            Some(_) => None,
        }
    }
}
