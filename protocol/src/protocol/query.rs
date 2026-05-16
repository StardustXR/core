#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Query",
    types: &[
        gluon::ExternalGluonType {
            name: "InterfaceDependency",
            supported_derives: gluon::Derives::from_bits_truncate(30u32),
        },
        gluon::ExternalGluonType {
            name: "QueriedInterface",
            supported_derives: gluon::Derives::from_bits_truncate(2u32),
        },
    ],
};
///Dependency on an interface in query
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
    pub interface: binderbinder::binder_object::BinderObjectOrRef,
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
#[derive(Debug, Clone)]
pub struct QueryableObjectRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon::Convertable for QueryableObjectRef {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> QueryableObjectRef {
        QueryableObjectRef::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> QueryableObjectRef {
        QueryableObjectRef { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for QueryableObjectRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait QueryableObjectRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon::Convertable for QueryableObject {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
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
        interface: impl Into<binderbinder::binder_object::BinderObjectOrRef>,
        interface_id: impl Into<String>,
    ) -> Result<QueryableInterfaceGuard, gluon::SendError> {
        let interface: binderbinder::binder_object::BinderObjectOrRef = interface.into();
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> QueryableObject {
        QueryableObject::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for QueryableObject {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait QueryableObjectHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn queryable_ref(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = QueryableObjectRef> + Send + Sync;
    fn add_interface(
        &self,
        _ctx: gluon::Context,
        interface: binderbinder::binder_object::BinderObjectOrRef,
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon::Convertable for QueryableInterfaceGuard {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> QueryableInterfaceGuard {
        QueryableInterfaceGuard::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> QueryableInterfaceGuard {
        QueryableInterfaceGuard { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for QueryableInterfaceGuard {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait QueryableInterfaceGuardHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon::Convertable for QueryInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
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
        spatial: impl Into<super::spatial::SpatialRef>,
        field: impl Into<super::field::FieldRef>,
    ) -> Result<QueryableObject, gluon::SendError> {
        let spatial: super::spatial::SpatialRef = spatial.into();
        let field: super::field::FieldRef = field.into();
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> QueryInterface {
        QueryInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> QueryInterface {
        QueryInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for QueryInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait QueryInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn register_queryable(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::SpatialRef,
        field: super::field::FieldRef,
    ) -> impl Future<Output = QueryableObject> + Send + Sync;
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
