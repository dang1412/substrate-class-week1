# RPC, Frontend

## Example Rpc

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

## Rpc for pallet

Create packages

- `pallet-template-rpc`
- `pallet-template-rpc-runtime-api`

Define api in `pallet-template-rpc-runtime-api`

```rs
#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    pub trait SumStorageApi {
        fn get_sum() -> u32;
    }
}
```

`pallet-template-rpc`

```rs
// use runtime api
use pallet_template_rpc_runtime_api::SumStorageApi as SumStorageRuntimeApi;

// define api
#[rpc]
pub trait SumStorageApi<BlockHash> {
    #[rpc(name = "sumStorage_get")]
    fn get_sum(&self, at: Option<BlockHash>) -> Result<u32>;
}

// struct
pub struct SumStorage<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

// implement
impl<C, Block> SumStorageApi<<Block as BlockT>::Hash>
    for SumStorage<C, Block>
where
    Block: BlockT,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: SumStorageRuntimeApi<Block>,    // use trait defined in runtime api
{
    fn get_sum(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<u32> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let result_api = api.get_sum(&at);

        // if error map to RpcError
        result_api.map_err(|e| RpcError {
            code: ErrorCode::ServerError(Error::RuntimeError.into()),
            message: "Unable to query dispatch info.".into(),
            data: Some(e.to_string().into()),
        })
    }
}
```

Update `node/src/rpc.rs`

```rs
//...
C::Api: pallet_template_rpc_runtime_api::SumStorageApi<Block>,
//...
io.extend_with(pallet_template_rpc::SumStorageApi::to_delegate(pallet_template_rpc::SumStorage::new(client.clone())));
```

Update `node/Cargo.toml`

```toml
pallet-template-rpc = { version = "4.0.0-dev", path = "../pallets/template/rpc" }
pallet-template-rpc-runtime-api = { version = "4.0.0-dev", path = "../pallets/template/rpc/runtime-api" }
```

Update `pallets/template/src/lib.rs`

```rs
    impl<T: Config> Pallet<T> {
        pub fn sum_storage() -> u32 {
            Something::<T>::get().unwrap() + Something2::<T>::get().unwrap()
        }
    }
```

Implement runtime `runtime/src/lib.rs`

```rs
    impl pallet_template_rpc_runtime_api::SumStorageApi<Block> for Runtime {
        fn get_sum() -> u32 {
            TemplateModule::sum_storage()
        }
    }
```

Update `runtime/Cargo.toml`

```toml
pallet-template-rpc-runtime-api = { version = "4.0.0-dev", default-features = false, path = "../pallets/template/rpc/runtime-api" }

"pallet-template-rpc-runtime-api/std",
```
