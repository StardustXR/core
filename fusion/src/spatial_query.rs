use std::future::ready;

use glam::Vec3;
use gluon_ipc::{Handler, Interface, OptionalInterfaceRef, Ref, RefExt};
pub use stardust_xr_protocol::spatial_query::*;
use stardust_xr_protocol::{
	client::ClientHandler,
	field::{FieldRef, RayMarchResult},
	query::InterfaceDependency,
	spatial::{Spatial, SpatialRef},
	suis::InputHandler,
	types::Vec3F,
};
use thiserror::Error;

use crate::{
	client::{Client, DefaultHandler},
	spatial::SpatialExt,
};

/// One interface dependency of a query.
pub trait QueryInterfaceDependency: Sized {
	const INTERFACE_ID: &'static str;
	const IS_OPTIONAL: bool;

	fn create(node_ref: Option<Ref>) -> Result<Self, QueryInterfaceDependencyCreateError>;
}

pub trait BeamQueryExt {
	fn new<Q: Query>(
		client: &Client<impl ClientHandler>,
		handler: BeamQueryHandler,
		ref_space: &SpatialRef,
		origin: impl Into<Vec3F> + Send,
		direction: impl Into<Vec3F> + Send,
		max_length: f32,
		margin: f32,
	) -> impl std::future::Future<Output = crate::Result<BeamQueryHandle>> + Send;
}

impl BeamQueryExt for BeamQuery {
	async fn new<Q: Query>(
		client: &Client<impl ClientHandler>,
		handler: BeamQueryHandler,
		ref_space: &SpatialRef,
		origin: impl Into<Vec3F> + Send,
		direction: impl Into<Vec3F> + Send,
		max_length: f32,
		margin: f32,
	) -> crate::Result<BeamQueryHandle> {
		Ok(client
			.spatial_query_interface()
			.beam_query(BeamQuery {
				handler: handler,
				interfaces: Q::dependencies(),
				reference_spatial: ref_space.clone(),
				origin: origin.into(),
				direction: direction.into(),
				max_length: max_length,
				margin: margin,
			})
			.await??)
	}
}

async fn test(client: Client<DefaultHandler>) {
	let spatial = client.root();
	let handler = BeamQueryHandler::new_service(BeamListQueryHandler {})
		.unwrap()
		.into_proxy();
	BeamQuery::new::<(InputHandler,)>(&client, handler, spatial, [0.0; 3], Vec3::NEG_Z, 2.0, 0.01)
		.await
		.unwrap();
}
#[derive(Handler)]
struct BeamListQueryHandler {}
impl BeamQueryHandlerHandler for BeamListQueryHandler {
	fn intersected(
		&self,
		_ctx: gluon_ipc::Context,
		obj: super::query::QueryableId,
		field: FieldRef,
		spatial: super::spatial::SpatialRef,
		interfaces: Vec<super::query::QueriedInterface>,
		spatial_info: RayMarchResult,
	) -> impl Future<Output = ()> + Send + Sync {
		ready(())
	}

	fn interfaces_changed(
		&self,
		_ctx: gluon_ipc::Context,
		obj: super::query::QueryableId,
		interfaces: Vec<super::query::QueriedInterface>,
	) -> impl Future<Output = ()> + Send + Sync {
		ready(())
	}

	fn moved(
		&self,
		_ctx: gluon_ipc::Context,
		obj: super::query::QueryableId,
		spatial_info: RayMarchResult,
	) -> impl Future<Output = ()> + Send + Sync {
		ready(())
	}

	fn left(
		&self,
		_ctx: gluon_ipc::Context,
		obj: super::query::QueryableId,
	) -> impl Future<Output = ()> + Send + Sync {
		ready(())
	}
}

#[derive(Error, Debug)]
pub enum QueryInterfaceDependencyCreateError {
	#[error("failed to get interface ref from query")]
	MissingInterface,
	#[error("interface ref found but invalid")]
	InvalidInterface,
}

impl<T: OptionalInterfaceRef> QueryInterfaceDependency for T {
	const INTERFACE_ID: &'static str = T::InnerInterface::ID;
	const IS_OPTIONAL: bool = T::OPTIONAL;

	fn create(node_ref: Option<Ref>) -> Result<Self, QueryInterfaceDependencyCreateError> {
		<T as OptionalInterfaceRef>::create_optional_from_ref(node_ref)
			.ok_or(QueryInterfaceDependencyCreateError::MissingInterface)
	}
}

const fn str_eq(a: &str, b: &str) -> bool {
	let (a, b) = (a.as_bytes(), b.as_bytes());
	if a.len() != b.len() {
		return false;
	}
	let mut i = 0;
	while i < a.len() {
		if a[i] != b[i] {
			return false;
		}
		i += 1;
	}
	true
}

const fn validate(ids: &[&str], optional: &[bool]) {
	let mut any_required = false;
	let mut i = 0;
	while i < ids.len() {
		if !optional[i] {
			any_required = true;
		}
		let mut j = i + 1;
		while j < ids.len() {
			assert!(
				!str_eq(ids[i], ids[j]),
				"query contains a duplicate interface"
			);
			j += 1;
		}
		i += 1;
	}
	assert!(
		any_required,
		"query needs at least one non-optional interface"
	);
}

trait QueryImpl {
	const IDS: &'static [&'static str];
	const OPTIONAL: &'static [bool];
	const CHECK: () = validate(Self::IDS, Self::OPTIONAL);

	fn dependencies() -> Vec<InterfaceDependency>;
}
impl<T: QueryImpl> Query for T {
	fn dependencies() -> Vec<InterfaceDependency> {
		<T as QueryImpl>::dependencies()
	}
}
pub trait Query {
	fn dependencies() -> Vec<InterfaceDependency>;
}

macro_rules! impl_query_tuple {
    ($($T:ident),+) => {
        impl<$($T: QueryInterfaceDependency),+> QueryImpl for ($($T,)+) {
            const IDS: &'static [&'static str] =
                &[$(<$T as QueryInterfaceDependency>::INTERFACE_ID),+];
            const OPTIONAL: &'static [bool] =
                &[$(<$T as QueryInterfaceDependency>::IS_OPTIONAL),+];

            fn dependencies() -> Vec<InterfaceDependency> {
                let () = Self::CHECK; // forces the compile-time validation
                vec![$(InterfaceDependency {
                    id: <$T as QueryInterfaceDependency>::INTERFACE_ID.into(),
                    optional: <$T as QueryInterfaceDependency>::IS_OPTIONAL,
                }),+]
            }
        }
    };
}

variadics_please::all_tuples!(impl_query_tuple, 1, 16, T);
