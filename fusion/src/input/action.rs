use super::{InputData, InputHandlerHandler};
use rustc_hash::FxHashSet;
use std::{fmt::Debug, iter::FromIterator, mem::swap, sync::Arc};

pub trait InputActionState: Sized + Clone + Send + Sync + 'static {}
impl<T: Sized + Clone + Send + Sync + 'static> InputActionState for T {}

pub type ActiveCondition<S> = fn(&InputData, state: &S) -> bool;

pub trait InputAction<S: InputActionState> {
	fn base(&self) -> &BaseInputAction<S>;
	fn base_mut(&mut self) -> &mut BaseInputAction<S>;
	fn type_erase(&mut self) -> &mut dyn InputAction<S>
	where
		Self: Sized,
	{
		self as &mut dyn InputAction<S>
	}
}

#[derive(Clone)]
pub struct BaseInputAction<S: InputActionState> {
	pub capture_on_trigger: bool,
	pub active_condition: ActiveCondition<S>,

	pub started_acting: FxHashSet<Arc<InputData>>,
	pub actively_acting: FxHashSet<Arc<InputData>>,
	pub stopped_acting: FxHashSet<Arc<InputData>>,
	queued_inputs: FxHashSet<Arc<InputData>>,
}
impl<S: InputActionState> BaseInputAction<S> {
	pub fn new(capture_on_trigger: bool, active_condition: ActiveCondition<S>) -> Self {
		Self {
			capture_on_trigger,
			active_condition,

			started_acting: FxHashSet::default(),
			actively_acting: FxHashSet::default(),
			stopped_acting: FxHashSet::default(),
			queued_inputs: FxHashSet::default(),
		}
	}

	fn update(&mut self, external: &mut BaseInputAction<S>) {
		self.started_acting = FxHashSet::from_iter(
			self.queued_inputs
				.difference(&self.actively_acting)
				.cloned(),
		);
		self.stopped_acting = FxHashSet::from_iter(
			self.actively_acting
				.difference(&self.queued_inputs)
				.cloned(),
		);
		swap(&mut self.actively_acting, &mut self.queued_inputs);
		self.queued_inputs.clear();

		external.started_acting = self.started_acting.clone();
		external.actively_acting = self.actively_acting.clone();
		external.stopped_acting = self.stopped_acting.clone();
		external.started_acting = self.started_acting.clone();

		self.capture_on_trigger = external.capture_on_trigger;
		self.active_condition = external.active_condition;
	}

	fn input_event(&mut self, input_data: &Arc<InputData>, state: &S) -> bool {
		if (self.active_condition)(input_data, state) {
			self.queued_inputs.insert(input_data.clone());
			true
		} else {
			false
		}
	}
}

impl<S: InputActionState> InputAction<S> for BaseInputAction<S> {
	fn base(&self) -> &BaseInputAction<S> {
		self
	}
	fn base_mut(&mut self) -> &mut BaseInputAction<S> {
		self
	}
}
impl<S: InputActionState> PartialEq for BaseInputAction<S> {
	fn eq(&self, other: &Self) -> bool {
		self.capture_on_trigger == other.capture_on_trigger
			&& self.active_condition as usize == other.active_condition as usize
	}
}
impl<S: InputActionState> Debug for BaseInputAction<S> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("InputAction")
			.field("capture_on_trigger", &self.capture_on_trigger)
			.field("started_acting", &self.started_acting)
			.field("actively_acting", &self.actively_acting)
			.field("stopped_acting", &self.stopped_acting)
			.field("queued_inputs", &self.queued_inputs)
			.finish()
	}
}

#[derive(Debug, Default)]
pub struct InputActionHandler<S: InputActionState> {
	actions: Vec<BaseInputAction<S>>,
	state: S,
	back_state: S,
}
impl<S: InputActionState> InputActionHandler<S> {
	pub fn new(state: S) -> Self {
		Self {
			actions: Vec::new(),
			back_state: state.clone(),
			state,
		}
	}

	pub fn update_actions<'a>(
		&mut self,
		actions: impl IntoIterator<Item = &'a mut (dyn InputAction<S> + 'a)>,
	) {
		self.back_state = self.state.clone();

		self.actions = actions
			.into_iter()
			.map(|action| {
				if let Some(internal_action) = self
					.actions
					.iter_mut()
					.find(|internal_action| **internal_action == *action.base())
				{
					internal_action.update(action.base_mut());
				}
				action.base().clone()
			})
			.collect();
	}
}
impl<S: InputActionState> InputHandlerHandler for InputActionHandler<S> {
	fn input(&mut self, input: InputData) -> bool {
		let input = Arc::new(input);
		self.actions
			.iter_mut()
			.map(|action| action.input_event(&input, &self.state) && action.capture_on_trigger)
			.any(|b| b)
	}
}

#[tokio::test]
async fn fusion_input_action_handler() {
	use crate::{client::Client, fields::SphereField, input::InputHandler};
	use stardust_xr::values::Transform;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	struct InputActionHandlerTest {
		field: SphereField,
		input_handler: crate::HandlerWrapper<InputHandler, InputActionHandler<f32>>,
		hover_action: BaseInputAction<f32>,
		fancy_action: FancyInputAction<f32>,
	}

	let field = SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1).unwrap();
	let input_action_test = InputActionHandlerTest {
		input_handler: InputHandler::create(client.get_root(), Transform::default(), &field)
			.unwrap()
			.wrap(InputActionHandler::new(0.05))
			.unwrap(),
		hover_action: BaseInputAction::new(false, |input_data, max_distance| {
			dbg!(input_data);
			input_data.distance < *max_distance
		}),
		fancy_action: FancyInputAction::default(),
		field,
	};
	struct FancyInputAction<S: InputActionState> {
		action: BaseInputAction<S>,
	}
	impl<S: InputActionState> Default for FancyInputAction<S> {
		fn default() -> Self {
			Self {
				action: BaseInputAction::new(false, |_, _| true),
			}
		}
	}
	impl<S: InputActionState> InputAction<S> for FancyInputAction<S> {
		fn base(&self) -> &BaseInputAction<S> {
			&self.action
		}
		fn base_mut(&mut self) -> &mut BaseInputAction<S> {
			&mut self.action
		}
	}

	impl crate::client::RootHandler for InputActionHandlerTest {
		fn frame(&mut self, info: crate::client::FrameInfo) {
			println!("Life cycle step {}s", info.elapsed);
			self.input_handler.lock_wrapped().update_actions(
				[
					self.hover_action.type_erase(),
					self.fancy_action.type_erase(),
				]
				.into_iter(),
			);
			// dbg!(&self.hover_action);
		}
	}

	let _root = client.wrap_root(input_action_test);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
