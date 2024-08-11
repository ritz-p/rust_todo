# 修正箇所
- hyper::body::to_bytes() -> http_body_util::BodyExt::collect().await.unwrap().to_bytes()
    - to_bytes() はメモリを無制限に使用される可能性があったため消された
- impl<T,S> FromRequest<S> for ValidateJson<T>
    - from_request の引数が req だけでなく state も必要になった
    - fron_request 追っていったら state 使ってなくて草

# How to build
## 1. Environment
1. Build the Docker image and container in WSL with command below
```
docker-compose up -d --build
```
2. Enter container with command below
```
docker exec -it rust_todo bash
```

## 2. Backend
1. build the backend server with commands below
```
cargo make db_create
cargo make migrate
cargo make back
```

## 3. Frontend
1. build the frontend app on browser with commands below
```
cd frontend
yarn install(first time only)
trunk serve
```
2. build the frontend app on tauri with commands below
```
cd frontend
yarn install(first time only)
cargo tauri dev
```
3. build the standalone frontend app on tauri with commands below
```
cd frontend
yarn install(first time only)
cargo tauri build
```

## Todo
1. http request をいい感じにできるようにする
2. Tauri に書くべき部分とそうじゃない部分をわかりやすくする