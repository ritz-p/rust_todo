# 修正箇所
- hyper::body::to_bytes() -> http_body_util::BodyExt::collect()
    - to_bytes() はメモリを無制限に使用される可能性があったため消された