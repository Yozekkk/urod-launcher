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
	let metadata = tokio::fs::metadata(&path)
		.await
		.map_err(|_| "Скин не найден")?;
	if !metadata.is_file() || metadata.len() > MAX_PNG_BYTES as u64 {
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
pub async fn rename(store: &Store, id: &str, name: &str) -> Result<Vec<SavedSkin>> {
	let name = valid_name(name)?;
	let mut skins = index(store).await?;
	let item = skins
		.iter_mut()
		.find(|item| item.id == id)
		.ok_or("Скин не найден")?;
	item.name = name;
	persist(store, &skins).await?;
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
	list(store).await
}
pub async fn reset_local(store: &Store, account: &str) -> Result<Vec<SavedSkin>> {
	let mut skins = index(store).await?;
	for item in &mut skins {
		item.active_for.retain(|value| value != account);
	}
	persist(store, &skins).await?;
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
	let (bytes, slim) = bytes(store, id).await?;
	let client = reqwest::Client::builder()
		.https_only(true)
		.redirect(reqwest::redirect::Policy::none())
		.timeout(std::time::Duration::from_secs(20))
		.build()
		.map_err(|_| "Не удалось подготовить соединение Minecraft")?;
	let boundary = format!("urod-{}", Uuid::new_v4());
	let mut body = format!("--{boundary}\r\nContent-Disposition: form-data; name=\"variant\"\r\n\r\n{}\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"skin.png\"\r\nContent-Type: image/png\r\n\r\n", if slim { "slim" } else { "classic" }).into_bytes();
	body.extend_from_slice(&bytes);
	body.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
	let endpoint = "https://api.minecraftservices.com/minecraft/profile";
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
		let png = BASE64_STANDARD.encode(include_bytes!("../assets/steve.png"));
		let first = save(&store, "Первый", false, &png).await.unwrap();
		let first_id = first[0].id.clone();
		let second = save(&store, "Второй", true, &png).await.unwrap();
		let second_id = second[0].id.clone();
		assert_eq!(second.len(), 2);
		apply_local(&store, "offline-test", &first_id)
			.await
			.unwrap();
		assert!(local_skin(&store, "offline-test").await.unwrap().is_some());
		apply_local(&store, "offline-test", &second_id)
			.await
			.unwrap();
		let reloaded = list(&store).await.unwrap();
		assert_eq!(
			reloaded
				.iter()
				.filter(|item| item.active_for.len() == 1)
				.count(),
			1
		);
		assert_eq!(reloaded[0].id, second_id);
		assert_eq!(reloaded[0].active_for, ["offline-test"]);
		reset_local(&store, "offline-test").await.unwrap();
		assert!(local_skin(&store, "offline-test").await.unwrap().is_none());
		remove(&store, &first_id).await.unwrap();
		assert_eq!(list(&store).await.unwrap().len(), 1);
		drop(store);
		let _ = tokio::fs::remove_dir_all(directory).await;
	}
}
