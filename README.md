# 修正箇所
- hyper::body::to_bytes() -> http_body_util::BodyExt::collect().await.unwrap().to_bytes()
    - to_bytes() はメモリを無制限に使用される可能性があったため消された
- impl<T,S> FromRequest<S> for ValidateJson<T>
    - from_request の引数が req だけでなく state も必要になった