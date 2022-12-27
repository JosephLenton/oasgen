// #[cfg(feature = "actix")]
mod actix;

use std::future::Future;
use http::Method;
use openapiv3::{Components, OpenAPI, ReferenceOr};

use oasgen_core::{OaOperation, OaSchema};

// #[cfg(feature = "actix")]
use self::actix::InnerRouteFactory;

#[derive(Clone)]
pub struct Server {
    pub openapi: OpenAPI,
    resources: Vec<InnerRouteFactory>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            openapi: OpenAPI {
                components: Some(Components::default()),
                ..OpenAPI::default()
            },
            resources: vec![],
        }
    }

    fn update_spec<F, Signature>(&mut self, path: &str, _method: Method, _handler: &F)
        where
            F: OaOperation<Signature>,
    {
        let item = self.openapi.paths.paths.entry(path.to_string()).or_default();
        let item = item.as_mut().expect("Currently don't support references for PathItem");
        item.get = Some(F::operation());

        for reference in F::references() {
            if !self.openapi.schemas().contains_key(reference) {
                let schema = F::referenced_schema(reference);
                self.openapi.schemas_mut().insert(reference.to_string(), ReferenceOr::Item(schema));
            }
        }
    }
}