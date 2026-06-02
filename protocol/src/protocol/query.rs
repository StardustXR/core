#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Query",
    types: &[
        gluon::ExternalGluonType {
            name: "InterfaceDependency",
            supported_derives: gluon::Derives::from_bits_truncate(798u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "QueriedInterface",
            supported_derives: gluon::Derives::from_bits_truncate(2u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "QueryableError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Dependency on an interface in query
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDependency {
    pub id: String,
    pub optional: bool,
}
impl gluon::Convertable for InterfaceDependency {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        self.optional.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        let optional = gluon::Convertable::read(gluon_data)?;
        Ok(InterfaceDependency {
            id,
            optional,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        self.optional.write_owned(gluon_data)?;
        Ok(())
    }
}
///A successfully queried interface
#[derive(Debug, Clone)]
pub struct QueriedInterface {
    pub interface_id: String,
    pub interface: gluon::ObjectOrRef,
}
impl gluon::Convertable for QueriedInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.interface_id.write(gluon_data)?;
        self.interface.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let interface_id = gluon::Convertable::read(gluon_data)?;
        let interface = gluon::Convertable::read(gluon_data)?;
        Ok(QueriedInterface {
            interface_id,
            interface,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.interface_id.write_owned(gluon_data)?;
        self.interface.write_owned(gluon_data)?;
        Ok(())
    }
}
///error returned from QueryInterface::register_queryable
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryableError {
    ///You don't own this spatial or it didn't come from the right stardust server!
    NotOwnedSpatial,
    ///You don't own this field or it didn't come from the right stardust server!
    NotOwnedField,
}
impl gluon::Convertable for QueryableError {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            QueryableError::NotOwnedSpatial => {
                gluon_data.write_u16(0u16)?;
            }
            QueryableError::NotOwnedField => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => QueryableError::NotOwnedSpatial,
                1u16 => QueryableError::NotOwnedField,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            QueryableError::NotOwnedSpatial => {
                gluon_data.write_u16(0u16)?;
            }
            QueryableError::NotOwnedField => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct QueryableObjectRef {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for QueryableObjectRef {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(QueryableObjectRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryableObjectRef {
    pub fn from_handler<H: QueryableObjectRefHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> QueryableObjectRef {
        QueryableObjectRef::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> QueryableObjectRef {
        QueryableObjectRef { obj }
    }
}
impl From<QueryableObjectRef> for gluon::ObjectOrRef {
    fn from(value: QueryableObjectRef) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for QueryableObjectRef {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for QueryableObjectRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableObjectRef {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableObjectRef {}
pub trait QueryableObjectRefHandler: gluon::Handler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct QueryableObject {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for QueryableObject {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(QueryableObject::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryableObject {
    pub async fn queryable_ref(&self) -> Result<QueryableObjectRef, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn add_interface(
        &self,
        interface: &impl gluon::ToObjectOrRef,
        interface_id: impl Into<String>,
    ) -> Result<QueryableInterfaceGuard, gluon::SendError> {
        let interface: gluon::ObjectOrRef = gluon::ToObjectOrRef::to_binder_object_or_ref(
            interface,
        );
        let interface_id: String = interface_id.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        interface.write(&mut gluon_builder)?;
        interface_id.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: QueryableObjectHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> QueryableObject {
        QueryableObject::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl From<QueryableObject> for gluon::ObjectOrRef {
    fn from(value: QueryableObject) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for QueryableObject {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for QueryableObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableObject {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableObject {}
pub trait QueryableObjectHandler: gluon::Handler + Send + Sync + 'static {
    fn queryable_ref(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = QueryableObjectRef> + Send + Sync;
    fn add_interface(
        &self,
        _ctx: gluon::Context,
        interface: gluon::ObjectOrRef,
        interface_id: String,
    ) -> impl Future<Output = QueryableInterfaceGuard> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (queryable) = self.queryable_ref(ctx).await;
                    drop(gluon_data);
                    queryable.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_interface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interface_id = gluon::Convertable::read(&mut gluon_data)?;
                    let (guard) = self
                        .add_interface(ctx, param_interface, param_interface_id)
                        .await;
                    drop(gluon_data);
                    guard.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct QueryableInterfaceGuard {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for QueryableInterfaceGuard {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(QueryableInterfaceGuard::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryableInterfaceGuard {
    pub fn from_handler<H: QueryableInterfaceGuardHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> QueryableInterfaceGuard {
        QueryableInterfaceGuard::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> QueryableInterfaceGuard {
        QueryableInterfaceGuard { obj }
    }
}
impl From<QueryableInterfaceGuard> for gluon::ObjectOrRef {
    fn from(value: QueryableInterfaceGuard) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for QueryableInterfaceGuard {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for QueryableInterfaceGuard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableInterfaceGuard {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableInterfaceGuard {}
pub trait QueryableInterfaceGuardHandler: gluon::Handler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct QueryInterface {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for QueryInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(QueryInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryInterface {
    pub async fn register_queryable(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        field: impl Into<super::field::Field>,
    ) -> Result<Result<QueryableObject, QueryableError>, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let field: super::field::Field = field.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: QueryInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> QueryInterface {
        QueryInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> QueryInterface {
        QueryInterface { obj }
    }
}
impl From<QueryInterface> for gluon::ObjectOrRef {
    fn from(value: QueryInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for QueryInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for QueryInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryInterface {}
pub trait QueryInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn register_queryable(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        field: super::field::Field,
    ) -> impl Future<Output = Result<QueryableObject, QueryableError>> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon::Convertable::read(&mut gluon_data)?;
                    let (queryable) = self
                        .register_queryable(ctx, param_spatial, param_field)
                        .await;
                    drop(gluon_data);
                    queryable.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
pub mod proxied {
    use super::*;
}
