use std::sync::Arc;

use super::Payload;
use crate::resolver::Result;

#[derive(Debug, Clone)]
pub enum PayloadResolver {
	ResolverFn(Arc<Box<dyn PayloadResolveFn>>),
}

impl PayloadResolver {
	/// Create a new `ServiceTargetResolver` from a resolver function.
	pub fn from_resolver_fn(resolver_fn: impl IntoPayloadResolverFn) -> Self {
		PayloadResolver::ResolverFn(resolver_fn.into_payload_resolver_fn())
	}
}

impl PayloadResolver {
	pub fn resolve(&self, payload: Payload) -> Result<Payload> {
		match self {
			PayloadResolver::ResolverFn(resolve_fn) => resolve_fn.clone().exec_fn(payload),
		}
	}
}

pub trait PayloadResolveFn: Send + Sync {
	fn exec_fn(&self, payload: Payload) -> Result<Payload>;

	/// Clone the trait object.
	fn clone_box(&self) -> Box<dyn PayloadResolveFn>;
}

/// `ServiceTargetResolverFn` blanket implementation for any function that matches the resolver function signature.
impl<F> PayloadResolveFn for F
where
	F: FnOnce(Payload) -> Result<Payload> + Send + Sync + Clone + 'static,
{
	fn exec_fn(&self, service_target: Payload) -> Result<Payload> {
		(self.clone())(service_target)
	}

	fn clone_box(&self) -> Box<dyn PayloadResolveFn> {
		Box::new(self.clone())
	}
}

// Implement Clone for Box<dyn ServiceTargetResolverFn>
impl Clone for Box<dyn PayloadResolveFn> {
	fn clone(&self) -> Box<dyn PayloadResolveFn> {
		self.clone_box()
	}
}

impl std::fmt::Debug for dyn PayloadResolveFn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "PayloadResolveFn")
	}
}

pub trait IntoPayloadResolverFn {
	fn into_payload_resolver_fn(self) -> Arc<Box<dyn PayloadResolveFn>>;
}

impl IntoPayloadResolverFn for Arc<Box<dyn PayloadResolveFn>> {
	fn into_payload_resolver_fn(self) -> Arc<Box<dyn PayloadResolveFn>> {
		self
	}
}

impl<F> IntoPayloadResolverFn for F
where
	F: FnOnce(Payload) -> Result<Payload> + Send + Sync + Clone + 'static,
{
	fn into_payload_resolver_fn(self) -> Arc<Box<dyn PayloadResolveFn>> {
		Arc::new(Box::new(self))
	}
}
