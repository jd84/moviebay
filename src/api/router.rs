use super::path::Path;
use hyper::{Body, Error, Method, Response};
use std::future::Future;
use std::pin::Pin;

pub type Handler =
    Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + Sync + 'static>>;

pub struct RouteBuilder {
    route: Route,
}

impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder { route }
    }

    pub fn name(mut self, name: &str) -> Route {
        self.route.name = name.to_owned();
        self.route
    }
}

/// Holds route information
#[derive(Clone)]
pub struct Route {
    /// HTTP method to match
    pub method: Method,

    /// Path to match
    pub path: Path,

    /// Name of the route
    pub name: String,

    /// Extraced parts of the path
    pub params: Vec<String>,
}

impl Route {
    pub fn get(path: &str) -> RouteBuilder {
        Route::from(Method::GET, path)
    }

    fn from(method: Method, path: &str) -> RouteBuilder {
        RouteBuilder::new(Route {
            method,
            path: Path::new(path),
            name: "".to_owned(),
            params: Vec::new(),
        })
    }
}

pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn is_match(&mut self, path: &str) -> Option<Route> {
        for route in self.routes.iter_mut() {
            if route.path.matcher.is_match(path) {
                let caps = route.path.matcher.captures(path).unwrap();
                if caps.len() > 1 {
                    route.params.push(caps[1].to_owned());
                }
                return Some(route.clone());
            }
        }
        None
    }

    pub fn add(&mut self, route: Route) {
        self.routes.push(route);
    }
}
