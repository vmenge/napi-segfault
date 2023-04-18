## About
Repo trying to reproduce following issues I've encountered with `napi-rs`:

- `SIGSEV` error
- `SIGABRT` error
- Threadsafe Function aborting lock with `PoisonError`

```
thread 'tokio-runtime-worker' panicked at 'Threadsafe Function aborted lock failed: PoisonError { .. }', /home/vmenge/.cargo/registry/src/github.com-1ecc6299db9ec823/napi-2.12.5/src/threadsafe_function.rs:123:8
```

## Running the project
- Install dependencies `yarn install`
- Build project `yarn build`
- Run project `ts-node segfault.ts`