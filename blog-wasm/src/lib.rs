use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

const TOKEN_KEY: &str = "blog_token";

#[wasm_bindgen]
pub struct BlogApp {
    server_url: String,
    token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<String>,
    title: String,
    content: String,
    author_id: Option<String>,
}

#[derive(Serialize)]
struct RegisterRequest<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct CreatePostRequest<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct RefreshRequest<'a> {
    refresh_token: &'a str,
}

#[derive(Deserialize)]
struct PostResponse {
    id: String,
    title: String,
    content: String,
    author_id: String,
}

#[wasm_bindgen]
impl BlogApp {
    #[wasm_bindgen(constructor)]
    pub fn new(server_url: Option<String>) -> BlogApp {
        let server_url = server_url.unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
        let token = get_token_from_storage();
        BlogApp { server_url, token }
    }

    #[wasm_bindgen]
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    #[wasm_bindgen]
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token.clone();
        if let Some(value) = token {
            let _ = save_token_to_storage(&value);
        }
    }

    #[wasm_bindgen]
    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    #[wasm_bindgen]
    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<JsValue, JsValue> {
        let url = format!("{}/api/user/register", self.server_url);
        let body = RegisterRequest {
            username: &username,
            email: &email,
            password: &password,
        };

        let resp = Request::post(&url)
            .json(&body)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        ensure_ok(resp).await?;

        // login to get token
        self.login(username, password).await
    }

    #[wasm_bindgen]
    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<JsValue, JsValue> {
        let url = format!("{}/api/auth/login", self.server_url);
        let body = LoginRequest {
            username: &username,
            password: &password,
        };

        let resp = Request::post(&url)
            .json(&body)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let resp = ensure_ok(resp).await?;
        let auth: AuthResponse = resp.json().await.map_err(js_error)?;
        self.token = Some(auth.access_token.clone());
        save_token_to_storage(&auth.access_token)?;
        serde_wasm_bindgen::to_value(&auth).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn load_posts(&self) -> Result<JsValue, JsValue> {
        let url = format!("{}/api/posts?page=1&per_page=20", self.server_url);
        let mut req = Request::get(&url);
        if let Some(token) = &self.token {
            req = req.header("Authorization", &format!("Bearer {token}"));
        }

        let resp = req.send().await.map_err(js_error)?;
        let resp = ensure_ok(resp).await?;
        let posts: Vec<PostResponse> = resp.json().await.map_err(js_error)?;

        let mapped: Vec<Post> = posts
            .into_iter()
            .map(|p| Post {
                id: Some(p.id),
                title: p.title,
                content: p.content,
                author_id: Some(p.author_id),
            })
            .collect();

        serde_wasm_bindgen::to_value(&mapped).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn create_post(
        &self,
        title: String,
        content: String,
    ) -> Result<JsValue, JsValue> {
        let token = self.token.as_ref().ok_or_else(|| {
            JsValue::from_str("Unauthorized: missing token")
        })?;

        let url = format!("{}/api/posts", self.server_url);
        let body = CreatePostRequest {
            title: &title,
            content: &content,
        };

        let resp = Request::post(&url)
            .header("Authorization", &format!("Bearer {token}"))
            .json(&body)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let resp = ensure_ok(resp).await?;
        let post: PostResponse = resp.json().await.map_err(js_error)?;
        let mapped = Post {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author_id: Some(post.author_id),
        };

        serde_wasm_bindgen::to_value(&mapped).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn update_post(
        &self,
        id: String,
        title: String,
        content: String,
    ) -> Result<JsValue, JsValue> {
        let token = self.token.as_ref().ok_or_else(|| {
            JsValue::from_str("Unauthorized: missing token")
        })?;

        let url = format!("{}/api/posts/{}", self.server_url, id);
        let body = CreatePostRequest {
            title: &title,
            content: &content,
        };

        let resp = Request::put(&url)
            .header("Authorization", &format!("Bearer {token}"))
            .json(&body)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let resp = ensure_ok(resp).await?;
        let post: PostResponse = resp.json().await.map_err(js_error)?;
        let mapped = Post {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author_id: Some(post.author_id),
        };

        serde_wasm_bindgen::to_value(&mapped).map_err(js_error)
    }

    #[wasm_bindgen]
    pub async fn delete_post(&self, id: String) -> Result<JsValue, JsValue> {
        let token = self.token.as_ref().ok_or_else(|| {
            JsValue::from_str("Unauthorized: missing token")
        })?;

        let url = format!("{}/api/posts/{}", self.server_url, id);
        let resp = Request::delete(&url)
            .header("Authorization", &format!("Bearer {token}"))
            .send()
            .await
            .map_err(js_error)?;

        ensure_ok(resp).await?;
        Ok(JsValue::from_bool(true))
    }

    #[wasm_bindgen]
    pub async fn refresh_token(&self, refresh_token: String) -> Result<JsValue, JsValue> {
        let url = format!("{}/api/auth/refresh", self.server_url);
        let body = RefreshRequest {
            refresh_token: &refresh_token,
        };

        let resp = Request::post(&url)
            .json(&body)
            .map_err(js_error)?
            .send()
            .await
            .map_err(js_error)?;

        let resp = ensure_ok(resp).await?;
        let auth: AuthResponse = resp.json().await.map_err(js_error)?;
        serde_wasm_bindgen::to_value(&auth).map_err(js_error)
    }
}

fn save_token_to_storage(token: &str) -> Result<(), JsValue> {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        storage.set_item(TOKEN_KEY, token)?;
    }
    Ok(())
}

fn get_token_from_storage() -> Option<String> {
    window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|storage| storage.get_item(TOKEN_KEY).ok().flatten())
}

fn js_error<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}

async fn ensure_ok(resp: gloo_net::http::Response) -> Result<gloo_net::http::Response, JsValue> {
    if resp.ok() {
        Ok(resp)
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(JsValue::from_str(&format!("HTTP {status}: {text}")))
    }
}
