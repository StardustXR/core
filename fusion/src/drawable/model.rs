use crate::{
	node::{NodeAspect, NodeError, NodeResult, NodeType},
	spatial::{SpatialAspect, Transform},
};
use stardust_xr::values::ResourceID;

stardust_xr_fusion_codegen::codegen_drawable_model_client_protocol!();
impl Model {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		model: &ResourceID,
	) -> NodeResult<Self> {
		load_model(
			&spatial_parent.client()?,
			&nanoid::nanoid!(),
			spatial_parent,
			transform,
			model,
		)
	}

	/// Set a property of a material on this model.
	pub fn model_part(&self, relative_path: &str) -> NodeResult<ModelPart> {
		if relative_path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}
		Ok(ModelPart::from_parent_name(
			&self.client()?,
			&self.node().get_path()?,
			relative_path,
			false,
		))
	}
}

#[tokio::test]
async fn fusion_model() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let gyro_resource = ResourceID::new_namespaced("fusion", "gyro");
	let gyro_model = Model::create(client.get_root(), Transform::none(), &gyro_resource).unwrap();
	gyro_model
		.model_part("Gem")
		.unwrap()
		.set_material_parameter(
			"color",
			MaterialParameter::Color(color::rgba_linear!(0.0, 1.0, 0.5, 0.75)),
		)
		.unwrap();

	let spike_resource = ResourceID::new_namespaced("fusion", "cursor_spike");
	let spike_model = Model::create(
		client.get_root(),
		Transform::from_translation_scale([0.0, 0.1, 0.0], [0.1; 3]),
		&spike_resource,
	)
	.unwrap();
	spike_model
		.model_part("Cone")
		.unwrap()
		.apply_holdout_material()
		.unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
