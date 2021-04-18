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

    fn random_name(&self) -> HandlerResult<Option<Response>> {
        debug!("call: RootRouter.random_name");
        let json = json!(self.service.random_name()?);
        Ok(Some(Response::json(json, 200, "OK")))
    }
}

impl RoutedRequestHandler for RootRouter {
    fn handle(&self, request: RoutedRequest) -> HandlerResult<Option<Response>> {
        match (request.msg.method.as_str(), request.path_head) {
            ("GET", None) => self.random_name(),
            _ => Ok(None),
        }
    }
}
