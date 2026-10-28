//! Manual network and desktop launch smoke test for Fabric and Quilt.
//! Set UROD_LOADER_TEST_KIND to `fabric` or `quilt`; the game uses an offline test identity.

#[tokio::test]
#[ignore = "downloads a full Minecraft runtime and opens a game window"]
async fn install_and_launch_loader_1_21_1() {
	use urod_launcher_core::{CreateInstance, Engine, LaunchIdentity, Loader};
	let kind = std::env::var("UROD_LOADER_TEST_KIND").expect("set UROD_LOADER_TEST_KIND");
	let loader = match kind.as_str() {
		"fabric" => Loader::Fabric,
		"quilt" => Loader::Quilt,
		_ => panic!("use fabric or quilt"),
	};
	let temporary = tempfile::tempdir().expect("temporary launcher directory");
	let root = std::env::var_os("UROD_LOADER_TEST_ROOT")
		.map(std::path::PathBuf::from)
		.unwrap_or_else(|| temporary.path().to_path_buf());
	let pool = sqlx::SqlitePool::connect("sqlite::memory:")
		.await
		.expect("temporary database");
	let engine = Engine::open(root, pool).await.expect("engine");
	let versions = engine
		.loader_versions(loader, "1.21.1")
		.await
		.expect("loader versions");
	let version = versions
		.iter()
		.find(|item| item.stable)
		.or_else(|| versions.first())
		.expect("available loader version")
		.id
		.clone();
	let instance = engine
		.create_instance(CreateInstance {
			name: format!("{kind} integration test"),
			game_version: "1.21.1".into(),
			loader,
			loader_version: Some(version.clone()),
			memory_mb: 4096,
			window_width: None,
			window_height: None,
			java_path: None,
		})
		.await
		.expect("create instance");
	let op = engine.begin("install_game", Some(&instance.id));
	let installed = engine
		.install_game(&instance.id, &op)
		.await
		.expect("install game and loader");
	assert_eq!(installed.status, "ready");
	println!(
		"REAL INSTALL {kind} 1.21.1 loader {version}: {}",
		installed.id
	);
	let game = engine
		.launch(
			&installed.id,
			LaunchIdentity {
				nickname: "UrodQA".into(),
				uuid: "88611cf2-09a6-3ea9-b10a-62faecf8157a".into(),
				access_token: "0".into(),
				user_type: "legacy".into(),
				xuid: None,
				authlib_injector: None,
			},
		)
		.await
		.expect("launch game");
	println!("REAL LAUNCH {kind}: pid {}", game.pid);
	tokio::time::sleep(std::time::Duration::from_secs(20)).await;
	assert_eq!(
		engine.instance(&installed.id).await.unwrap().status,
		"running"
	);
	engine.stop(&installed.id).await.expect("stop game");
	println!("REAL STOP {kind}: PASS");
}
