
namespace rs kvs

struct BaseResponse {
    1: required i32 status_code,
    2: required string msg,
}

struct GetRequest {
    1: required string key,
}

struct GetResponse {
    1: optional string value,

    10: BaseResponse BaseResponse,
}

struct RemoveRequest {
    1: required string key,
}

struct RemoveResponse {
    10: BaseResponse BaseResponse,
}

struct SetRequest {
    1: required string key,
    2: required string value,
}

struct SetResponse {
    10: BaseResponse BaseResponse,
}

service KvsService {
    GetResponse Get(1: GetRequest req),

    SetResponse Set(1: SetRequest req),

    RemoveResponse Remove(1: RemoveRequest req),
}