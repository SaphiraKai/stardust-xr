use anyhow::Result;
use glam::Quat;
use manifest_dir_macros::directory_relative_path;
use stardust_xr_fusion::{
	client::{Client, LifeCycleHandler, LogicStepInfo},
	drawable::Model,
	resource::NamespacedResource,
	spatial::Spatial,
};
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	let _root = client.wrap_root(SpatialDemo::new(&client)?);

	tokio::select! {
		_ = tokio::signal::ctrl_c() => Ok(()),
		_ = event_loop => Err(anyhow::anyhow!("Server crashed")),
	}
}

struct SpatialDemo {
	_root: Spatial,
	gem: Model,
	ring_inner: Model,
	ring_middle: Model,
	ring_outer: Model,
}
impl SpatialDemo {
	fn new(client: &Arc<Client>) -> Result<Self> {
		let _root = Spatial::builder()
			.spatial_parent(client.get_root())
			.zoneable(true)
			.build()?;

		let gem = Model::builder()
			.spatial_parent(&_root)
			.resource(&NamespacedResource::new("fusion", "gyro_gem.glb"))
			.build()?;
		let ring_inner = Model::builder()
			.spatial_parent(&_root)
			.resource(&NamespacedResource::new("fusion", "gyro_inside.glb"))
			.build()?;
		let ring_middle = Model::builder()
			.spatial_parent(&ring_inner)
			.resource(&NamespacedResource::new("fusion", "gyro_middle.glb"))
			.build()?;
		let ring_outer = Model::builder()
			.spatial_parent(&ring_middle)
			.resource(&NamespacedResource::new("fusion", "gyro_outside.glb"))
			.build()?;

		Ok(SpatialDemo {
			_root,
			gem,
			ring_inner,
			ring_middle,
			ring_outer,
		})
	}
}
impl LifeCycleHandler for SpatialDemo {
	fn logic_step(&mut self, info: LogicStepInfo) {
		let elapsed = info.elapsed as f32;

		self.gem
			.set_rotation(None, Quat::from_rotation_y(elapsed))
			.unwrap();
		self.ring_inner
			.set_rotation(None, Quat::from_rotation_z(elapsed))
			.unwrap();
		self.ring_middle
			.set_rotation(None, Quat::from_rotation_x(elapsed))
			.unwrap();
		self.ring_outer
			.set_rotation(None, Quat::from_rotation_z(elapsed))
			.unwrap();
	}
}
