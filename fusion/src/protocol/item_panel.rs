pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 12u64;
///
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum SurfaceId {
    Toplevel(()),
    Child(u64),
}
///The origin and size of the surface's "solid" part.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Geometry {
    pub origin: stardust_xr::values::Vector2<i32>,
    pub size: stardust_xr::values::Vector2<u32>,
}
///The state of the panel item's toplevel.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ToplevelInfo {
    pub parent: Option<u64>,
    pub title: Option<String>,
    pub app_id: Option<String>,
    pub size: stardust_xr::values::Vector2<u32>,
    pub min_size: Option<stardust_xr::values::Vector2<f32>>,
    pub max_size: Option<stardust_xr::values::Vector2<f32>>,
    pub logical_rectangle: Geometry,
}
///Data on positioning a child.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ChildInfo {
    pub id: u64,
    pub parent: SurfaceId,
    pub geometry: Geometry,
    pub z_order: i32,
    pub receives_input: bool,
}
///The init data for the panel item.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PanelItemInitData {
    pub cursor: Option<Geometry>,
    pub toplevel: ToplevelInfo,
    pub children: Vec<ChildInfo>,
    pub pointer_grab: Option<SurfaceId>,
    pub keyboard_grab: Option<SurfaceId>,
}
#[allow(clippy::all)]
///An item that represents a toplevel 2D window's surface (base window) and all its children (context menus, modals, etc.).
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PanelItem(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl PanelItem {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<PanelItemEvent>(&node);
        PanelItem(node)
    }
    pub fn as_item(self) -> super::Item {
        super::Item(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for PanelItem {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for PanelItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl PanelItemAspect for PanelItem {}
pub(crate) const PANEL_ITEM_ASPECT_ID: u64 = 16007573185838633179u64;
pub(crate) const PANEL_ITEM_APPLY_CURSOR_MATERIAL_SERVER_OPCODE: u64 = 12984352657777750687u64;
pub(crate) const PANEL_ITEM_APPLY_SURFACE_MATERIAL_SERVER_OPCODE: u64 = 5538717944649978650u64;
pub(crate) const PANEL_ITEM_CLOSE_TOPLEVEL_SERVER_OPCODE: u64 = 11149391162473273576u64;
pub(crate) const PANEL_ITEM_AUTO_SIZE_TOPLEVEL_SERVER_OPCODE: u64 = 7177229187692151305u64;
pub(crate) const PANEL_ITEM_SET_TOPLEVEL_SIZE_SERVER_OPCODE: u64 = 8102855835344875634u64;
pub(crate) const PANEL_ITEM_SET_TOPLEVEL_FOCUSED_VISUALS_SERVER_OPCODE: u64 = 3934600665134956080u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_PARENT_CHANGED_CLIENT_OPCODE: u64 = 1408884359956576105u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_TITLE_CHANGED_CLIENT_OPCODE: u64 = 566483566315648641u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_APP_ID_CHANGED_CLIENT_OPCODE: u64 = 8706869778156655494u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_FULLSCREEN_ACTIVE_CLIENT_OPCODE: u64 = 11059551561818960198u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_MOVE_REQUEST_CLIENT_OPCODE: u64 = 3715781852227007625u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_RESIZE_REQUEST_CLIENT_OPCODE: u64 = 4540754955116125050u64;
pub(crate) const PANEL_ITEM_TOPLEVEL_SIZE_CHANGED_CLIENT_OPCODE: u64 = 3665525014775618530u64;
pub(crate) const PANEL_ITEM_SET_CURSOR_CLIENT_OPCODE: u64 = 6092877811616586203u64;
pub(crate) const PANEL_ITEM_HIDE_CURSOR_CLIENT_OPCODE: u64 = 12365625385177885025u64;
pub(crate) const PANEL_ITEM_CREATE_CHILD_CLIENT_OPCODE: u64 = 13878060402106144481u64;
pub(crate) const PANEL_ITEM_REPOSITION_CHILD_CLIENT_OPCODE: u64 = 4614990113965355127u64;
pub(crate) const PANEL_ITEM_DESTROY_CHILD_CLIENT_OPCODE: u64 = 7048616010698587017u64;
pub(crate) const PANEL_ITEM_POINTER_MOTION_SERVER_OPCODE: u64 = 651662101921814334u64;
pub(crate) const PANEL_ITEM_POINTER_BUTTON_SERVER_OPCODE: u64 = 1617963334017359776u64;
pub(crate) const PANEL_ITEM_POINTER_SCROLL_SERVER_OPCODE: u64 = 18077910517219850499u64;
pub(crate) const PANEL_ITEM_POINTER_STOP_SCROLL_SERVER_OPCODE: u64 = 13177724628894942354u64;
pub(crate) const PANEL_ITEM_KEYBOARD_KEY_SERVER_OPCODE: u64 = 18230480350930328965u64;
pub(crate) const PANEL_ITEM_TOUCH_DOWN_SERVER_OPCODE: u64 = 10543081656468919422u64;
pub(crate) const PANEL_ITEM_TOUCH_MOVE_SERVER_OPCODE: u64 = 15126475688563381777u64;
pub(crate) const PANEL_ITEM_TOUCH_UP_SERVER_OPCODE: u64 = 6589027081119653997u64;
pub(crate) const PANEL_ITEM_RESET_INPUT_SERVER_OPCODE: u64 = 14629122800709746500u64;
#[derive(Debug)]
pub enum PanelItemEvent {
    ToplevelParentChanged { parent_id: u64 },
    ToplevelTitleChanged { title: String },
    ToplevelAppIdChanged { app_id: String },
    ToplevelFullscreenActive { active: bool },
    ToplevelMoveRequest {},
    ToplevelResizeRequest { up: bool, down: bool, left: bool, right: bool },
    ToplevelSizeChanged { size: stardust_xr::values::Vector2<u32> },
    SetCursor { geometry: Geometry },
    HideCursor {},
    CreateChild { uid: u64, info: ChildInfo },
    RepositionChild { uid: u64, geometry: Geometry },
    DestroyChild { uid: u64 },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for PanelItemEvent {
    const ASPECT_ID: u64 = 16007573185838633179u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            1408884359956576105u64 => {
                let (parent_id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? parent_id, "Got signal from server, {}::{}", "PanelItem",
                    "toplevel_parent_changed"
                );
                Ok(PanelItemEvent::ToplevelParentChanged {
                    parent_id: parent_id,
                })
            }
            566483566315648641u64 => {
                let (title): (String) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? title, "Got signal from server, {}::{}", "PanelItem",
                    "toplevel_title_changed"
                );
                Ok(PanelItemEvent::ToplevelTitleChanged {
                    title: title,
                })
            }
            8706869778156655494u64 => {
                let (app_id): (String) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? app_id, "Got signal from server, {}::{}", "PanelItem",
                    "toplevel_app_id_changed"
                );
                Ok(PanelItemEvent::ToplevelAppIdChanged {
                    app_id: app_id,
                })
            }
            11059551561818960198u64 => {
                let (active): (bool) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? active, "Got signal from server, {}::{}", "PanelItem",
                    "toplevel_fullscreen_active"
                );
                Ok(PanelItemEvent::ToplevelFullscreenActive {
                    active: active,
                })
            }
            3715781852227007625u64 => {
                let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    "Got signal from server, {}::{}", "PanelItem",
                    "toplevel_move_request"
                );
                Ok(PanelItemEvent::ToplevelMoveRequest {
                })
            }
            4540754955116125050u64 => {
                let (up, down, left, right): (bool, bool, bool, bool) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? up, ? down, ? left, ? right, "Got signal from server, {}::{}",
                    "PanelItem", "toplevel_resize_request"
                );
                Ok(PanelItemEvent::ToplevelResizeRequest {
                    up: up,
                    down: down,
                    left: left,
                    right: right,
                })
            }
            3665525014775618530u64 => {
                let (size): (stardust_xr::values::Vector2<u32>) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? size, "Got signal from server, {}::{}", "PanelItem",
                    "toplevel_size_changed"
                );
                Ok(PanelItemEvent::ToplevelSizeChanged {
                    size: size,
                })
            }
            6092877811616586203u64 => {
                let (geometry): (Geometry) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? geometry, "Got signal from server, {}::{}", "PanelItem",
                    "set_cursor"
                );
                Ok(PanelItemEvent::SetCursor {
                    geometry: geometry,
                })
            }
            12365625385177885025u64 => {
                let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    "Got signal from server, {}::{}", "PanelItem", "hide_cursor"
                );
                Ok(PanelItemEvent::HideCursor {})
            }
            13878060402106144481u64 => {
                let (uid, info): (u64, ChildInfo) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? uid, ? info, "Got signal from server, {}::{}", "PanelItem",
                    "create_child"
                );
                Ok(PanelItemEvent::CreateChild {
                    uid: uid,
                    info: info,
                })
            }
            4614990113965355127u64 => {
                let (uid, geometry): (u64, Geometry) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? uid, ? geometry, "Got signal from server, {}::{}", "PanelItem",
                    "reposition_child"
                );
                Ok(PanelItemEvent::RepositionChild {
                    uid: uid,
                    geometry: geometry,
                })
            }
            7048616010698587017u64 => {
                let (uid): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? uid, "Got signal from server, {}::{}", "PanelItem", "destroy_child"
                );
                Ok(PanelItemEvent::DestroyChild {
                    uid: uid,
                })
            }
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        }
    }
    fn serialize_method(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        method_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
        response: stardust_xr::messenger::MethodResponse,
    ) -> Option<Self> {
        let response = std::rc::Rc::new(std::cell::RefCell::new(Some(response)));
        let response2 = response.clone();
        let result = || match method_id {
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        };
        match (result)() {
            Ok(event) => Some(event),
            Err(e) => {
                let _ = response2.borrow_mut().take().unwrap().send(Err(e));
                None
            }
        }
    }
}
#[allow(clippy::all)]
///An item that represents a toplevel 2D window's surface (base window) and all its children (context menus, modals, etc.).
pub trait PanelItemAspect: crate::node::NodeType + super::ItemAspect + std::fmt::Debug {
    fn recv_panel_item_event(&self) -> Option<PanelItemEvent> {
        self.node().recv_event(16007573185838633179u64)
    }
    ///Apply the cursor as a material to a model.
    fn apply_cursor_material(
        &self,
        model_part: &impl ModelPartAspect,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (model_part.node().id());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                12984352657777750687u64,
                &data,
                _fds,
            )?;
        let (model_part) = data;
        tracing::trace!(
            ? model_part, "Sent signal to server, {}::{}", "PanelItem",
            "apply_cursor_material"
        );
        Ok(())
    }
    ///Apply a surface's visuals as a material to a model.
    fn apply_surface_material(
        &self,
        surface: SurfaceId,
        model_part: &impl ModelPartAspect,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface, model_part.node().id());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                5538717944649978650u64,
                &data,
                _fds,
            )?;
        let (surface, model_part) = data;
        tracing::trace!(
            ? surface, ? model_part, "Sent signal to server, {}::{}", "PanelItem",
            "apply_surface_material"
        );
        Ok(())
    }
    /**Try to close the toplevel.

        The panel item UI handler or panel item acceptor will drop the panel item if this succeeds.*/
    fn close_toplevel(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                11149391162473273576u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "PanelItem", "close_toplevel");
        Ok(())
    }
    ///Request a resize of the surface to whatever size the 2D app wants.
    fn auto_size_toplevel(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                7177229187692151305u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!(
            "Sent signal to server, {}::{}", "PanelItem", "auto_size_toplevel"
        );
        Ok(())
    }
    ///Request a resize of the surface (in pixels).
    fn set_toplevel_size(
        &self,
        size: impl Into<stardust_xr::values::Vector2<u32>>,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (size.into());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                8102855835344875634u64,
                &data,
                _fds,
            )?;
        let (size) = data;
        tracing::trace!(
            ? size, "Sent signal to server, {}::{}", "PanelItem", "set_toplevel_size"
        );
        Ok(())
    }
    ///Tell the toplevel to appear focused visually if true, or unfocused if false.
    fn set_toplevel_focused_visuals(
        &self,
        focused: bool,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (focused);
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                3934600665134956080u64,
                &data,
                _fds,
            )?;
        let (focused) = data;
        tracing::trace!(
            ? focused, "Sent signal to server, {}::{}", "PanelItem",
            "set_toplevel_focused_visuals"
        );
        Ok(())
    }
    ///Send an event to set the pointer's position (in pixels, relative to top-left of surface). This will activate the pointer.
    fn pointer_motion(
        &self,
        surface: SurfaceId,
        position: impl Into<stardust_xr::values::Vector2<f32>>,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface, position.into());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                651662101921814334u64,
                &data,
                _fds,
            )?;
        let (surface, position) = data;
        tracing::trace!(
            ? surface, ? position, "Sent signal to server, {}::{}", "PanelItem",
            "pointer_motion"
        );
        Ok(())
    }
    ///Send an event to set a pointer button's state if the pointer's active. The `button` is from the `input_event_codes` crate (e.g. BTN_LEFT for left click).
    fn pointer_button(
        &self,
        surface: SurfaceId,
        button: u32,
        pressed: bool,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface, button, pressed);
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                1617963334017359776u64,
                &data,
                _fds,
            )?;
        let (surface, button, pressed) = data;
        tracing::trace!(
            ? surface, ? button, ? pressed, "Sent signal to server, {}::{}", "PanelItem",
            "pointer_button"
        );
        Ok(())
    }
    /**Send an event to scroll the pointer if it's active.
Scroll distance is a value in pixels corresponding to the `distance` the surface should be scrolled.
Scroll steps is a value in columns/rows corresponding to the wheel clicks of a mouse or such. This also supports fractions of a wheel click.*/
    fn pointer_scroll(
        &self,
        surface: SurfaceId,
        scroll_distance: impl Into<stardust_xr::values::Vector2<f32>>,
        scroll_steps: impl Into<stardust_xr::values::Vector2<f32>>,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface, scroll_distance.into(), scroll_steps.into());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                18077910517219850499u64,
                &data,
                _fds,
            )?;
        let (surface, scroll_distance, scroll_steps) = data;
        tracing::trace!(
            ? surface, ? scroll_distance, ? scroll_steps,
            "Sent signal to server, {}::{}", "PanelItem", "pointer_scroll"
        );
        Ok(())
    }
    ///Send an event to stop scrolling the pointer.
    fn pointer_stop_scroll(&self, surface: SurfaceId) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface);
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                13177724628894942354u64,
                &data,
                _fds,
            )?;
        let (surface) = data;
        tracing::trace!(
            ? surface, "Sent signal to server, {}::{}", "PanelItem",
            "pointer_stop_scroll"
        );
        Ok(())
    }
    ///Send a key press or release event with the given keymap ID.
    fn keyboard_key(
        &self,
        surface: SurfaceId,
        keymap_id: u64,
        key: u32,
        pressed: bool,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface, keymap_id, key, pressed);
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                18230480350930328965u64,
                &data,
                _fds,
            )?;
        let (surface, keymap_id, key, pressed) = data;
        tracing::trace!(
            ? surface, ? keymap_id, ? key, ? pressed, "Sent signal to server, {}::{}",
            "PanelItem", "keyboard_key"
        );
        Ok(())
    }
    ///Put a touch down on this surface with the unique ID `uid` at `position` (in pixels) from top left corner of the surface.
    fn touch_down(
        &self,
        surface: SurfaceId,
        uid: u32,
        position: impl Into<stardust_xr::values::Vector2<f32>>,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (surface, uid, position.into());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                10543081656468919422u64,
                &data,
                _fds,
            )?;
        let (surface, uid, position) = data;
        tracing::trace!(
            ? surface, ? uid, ? position, "Sent signal to server, {}::{}", "PanelItem",
            "touch_down"
        );
        Ok(())
    }
    ///Move an existing touch point.
    fn touch_move(
        &self,
        uid: u32,
        position: impl Into<stardust_xr::values::Vector2<f32>>,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (uid, position.into());
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                15126475688563381777u64,
                &data,
                _fds,
            )?;
        let (uid, position) = data;
        tracing::trace!(
            ? uid, ? position, "Sent signal to server, {}::{}", "PanelItem", "touch_move"
        );
        Ok(())
    }
    ///Release a touch from its surface.
    fn touch_up(&self, uid: u32) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (uid);
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                6589027081119653997u64,
                &data,
                _fds,
            )?;
        let (uid) = data;
        tracing::trace!(? uid, "Sent signal to server, {}::{}", "PanelItem", "touch_up");
        Ok(())
    }
    ///Reset all input, such as pressed keys and pointer clicks and touches. Useful for when it's newly captured into an item acceptor to make sure no input gets stuck.
    fn reset_input(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                16007573185838633179u64,
                14629122800709746500u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "PanelItem", "reset_input");
        Ok(())
    }
}
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PanelItemUi(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl PanelItemUi {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<PanelItemUiEvent>(&node);
        PanelItemUi(node)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for PanelItemUi {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for PanelItemUi {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl PanelItemUiAspect for PanelItemUi {}
pub(crate) const PANEL_ITEM_UI_ASPECT_ID: u64 = 11713374794499719853u64;
pub(crate) const PANEL_ITEM_UI_CREATE_ITEM_CLIENT_OPCODE: u64 = 15524466827491111758u64;
pub(crate) const PANEL_ITEM_UI_CREATE_ACCEPTOR_CLIENT_OPCODE: u64 = 16628549773568263004u64;
#[derive(Debug)]
pub enum PanelItemUiEvent {
    CreateItem { item: PanelItem, initial_data: PanelItemInitData },
    CreateAcceptor { acceptor: PanelItemAcceptor, acceptor_field: Field },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for PanelItemUiEvent {
    const ASPECT_ID: u64 = 11713374794499719853u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            15524466827491111758u64 => {
                let (item, initial_data): (u64, PanelItemInitData) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? item, ? initial_data, "Got signal from server, {}::{}",
                    "PanelItemUi", "create_item"
                );
                Ok(PanelItemUiEvent::CreateItem {
                    item: PanelItem::from_id(&_client, item, false),
                    initial_data: initial_data,
                })
            }
            16628549773568263004u64 => {
                let (acceptor, acceptor_field): (u64, u64) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? acceptor, ? acceptor_field, "Got signal from server, {}::{}",
                    "PanelItemUi", "create_acceptor"
                );
                Ok(PanelItemUiEvent::CreateAcceptor {
                    acceptor: PanelItemAcceptor::from_id(&_client, acceptor, false),
                    acceptor_field: Field::from_id(&_client, acceptor_field, false),
                })
            }
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        }
    }
    fn serialize_method(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        method_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
        response: stardust_xr::messenger::MethodResponse,
    ) -> Option<Self> {
        let response = std::rc::Rc::new(std::cell::RefCell::new(Some(response)));
        let response2 = response.clone();
        let result = || match method_id {
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        };
        match (result)() {
            Ok(event) => Some(event),
            Err(e) => {
                let _ = response2.borrow_mut().take().unwrap().send(Err(e));
                None
            }
        }
    }
}
#[allow(clippy::all)]
///
pub trait PanelItemUiAspect: crate::node::NodeType + std::fmt::Debug {
    fn recv_panel_item_ui_event(&self) -> Option<PanelItemUiEvent> {
        self.node().recv_event(11713374794499719853u64)
    }
}
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PanelItemAcceptor(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl PanelItemAcceptor {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<PanelItemAcceptorEvent>(&node);
        PanelItemAcceptor(node)
    }
    pub fn as_item_acceptor(self) -> super::ItemAcceptor {
        super::ItemAcceptor(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for PanelItemAcceptor {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for PanelItemAcceptor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl PanelItemAcceptorAspect for PanelItemAcceptor {}
pub(crate) const PANEL_ITEM_ACCEPTOR_ASPECT_ID: u64 = 6398932320740499836u64;
pub(crate) const PANEL_ITEM_ACCEPTOR_CAPTURE_ITEM_SERVER_OPCODE: u64 = 1751367302976798762u64;
pub(crate) const PANEL_ITEM_ACCEPTOR_CAPTURE_ITEM_CLIENT_OPCODE: u64 = 1751367302976798762u64;
#[derive(Debug)]
pub enum PanelItemAcceptorEvent {
    CaptureItem { item: PanelItem, initial_data: PanelItemInitData },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for PanelItemAcceptorEvent {
    const ASPECT_ID: u64 = 6398932320740499836u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            1751367302976798762u64 => {
                let (item, initial_data): (u64, PanelItemInitData) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? item, ? initial_data, "Got signal from server, {}::{}",
                    "PanelItemAcceptor", "capture_item"
                );
                Ok(PanelItemAcceptorEvent::CaptureItem {
                    item: PanelItem::from_id(&_client, item, false),
                    initial_data: initial_data,
                })
            }
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        }
    }
    fn serialize_method(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        method_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
        response: stardust_xr::messenger::MethodResponse,
    ) -> Option<Self> {
        let response = std::rc::Rc::new(std::cell::RefCell::new(Some(response)));
        let response2 = response.clone();
        let result = || match method_id {
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        };
        match (result)() {
            Ok(event) => Some(event),
            Err(e) => {
                let _ = response2.borrow_mut().take().unwrap().send(Err(e));
                None
            }
        }
    }
}
#[allow(clippy::all)]
///
pub trait PanelItemAcceptorAspect: crate::node::NodeType + super::ItemAcceptorAspect + std::fmt::Debug {
    fn recv_panel_item_acceptor_event(&self) -> Option<PanelItemAcceptorEvent> {
        self.node().recv_event(6398932320740499836u64)
    }
    ///
    fn capture_item(&self, item: &impl PanelItemAspect) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (item.node().id());
        self.node()
            .send_remote_signal(
                6398932320740499836u64,
                1751367302976798762u64,
                &data,
                _fds,
            )?;
        let (item) = data;
        tracing::trace!(
            ? item, "Sent signal to server, {}::{}", "PanelItemAcceptor", "capture_item"
        );
        Ok(())
    }
}
pub(crate) const INTERFACE_REGISTER_KEYMAP_SERVER_OPCODE: u64 = 13267771052011565359u64;
///Register a keymap with the server to easily identify it later
pub async fn register_keymap(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    keymap: &str,
) -> crate::node::NodeResult<u64> {
    let mut _fds = Vec::new();
    let data = (keymap);
    {
        let (keymap) = &data;
        tracing::trace!(
            ? keymap, "Called method on server, {}::{}", "Interface", "register_keymap"
        );
    }
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    let message = _client
        .message_sender_handle
        .method(12u64, 0u64, 13267771052011565359u64, &serialized_data, _fds)
        .await?
        .map_err(|e| crate::node::NodeError::ReturnedError {
            e,
        })?
        .into_message();
    let result: u64 = stardust_xr::schemas::flex::deserialize(&message)?;
    let deserialized = result;
    tracing::trace!(
        "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
        "register_keymap"
    );
    Ok(deserialized)
}
pub(crate) const INTERFACE_GET_KEYMAP_SERVER_OPCODE: u64 = 18393315648981916968u64;
///Get the keymap string representation from an ID
pub async fn get_keymap(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    keymap_id: u64,
) -> crate::node::NodeResult<String> {
    let mut _fds = Vec::new();
    let data = (keymap_id);
    {
        let (keymap_id) = &data;
        tracing::trace!(
            ? keymap_id, "Called method on server, {}::{}", "Interface", "get_keymap"
        );
    }
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    let message = _client
        .message_sender_handle
        .method(12u64, 0u64, 18393315648981916968u64, &serialized_data, _fds)
        .await?
        .map_err(|e| crate::node::NodeError::ReturnedError {
            e,
        })?
        .into_message();
    let result: String = stardust_xr::schemas::flex::deserialize(&message)?;
    let deserialized = result;
    tracing::trace!(
        "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
        "get_keymap"
    );
    Ok(deserialized)
}
pub(crate) const INTERFACE_REGISTER_PANEL_ITEM_UI_SERVER_OPCODE: u64 = 13016197282381545765u64;
#[allow(clippy::all)]
///Register this client to manage the items of a certain type and create default 3D UI for them.
pub fn register_panel_item_ui(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
) -> crate::node::NodeResult<()> {
    let mut _fds = Vec::new();
    let data = ();
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    _client
        .message_sender_handle
        .signal(12u64, 0u64, 13016197282381545765u64, &serialized_data, _fds)?;
    let () = data;
    tracing::trace!(
        "Sent signal to server, {}::{}", "Interface", "register_panel_item_ui"
    );
    Ok(())
}
pub(crate) const INTERFACE_CREATE_PANEL_ITEM_ACCEPTOR_SERVER_OPCODE: u64 = 793626320493717815u64;
///Create an item acceptor to allow temporary ownership of a given type of item. Creates a node at `/item/panel/acceptor/<name>`.
fn create_panel_item_acceptor(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    field: &impl FieldAspect,
) -> crate::node::NodeResult<PanelItemAcceptor> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, field.node().id());
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(12u64, 0u64, 793626320493717815u64, &serialized_data, _fds)?;
        let (id, parent, transform, field) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? field, "Sent signal to server, {}::{}",
            "Interface", "create_panel_item_acceptor"
        );
    }
    Ok(PanelItemAcceptor::from_id(_client, id, true))
}
