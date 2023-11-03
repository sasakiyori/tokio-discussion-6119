# tokio-discussion-6119

Reproduction for discussion: https://github.com/tokio-rs/tokio/discussions/6119

## Branches

- [main](https://github.com/sasakiyori/tokio-discussion-6119/tree/main): compile error
- [arc_mutex](https://github.com/sasakiyori/tokio-discussion-6119/tree/arc_mutex): use `Arc`+`Mutex` to solve problem
- [final_solution](https://github.com/sasakiyori/tokio-discussion-6119/tree/final_solution): use `Mutex`+`Option` to solve problem, the cost of `Mutex` is nearly zero
