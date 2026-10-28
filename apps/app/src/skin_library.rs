use crate::skins::{Skin, render};
use crate::storage::{Result, Store};
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

const MAX_PNG_BYTES: usize = 1_048_576;
const MAX_ITEMS: usize = 128;

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedSkin {
	pub id: String,
	pub name: String,
	pub slim: bool,
	#[serde(default)]
	pub active_for: Vec<String>,
	#[serde(skip_deserializing)]
	pub preview: Option<Skin>,
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct SkinHistoryEntry {
	pub id: i64,
	pub account_uuid: String,
	pub skin_id: Option<String>,
	pub skin_name: String,
	pub applied_at: String,
}

pub async fn history(store: &Store, account: &str) -> Result<Vec<SkinHistoryEntry>> {
	store.account(account).await?;
	sqlx::query_as::<_, SkinHistoryEntry>("SELECT id, account_uuid, skin_id, skin_name, applied_at FROM skin_history WHERE account_uuid = ? ORDER BY id DESC LIMIT 100")
		.bind(account).fetch_all(&store.pool).await.map_err(|_| "Не удалось прочитать историю скинов".into())
}

async fn record_history(store: &Store, account: &str, skin: Option<(&str, &str)>) -> Result<()> {
	let (id, name) = skin.map_or((None, "Стандартный скин"), |(id, name)| {
		(Some(id), name)
	});
	sqlx::query("INSERT INTO skin_history(account_uuid, skin_id, skin_name) VALUES(?, ?, ?)")
		.bind(account)
		.bind(id)
		.bind(name)
		.execute(&store.pool)
		.await
		.map_err(|_| "Не удалось сохранить историю скинов".to_string())?;
	Ok(())
}

fn directory(store: &Store) -> std::path::PathBuf {
	store.directory.join("skin-library")
}
async fn index(store: &Store) -> Result<Vec<SavedSkin>> {
	match tokio::fs::read(directory(store).join("index.json")).await {
		Ok(bytes) if bytes.len() <= 65536 => serde_json::from_slice(&bytes).map_err(|_| {
			"Повреждён каталог скинов. Восстановите index.json из резервной копии.".into()
		}),
		Ok(_) => Err("Каталог скинов слишком большой".into()),
		Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
		Err(_) => Err("Не удалось прочитать каталог скинов".into()),
	}
}
async fn persist(store: &Store, skins: &[SavedSkin]) -> Result<()> {
	let root = directory(store);
	tokio::fs::create_dir_all(&root)
		.await
		.map_err(|_| "Не удалось создать каталог скинов")?;
	let temporary = root.join(format!("{}.tmp", Uuid::new_v4()));
	let data = serde_json::to_vec(skins).map_err(|_| "Не удалось сохранить каталог скинов")?;
	tokio::fs::write(&temporary, data)
		.await
		.map_err(|_| "Не удалось сохранить каталог скинов")?;
	if tokio::fs::rename(&temporary, root.join("index.json"))
		.await
		.is_err()
	{
		let _ = tokio::fs::remove_file(&temporary).await;
		return Err("Не удалось обновить каталог скинов".into());
	}
	Ok(())
}
fn skin_path(store: &Store, id: &str) -> Result<std::path::PathBuf> {
	Uuid::parse_str(id).map_err(|_| "Некорректный идентификатор скина")?;
	Ok(directory(store).join(format!("{id}.png")))
}
fn valid_name(name: &str) -> Result<String> {
	let name = name.trim();
	if name.is_empty() || name.chars().count() > 48 || name.chars().any(char::is_control) {
		return Err("Название скина должно содержать от 1 до 48 символов".into());
	}
	Ok(name.into())
}
fn decode_png(data: &str) -> Result<Vec<u8>> {
	let raw = data.strip_prefix("data:image/png;base64,").unwrap_or(data);
	if raw.len() > MAX_PNG_BYTES * 4 / 3 + 8 {
		return Err("PNG слишком большой".into());
	}
	let bytes = BASE64_STANDARD
		.decode(raw)
		.map_err(|_| "Некорректный PNG")?;
	if bytes.len() > MAX_PNG_BYTES || !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
		return Err("Выберите PNG размером не более 1 МБ".into());
	}
	render(&bytes, "local", "ready")?;
	Ok(bytes)
}
async fn read_skin(store: &Store, id: &str) -> Result<Vec<u8>> {
	let path = skin_path(store, id)?;
	let metadata = tokio::fs::symlink_metadata(&path)
		.await
		.map_err(|_| "Скин не найден")?;
	if metadata.file_type().is_symlink()
		|| !metadata.is_file()
		|| metadata.len() > MAX_PNG_BYTES as u64
	{
		return Err("Файл скина повреждён".into());
	}
	let bytes = tokio::fs::read(path)
		.await
		.map_err(|_| "Не удалось прочитать скин")?;
	render(&bytes, "local", "ready")?;
	Ok(bytes)
}
pub async fn list(store: &Store) -> Result<Vec<SavedSkin>> {
	let mut skins = index(store).await?;
	for item in &mut skins {
		item.preview = read_skin(store, &item.id)
			.await
			.ok()
			.and_then(|bytes| render(&bytes, "local", "ready").ok());
	}
	Ok(skins)
}
pub async fn save(store: &Store, name: &str, slim: bool, data: &str) -> Result<Vec<SavedSkin>> {
	let name = valid_name(name)?;
	let bytes = decode_png(data)?;
	let mut skins = index(store).await?;
	if skins.len() >= MAX_ITEMS {
		return Err("Библиотека заполнена (128 скинов)".into());
	}
	let id = Uuid::new_v4().to_string();
	let path = skin_path(store, &id)?;
	tokio::fs::create_dir_all(path.parent().unwrap_or(Path::new(".")))
		.await
		.map_err(|_| "Не удалось создать каталог скинов")?;
	tokio::fs::write(&path, bytes)
		.await
		.map_err(|_| "Не удалось сохранить скин")?;
	skins.insert(
		0,
		SavedSkin {
			id,
			name,
			slim,
			active_for: Vec::new(),
			preview: None,
		},
	);
	if let Err(error) = persist(store, &skins).await {
		let _ = tokio::fs::remove_file(path).await;
		return Err(error);
	}
	list(store).await
}
pub async fn rename(
	store: &Store,
	id: &str,
	name: &str,
	slim: Option<bool>,
) -> Result<Vec<SavedSkin>> {
	let name = valid_name(name)?;
	let mut skins = index(store).await?;
	let item = skins
		.iter_mut()
		.find(|item| item.id == id)
		.ok_or("Скин не найден")?;
	item.name = name;
	if let Some(slim) = slim {
		if item.slim != slim {
			item.active_for.clear();
		}
		item.slim = slim;
	}
	persist(store, &skins).await?;
	list(store).await
}
pub async fn update(
	store: &Store,
	id: &str,
	name: &str,
	slim: bool,
	data: Option<&str>,
) -> Result<Vec<SavedSkin>> {
	let name = valid_name(name)?;
	let replacement = data.map(decode_png).transpose()?;
	let mut skins = index(store).await?;
	let item = skins
		.iter_mut()
		.find(|item| item.id == id)
		.ok_or("Скин не найден")?;
	let changed_appearance = item.slim != slim || replacement.is_some();
	item.name = name;
	item.slim = slim;
	if changed_appearance {
		item.active_for.clear();
	}
	if let Some(bytes) = replacement {
		let path = skin_path(store, id)?;
		let original = read_skin(store, id).await?;
		let staging = directory(store).join(format!("{}.tmp", Uuid::new_v4()));
		tokio::fs::write(&staging, bytes)
			.await
			.map_err(|_| "Не удалось подготовить новую текстуру")?;
		if tokio::fs::rename(&staging, &path).await.is_err() {
			let _ = tokio::fs::remove_file(staging).await;
			return Err("Не удалось заменить текстуру скина".into());
		}
		if let Err(error) = persist(store, &skins).await {
			let rollback = directory(store).join(format!("{}.tmp", Uuid::new_v4()));
			if tokio::fs::write(&rollback, original).await.is_err()
				|| tokio::fs::rename(&rollback, &path).await.is_err()
			{
				return Err(
					"Не удалось восстановить прежнюю текстуру после ошибки записи каталога".into(),
				);
			}
			return Err(error);
		}
	} else {
		persist(store, &skins).await?;
	}
	list(store).await
}
pub async fn remove(store: &Store, id: &str) -> Result<Vec<SavedSkin>> {
	let mut skins = index(store).await?;
	let path = skin_path(store, id)?;
	let count = skins.len();
	skins.retain(|item| item.id != id);
	if count == skins.len() {
		return Err("Скин не найден".into());
	}
	persist(store, &skins).await?;
	let _ = tokio::fs::remove_file(path).await;
	list(store).await
}
pub async fn local_skin(store: &Store, account: &str) -> Result<Option<Skin>> {
	let skins = index(store).await?;
	let Some(item) = skins
		.iter()
		.find(|item| item.active_for.iter().any(|id| id == account))
	else {
		return Ok(None);
	};
	let bytes = read_skin(store, &item.id).await?;
	Ok(Some(render(&bytes, "local", "ready")?))
}
pub async fn apply_local(store: &Store, account: &str, id: &str) -> Result<Vec<SavedSkin>> {
	store.account(account).await?;
	read_skin(store, id).await?;
	let mut skins = index(store).await?;
	if !skins.iter().any(|item| item.id == id) {
		return Err("Скин не найден".into());
	}
	for item in &mut skins {
		item.active_for.retain(|value| value != account);
		if item.id == id {
			item.active_for.push(account.into());
		}
	}
	persist(store, &skins).await?;
	let name = &skins
		.iter()
		.find(|item| item.id == id)
		.ok_or("Скин не найден")?
		.name;
	record_history(store, account, Some((id, name))).await?;
	list(store).await
}
pub async fn reset_local(store: &Store, account: &str) -> Result<Vec<SavedSkin>> {
	store.account(account).await?;
	let mut skins = index(store).await?;
	for item in &mut skins {
		item.active_for.retain(|value| value != account);
	}
	persist(store, &skins).await?;
	record_history(store, account, None).await?;
	list(store).await
}
pub async fn bytes(store: &Store, id: &str) -> Result<(Vec<u8>, bool)> {
	let skins = index(store).await?;
	let item = skins
		.iter()
		.find(|item| item.id == id)
		.ok_or("Скин не найден")?;
	Ok((read_skin(store, id).await?, item.slim))
}
pub async fn upload_microsoft(
	store: &Store,
	account: &str,
	id: &str,
	token: &str,
) -> Result<Vec<SavedSkin>> {
	let client = reqwest::Client::builder()
		.https_only(true)
		.redirect(reqwest::redirect::Policy::none())
		.timeout(std::time::Duration::from_secs(20))
		.build()
		.map_err(|_| "Не удалось подготовить соединение Minecraft")?;
	upload_microsoft_with_client(
		store,
		account,
		id,
		token,
		&client,
		"https://api.minecraftservices.com/minecraft/profile",
	)
	.await
}

async fn upload_microsoft_with_client(
	store: &Store,
	account: &str,
	id: &str,
	token: &str,
	client: &reqwest::Client,
	endpoint: &str,
) -> Result<Vec<SavedSkin>> {
	let (bytes, slim) = bytes(store, id).await?;
	let boundary = format!("urod-{}", Uuid::new_v4());
	let mut body = format!("--{boundary}\r\nContent-Disposition: form-data; name=\"variant\"\r\n\r\n{}\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"skin.png\"\r\nContent-Type: image/png\r\n\r\n", if slim { "slim" } else { "classic" }).into_bytes();
	body.extend_from_slice(&bytes);
	body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
	let response = client
		.post(format!("{endpoint}/skins"))
		.bearer_auth(token)
		.header(
			reqwest::header::CONTENT_TYPE,
			format!("multipart/form-data; boundary={boundary}"),
		)
		.body(body)
		.send()
		.await
		.map_err(|_| "Minecraft Services недоступен")?;
	if !response.status().is_success() {
		return Err(if response.status() == reqwest::StatusCode::UNAUTHORIZED {
			"Сессия Minecraft истекла. Войдите в Microsoft снова.".into()
		} else {
			format!(
				"Minecraft Services отклонил скин: HTTP {}",
				response.status().as_u16()
			)
		});
	}
	let profile = client
		.get(endpoint)
		.bearer_auth(token)
		.send()
		.await
		.map_err(|_| "Не удалось проверить профиль Minecraft")?;
	if !profile.status().is_success() || profile.content_length().is_some_and(|n| n > 65536) {
		return Err("Не удалось проверить профиль Minecraft после загрузки".into());
	}
	let body = profile
		.bytes()
		.await
		.map_err(|_| "Не удалось прочитать профиль Minecraft")?;
	if body.len() > 65536 {
		return Err("Ответ профиля Minecraft слишком большой".into());
	}
	let value: serde_json::Value =
		serde_json::from_slice(&body).map_err(|_| "Некорректный профиль Minecraft")?;
	let profile_id = value["id"].as_str().unwrap_or("");
	let account_id = account.replace('-', "");
	if !profile_id.eq_ignore_ascii_case(&account_id) {
		return Err("Minecraft вернул другой профиль".into());
	}
	let active = value["skins"]
		.as_array()
		.and_then(|items| items.iter().find(|item| item["state"] == "ACTIVE"))
		.ok_or("Скин ещё не появился в профиле Minecraft. Проверьте позже.")?;
	let variant = active["variant"].as_str().unwrap_or("");
	if !variant.eq_ignore_ascii_case(if slim { "slim" } else { "classic" }) {
		return Err("Minecraft вернул другую модель скина".into());
	}
	let skin_url = active["url"]
		.as_str()
		.ok_or("Minecraft не вернул URL скина")?;
	let parsed = url::Url::parse(skin_url).map_err(|_| "Некорректный URL скина Minecraft")?;
	if !["http", "https"].contains(&parsed.scheme())
		|| parsed.host_str() != Some("textures.minecraft.net")
		|| !parsed.path().starts_with("/texture/")
	{
		return Err("Minecraft вернул недопустимый URL скина".into());
	}
	sqlx::query("UPDATE accounts SET skin_url = ?, skin_provider = 'mojang' WHERE uuid = ?")
		.bind(skin_url)
		.bind(account)
		.execute(&store.pool)
		.await
		.map_err(|_| "Не удалось сохранить подтверждённый скин")?;
	let mut skins = index(store).await?;
	for item in &mut skins {
		item.active_for.retain(|value| value != account);
		if item.id == id {
			item.active_for.push(account.into());
		}
	}
	persist(store, &skins).await?;
	let name = &skins
		.iter()
		.find(|item| item.id == id)
		.ok_or("Скин не найден")?
		.name;
	record_history(store, account, Some((id, name))).await?;
	list(store).await
}

pub async fn reset_microsoft(store: &Store, account: &str, token: &str) -> Result<Vec<SavedSkin>> {
	let previous_url: Option<String> =
		sqlx::query_scalar("SELECT skin_url FROM accounts WHERE uuid = ?")
			.bind(account)
			.fetch_optional(&store.pool)
			.await
			.map_err(|_| "Не удалось прочитать текущий скин")?
			.flatten();
	let client = reqwest::Client::builder()
		.https_only(true)
		.redirect(reqwest::redirect::Policy::none())
		.timeout(std::time::Duration::from_secs(20))
		.build()
		.map_err(|_| "Не удалось подготовить соединение Minecraft")?;
	let endpoint = "https://api.minecraftservices.com/minecraft/profile";
	let response = client
		.delete(format!("{endpoint}/skins/active"))
		.bearer_auth(token)
		.send()
		.await
		.map_err(|_| "Minecraft Services недоступен")?;
	if !response.status().is_success() {
		return Err(format!(
			"Minecraft Services отклонил сброс: HTTP {}",
			response.status().as_u16()
		));
	}
	let profile = client
		.get(endpoint)
		.bearer_auth(token)
		.send()
		.await
		.map_err(|_| "Не удалось проверить сброс скина")?;
	if !profile.status().is_success() || profile.content_length().is_some_and(|n| n > 65536) {
		return Err("Не удалось проверить сброс скина".into());
	}
	let body = profile
		.bytes()
		.await
		.map_err(|_| "Не удалось прочитать профиль Minecraft")?;
	if body.len() > 65536 {
		return Err("Ответ профиля Minecraft слишком большой".into());
	}
	let value: serde_json::Value =
		serde_json::from_slice(&body).map_err(|_| "Некорректный профиль Minecraft")?;
	if !value["id"]
		.as_str()
		.unwrap_or("")
		.eq_ignore_ascii_case(&account.replace('-', ""))
	{
		return Err("Minecraft вернул другой профиль".into());
	}
	let active_url = value["skins"]
		.as_array()
		.and_then(|items| items.iter().find(|item| item["state"] == "ACTIVE"))
		.and_then(|item| item["url"].as_str());
	if previous_url
		.as_deref()
		.is_some_and(|url| Some(url) == active_url)
	{
		return Err("Minecraft всё ещё показывает прежний скин. Проверьте профиль позже.".into());
	}
	sqlx::query("UPDATE accounts SET skin_url = NULL, skin_provider = 'fallback' WHERE uuid = ?")
		.bind(account)
		.execute(&store.pool)
		.await
		.map_err(|_| "Не удалось сохранить сброс скина")?;
	reset_local(store, account).await
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn rejects_invalid_skin_data_and_unsafe_names() {
		assert!(decode_png("data:image/png;base64,YWJj").is_err());
		assert!(valid_name("\n").is_err());
		assert!(valid_name(&"a".repeat(49)).is_err());
	}
	#[tokio::test]
	async fn stores_multiple_skins_and_restores_active_offline_skin() {
		let directory = std::env::temp_dir().join(format!("urod-skins-{}", Uuid::new_v4()));
		let store = Store::open(directory.clone()).await.unwrap();
		store.add_offline("offline_test").await.unwrap();
		let account = crate::storage::offline_uuid("offline_test").to_string();
		let png = BASE64_STANDARD.encode(include_bytes!("../assets/steve.png"));
		let first = save(&store, "Первый", false, &png).await.unwrap();
		let first_id = first[0].id.clone();
		let second = save(&store, "Второй", true, &png).await.unwrap();
		let second_id = second[0].id.clone();
		assert_eq!(second.len(), 2);
		let edited = rename(&store, &second_id, "Переименован", Some(false))
			.await
			.unwrap();
		assert_eq!(edited[0].name, "Переименован");
		assert!(!edited[0].slim);
		apply_local(&store, &account, &first_id).await.unwrap();
		assert!(local_skin(&store, &account).await.unwrap().is_some());
		apply_local(&store, &account, &second_id).await.unwrap();
		let reloaded = list(&store).await.unwrap();
		assert_eq!(
			reloaded
				.iter()
				.filter(|item| item.active_for.len() == 1)
				.count(),
			1
		);
		assert_eq!(reloaded[0].id, second_id);
		assert_eq!(reloaded[0].active_for, [account.as_str()]);
		assert_eq!(history(&store, &account).await.unwrap().len(), 2);
		let original = bytes(&store, &second_id).await.unwrap().0;
		assert!(
			update(&store, &second_id, "Изменён", true, Some("not a PNG"))
				.await
				.is_err()
		);
		assert_eq!(bytes(&store, &second_id).await.unwrap().0, original);
		assert_eq!(list(&store).await.unwrap()[0].name, "Переименован");
		let mut image = image::load_from_memory(&original).unwrap().to_rgba8();
		image.put_pixel(8, 8, image::Rgba([244, 32, 31, 255]));
		let mut encoded = std::io::Cursor::new(Vec::new());
		image::DynamicImage::ImageRgba8(image)
			.write_to(&mut encoded, image::ImageFormat::Png)
			.unwrap();
		let replacement = BASE64_STANDARD.encode(encoded.into_inner());
		let updated = update(&store, &second_id, "Изменён", true, Some(&replacement))
			.await
			.unwrap();
		assert_eq!(updated[0].name, "Изменён");
		assert!(updated[0].slim);
		assert!(updated[0].active_for.is_empty());
		assert_ne!(bytes(&store, &second_id).await.unwrap().0, original);
		assert!(local_skin(&store, &account).await.unwrap().is_none());
		reset_local(&store, &account).await.unwrap();
		assert!(local_skin(&store, &account).await.unwrap().is_none());
		assert_eq!(history(&store, &account).await.unwrap().len(), 3);
		remove(&store, &first_id).await.unwrap();
		assert_eq!(list(&store).await.unwrap().len(), 1);
		drop(store);
		let _ = tokio::fs::remove_dir_all(directory).await;
	}
	#[tokio::test]
	async fn fifteen_pngs_keep_per_account_selections_and_history_after_reopens() {
		let directory = std::env::temp_dir().join(format!("urod-skin-stress-{}", Uuid::new_v4()));
		let store = Store::open(directory.clone())
			.await
			.expect("isolated store");
		let accounts = ["SkinUserA", "SkinUserB", "SkinUserC"];
		for name in accounts {
			store.add_offline(name).await.expect("account");
		}
		let base = image::load_from_memory(include_bytes!("../assets/steve.png"))
			.unwrap()
			.to_rgba8();
		let mut ids = Vec::new();
		for n in 0..15 {
			let mut image = base.clone();
			image.put_pixel(8, 8, image::Rgba([n, 40, 80, 255]));
			let mut output = std::io::Cursor::new(Vec::new());
			image::DynamicImage::ImageRgba8(image)
				.write_to(&mut output, image::ImageFormat::Png)
				.unwrap();
			let saved = save(
				&store,
				&format!("Skin {n}"),
				n % 2 == 1,
				&BASE64_STANDARD.encode(output.into_inner()),
			)
			.await
			.expect("PNG import");
			ids.push(saved[0].id.clone());
		}
		assert_eq!(list(&store).await.unwrap().len(), 15);
		assert_eq!(
			ids.iter().collect::<std::collections::HashSet<_>>().len(),
			15
		);
		for (position, name) in accounts.iter().enumerate() {
			let account = crate::storage::offline_uuid(name).to_string();
			for id in ids.iter().skip(position).take(3) {
				apply_local(&store, &account, id)
					.await
					.expect("select skin");
			}
			assert_eq!(history(&store, &account).await.unwrap().len(), 3);
			assert_eq!(
				history(&store, &account).await.unwrap()[0]
					.skin_id
					.as_deref(),
				Some(ids[position + 2].as_str())
			);
		}
		let malformed = BASE64_STANDARD.encode(b"\x89PNG\r\n\x1a\ninvalid");
		assert!(save(&store, "broken", false, &malformed).await.is_err());
		let invalid_size = image::DynamicImage::new_rgba8(32, 32);
		let mut output = std::io::Cursor::new(Vec::new());
		invalid_size
			.write_to(&mut output, image::ImageFormat::Png)
			.unwrap();
		assert!(
			save(
				&store,
				"wrong size",
				false,
				&BASE64_STANDARD.encode(output.into_inner())
			)
			.await
			.is_err()
		);
		store.pool.close().await;
		for _ in 0..20 {
			let reopened = Store::open(directory.clone()).await.expect("reopen store");
			assert_eq!(list(&reopened).await.unwrap().len(), 15);
			for (position, name) in accounts.iter().enumerate() {
				let account = crate::storage::offline_uuid(name).to_string();
				let selected = list(&reopened)
					.await
					.unwrap()
					.into_iter()
					.find(|item| item.active_for.contains(&account))
					.expect("selected skin");
				assert_eq!(selected.id, ids[position + 2]);
				let history = history(&reopened, &account).await.expect("history");
				assert_eq!(history.len(), 3);
				assert!(
					history
						.iter()
						.all(|entry| entry.account_uuid == account && !entry.applied_at.is_empty())
				);
			}
			reopened.pool.close().await;
		}
		let reopened = Store::open(directory.clone()).await.unwrap();
		let account = crate::storage::offline_uuid(accounts[0]).to_string();
		apply_local(&reopened, &account, &ids[0])
			.await
			.expect("restore previous skin");
		assert_eq!(history(&reopened, &account).await.unwrap().len(), 4);
		rename(&reopened, &ids[0], "Restored", None)
			.await
			.expect("rename skin");
		remove(&reopened, &ids[1]).await.expect("delete skin");
		assert_eq!(list(&reopened).await.unwrap().len(), 14);
		reopened.pool.close().await;
		tokio::fs::remove_dir_all(directory).await.expect("cleanup");
	}
	#[tokio::test]
	async fn microsoft_skin_mock_records_only_a_verified_profile_change() {
		use std::sync::{
			Arc,
			atomic::{AtomicBool, AtomicU8, Ordering},
		};
		let directory = std::env::temp_dir().join(format!("urod-ms-skin-{}", Uuid::new_v4()));
		let store = Store::open(directory.clone())
			.await
			.expect("isolated store");
		let account_id = Uuid::new_v4();
		store
			.microsoft(
				urod_app_lib::MicrosoftProfile {
					uuid: account_id,
					nickname: "SyntheticMicrosoft".into(),
					skin_url: None,
				},
				true,
			)
			.await
			.expect("synthetic profile");
		let account = account_id.to_string();
		let png = BASE64_STANDARD.encode(include_bytes!("../assets/steve.png"));
		let id = save(&store, "Mock Skin", false, &png).await.unwrap()[0]
			.id
			.clone();
		let server = tiny_http::Server::http("127.0.0.1:0").expect("mock bind");
		let endpoint = format!("http://{}/minecraft/profile", server.server_addr());
		let mode = Arc::new(AtomicU8::new(1));
		let stop = Arc::new(AtomicBool::new(false));
		let worker_mode = mode.clone();
		let worker_stop = stop.clone();
		let expected_id = account.replace('-', "");
		let worker = std::thread::spawn(move || {
			while !worker_stop.load(Ordering::SeqCst) {
				let Some(request) = server
					.recv_timeout(std::time::Duration::from_millis(20))
					.expect("mock request")
				else {
					continue;
				};
				assert!(
					request
						.headers()
						.iter()
						.any(|header| header.field.equiv("Authorization")
							&& header.value.as_str() == "Bearer synthetic-skin-token")
				);
				let phase = worker_mode.load(Ordering::SeqCst);
				let response = if request.url().ends_with("/skins") {
					if phase == 1 {
						tiny_http::Response::from_string("expired").with_status_code(401)
					} else if phase == 3 {
						tiny_http::Response::from_string("offline").with_status_code(503)
					} else {
						tiny_http::Response::from_string("{}").with_status_code(200)
					}
				} else if phase == 4 {
					tiny_http::Response::from_string("offline").with_status_code(503)
				} else {
					let id = if phase == 2 {
						"wrong-profile"
					} else {
						&expected_id
					};
					let body = serde_json::json!({"id":id,"skins":[{"state":"ACTIVE","variant":"CLASSIC","url":"https://textures.minecraft.net/texture/synthetic"}]}).to_string();
					tiny_http::Response::from_string(body).with_status_code(200)
				};
				request.respond(response).expect("mock response");
			}
		});
		let client = reqwest::Client::builder()
			.redirect(reqwest::redirect::Policy::none())
			.timeout(std::time::Duration::from_secs(2))
			.build()
			.unwrap();
		let token = "synthetic-skin-token";
		let expired =
			upload_microsoft_with_client(&store, &account, &id, token, &client, &endpoint)
				.await
				.err()
				.expect("rejected upload");
		assert!(expired.contains("истекла"));
		assert!(!expired.contains(token));
		assert!(history(&store, &account).await.unwrap().is_empty());
		mode.store(3, Ordering::SeqCst);
		let unavailable =
			upload_microsoft_with_client(&store, &account, &id, token, &client, &endpoint)
				.await
				.err()
				.expect("service unavailable");
		assert!(unavailable.contains("HTTP 503"));
		assert!(history(&store, &account).await.unwrap().is_empty());
		mode.store(2, Ordering::SeqCst);
		let mismatch =
			upload_microsoft_with_client(&store, &account, &id, token, &client, &endpoint)
				.await
				.err()
				.expect("rejected upload");
		assert!(mismatch.contains("другой профиль"));
		assert!(history(&store, &account).await.unwrap().is_empty());
		mode.store(4, Ordering::SeqCst);
		let unverified =
			upload_microsoft_with_client(&store, &account, &id, token, &client, &endpoint)
				.await
				.err()
				.expect("profile unavailable");
		assert!(unverified.contains("Не удалось проверить профиль"));
		assert!(history(&store, &account).await.unwrap().is_empty());
		mode.store(0, Ordering::SeqCst);
		let saved = upload_microsoft_with_client(&store, &account, &id, token, &client, &endpoint)
			.await
			.expect("verified upload");
		assert_eq!(saved[0].active_for, [account.as_str()]);
		assert_eq!(
			history(&store, &account).await.unwrap()[0]
				.skin_id
				.as_deref(),
			Some(id.as_str())
		);
		stop.store(true, Ordering::SeqCst);
		worker.join().expect("mock server join");
		store.pool.close().await;
		tokio::fs::remove_dir_all(directory).await.expect("cleanup");
	}
	#[cfg(unix)]
	#[tokio::test]
	async fn rejects_a_symlink_replacing_a_saved_skin() {
		let directory = std::env::temp_dir().join(format!("urod-skin-link-{}", Uuid::new_v4()));
		let store = Store::open(directory.clone()).await.unwrap();
		let png = BASE64_STANDARD.encode(include_bytes!("../assets/steve.png"));
		let saved = save(&store, "Link test", false, &png).await.unwrap();
		let id = &saved[0].id;
		let path = skin_path(&store, id).unwrap();
		tokio::fs::remove_file(&path).await.unwrap();
		std::os::unix::fs::symlink(
			Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/steve.png"),
			&path,
		)
		.unwrap();
		assert!(read_skin(&store, id).await.is_err());
		assert!(list(&store).await.unwrap()[0].preview.is_none());
		drop(store);
		let _ = tokio::fs::remove_dir_all(directory).await;
	}
}
