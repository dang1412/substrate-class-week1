# RPC, Frontend

## Rpc

`node/Cargo.toml`

```toml
jsonrpc-core = "18.0.0"
jsonrpc-derive = "18.0.0"
jsonrpc-core-client = "18.0.0"
```

Create `node/src/example_rpc.rs`

```rs
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait ExampleRpc {
    #[rpc(name = "example_value")]
    fn get_value(&self) -> Result<u32>;
}

pub struct Example;

impl ExampleRpc for Example {
    fn get_value(&self) -> Result<u32> {
        Ok(5)
    }
}
```

Add `node/src/lib.rs`

```rs
pub mod example_rpc;
```

Add `node/src/main.rs`

```rs
mod example_rpc;
```

Add `node/src/rpc.rs`

```rs
io.extend_with(crate::example_rpc::ExampleRpc::to_delegate(crate::example_rpc::Example{}));
```

Create `run.sh`

```sh
curl http://localhost:9933 -H "Content-type:application/json;charset=utf-8" -d '{"jsonrpc": "2.0", "id": 1, "method": "example_value", "params":[]}'
```
