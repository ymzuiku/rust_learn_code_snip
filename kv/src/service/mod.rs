use std::sync::Arc;

use tracing::debug;

use crate::{command_request::RequestData, *};

mod command_service;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<S = MemTable> {
    inner: Arc<ServiceInner<S>>,
}

impl<S> Clone for Service<S> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct ServiceInner<S> {
    store: S,
}

impl<S: Storage> Service<S> {
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got request: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        debug!("Executed response: {:?}", res);
        res
    }
}

pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(v)) => v.execute(store),
        Some(RequestData::Hgetall(v)) => v.execute(store),
        Some(RequestData::Hset(v)) => v.execute(store),
        None => KvError::InvalidCommand("Request has not data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    #[test]
    fn service_should_works() {
        let service = Service::new(MemTable::default());
        let cloned = service.clone();
        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &["v1".into()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

#[cfg(test)]
fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// #[cfg(test)]
// fn assert_res_error(mut res: CommandResponse, code: u32, msg: &str) {
//     assert_eq!(res.status, code);
//     // assert_eq!(res.message.contains(msg));
//     assert_eq!(res.values, &[]);
//     assert_eq!(res.pairs, &[]);
// }
