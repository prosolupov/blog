## Blog Backend + Clients (Rust Workspace)

Monorepo с сервером (HTTP + gRPC), библиотекой клиента, CLI и WASM‑фронтендом.

### Архитектура и крейты
- `blog-server` — основной сервер: HTTP (Actix) + gRPC (tonic), Postgres, JWT, миграции.
- `blog-client` — библиотека‑клиент с двумя транспортами: HTTP (reqwest) и gRPC (tonic).
- `blog-cli` — CLI для сценариев регистрации/логина/CRUD через `blog-client`.
- `blog-wasm` — браузерный WASM‑фронтенд, работает только через HTTP API.

Связь:
- `blog-cli` зависит от `blog-client`.
- `blog-wasm` делает HTTP запросы напрямую (gRPC в браузере не используется).

---

## Зависимости и окружение

### PostgreSQL
Локально должен быть запущен Postgres. Пример:
- host: `localhost`
- port: `5432`
- db: `blog`
- user: `postgres`
- password: `pass`

### Переменные окружения
`blog-server` ожидает `.env` (в корне проекта) с параметрами:
```
DB_HOST=localhost
DB_PORT=5432
DB_NAME=blog
DB_USER=postgres
DB_PASSWORD=pass
TOKEN_KEY=your_jwt_secret
```

JWT‑ключ можно сгенерировать так:
```
openssl rand -hex 32
```

---

## Сборка и запуск

### Server (HTTP + gRPC)
```
cargo run -p blog-server
```

По умолчанию:
- HTTP: `http://127.0.0.1:8080`
- gRPC: `http://127.0.0.1:50051`

Миграции запускаются автоматически при старте сервера.

### Client library
```
cargo check -p blog-client
```

### CLI
```
cargo run -p blog-cli -- --help
```

### WASM (Frontend)
```
cd blog-wasm
wasm-pack build --target web
cd ..
python3 -m http.server 8000
```
Открыть `http://localhost:8000` (используется `index.html` в корне проекта).

---

## Примеры сценариев

### HTTP через curl

Регистрация:
```
curl -X POST http://127.0.0.1:8080/api/user/register \
  -H "Content-Type: application/json" \
  -d '{"username":"ivan","email":"ivan@example.com","password":"secret123"}'
```

Логин:
```
curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"ivan","password":"secret123"}'
```

Создание поста (с токеном):
```
curl -X POST http://127.0.0.1:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <ACCESS_TOKEN>" \
  -d '{"title":"Мой первый пост","content":"Содержание"}'
```

Список постов:
```
curl -X GET "http://127.0.0.1:8080/api/posts?page=1&per_page=20" \
  -H "Authorization: Bearer <ACCESS_TOKEN>"
```

Обновление:
```
curl -X PUT http://127.0.0.1:8080/api/posts/<ID> \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <ACCESS_TOKEN>" \
  -d '{"title":"Обновлённый заголовок","content":"Новый текст"}'
```

Удаление:
```
curl -X DELETE http://127.0.0.1:8080/api/posts/<ID> \
  -H "Authorization: Bearer <ACCESS_TOKEN>"
```

### CLI

Регистрация:
```
cargo run -p blog-cli -- register --username "ivan" --email "ivan@example.com" --password "secret123"
```

Логин:
```
cargo run -p blog-cli -- login --username "ivan" --password "secret123"
```

Создание поста:
```
cargo run -p blog-cli -- create --title "Мой первый пост" --content "Содержание"
```

Создание поста через gRPC:
```
cargo run -p blog-cli -- --grpc create --title "Мой первый пост" --content "Содержание"
```

Получение:
```
cargo run -p blog-cli -- get --id 1
```

Обновление:
```
cargo run -p blog-cli -- update --id 1 --title "Обновлённый заголовок" --content "Новый текст"
```

Удаление:
```
cargo run -p blog-cli -- delete --id 1
```

Список:
```
cargo run -p blog-cli -- list --limit 20 --offset 0
```

Токен сохраняется в файл `.blog_token`.

### WASM в браузере
1. Запустить сервер (`cargo run -p blog-server`).
2. Собрать wasm (`wasm-pack build --target web`).
3. Поднять локальный http server (`python3 -m http.server 8000`).
4. Открыть `http://localhost:8000`.
5. Зарегистрироваться или войти, затем создать пост.
