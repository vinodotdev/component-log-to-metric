/**********************************************
***** This file is generated, do not edit *****
***********************************************/

pub use vino_provider::prelude::*;

pub mod __multi__;
pub mod counter; // counter

type Result<T> = std::result::Result<T, WasmError>;

#[no_mangle]
pub(crate) extern "C" fn __guest_call(op_len: i32, req_len: i32) -> i32 {
    use std::slice;

    let buf: Vec<u8> = Vec::with_capacity(req_len as _);
    let req_ptr = buf.as_ptr();

    let opbuf: Vec<u8> = Vec::with_capacity(op_len as _);
    let op_ptr = opbuf.as_ptr();

    let (slice, op) = unsafe {
        wapc::__guest_request(op_ptr, req_ptr);
        (
            slice::from_raw_parts(req_ptr, req_len as _),
            slice::from_raw_parts(op_ptr, op_len as _),
        )
    };

    let op_str = ::std::str::from_utf8(op).unwrap();

    match Dispatcher::dispatch(op_str, slice) {
        Ok(response) => {
            unsafe { wapc::__guest_response(response.as_ptr(), response.len()) }
            1
        }
        Err(e) => {
            let errmsg = e.to_string();
            unsafe {
                wapc::__guest_error(errmsg.as_ptr(), errmsg.len() as _);
            }
            0
        }
    }
}

static ALL_COMPONENTS: &[&str] = &["counter"];

pub struct Dispatcher {}
impl Dispatch for Dispatcher {
    fn dispatch(op: &str, payload: &[u8]) -> CallResult {
        let payload = IncomingPayload::from_buffer(payload)?;
        let result = match op {
            "counter" => {
                crate::components::generated::counter::Component::default().execute(&payload)
            }
            _ => Err(WasmError::ComponentNotFound(
                op.to_owned(),
                ALL_COMPONENTS.join(", "),
            )),
        }?;
        Ok(serialize(&result)?)
    }
}

pub mod types {
    // no additional types
}

pub mod generated {
    use super::*;

    // start namespace
    pub mod counter {
        use crate::components::counter as implementation;

        pub use vino_provider::prelude::*;

        use super::*;

        #[derive(Default)]
        pub struct Component {}

        impl WapcComponent for Component {
            fn execute(&self, payload: &IncomingPayload) -> JobResult {
                let outputs = get_outputs(payload.id());
                let inputs = populate_inputs(payload)?;
                implementation::job(inputs, outputs)
            }
        }

        fn populate_inputs(payload: &IncomingPayload) -> Result<Inputs> {
            Ok(Inputs {
                message: deserialize(payload.get("message")?)?,
                summary_field: deserialize(payload.get("summary_field")?)?,
                tags: deserialize(payload.get("tags")?)?,
            })
        }

        impl From<Inputs> for TransportMap {
            fn from(inputs: Inputs) -> TransportMap {
                let mut map = TransportMap::new();
                map.insert("message", MessageTransport::success(&inputs.message));
                map.insert(
                    "summary_field",
                    MessageTransport::success(&inputs.summary_field),
                );
                map.insert("tags", MessageTransport::success(&inputs.tags));
                map
            }
        }

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub struct Inputs {
            #[serde(rename = "message")]
            pub message: serde_json::Value,
            #[serde(rename = "summary_field")]
            pub summary_field: String,
            #[serde(rename = "tags")]
            pub tags: Vec<String>,
        }

        #[derive(Debug)]
        pub struct OutputPorts {
            pub count: CountSender,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct CountSender {
            id: u32,
        }

        impl PortSender for CountSender {
            type PayloadType = u32;
            fn get_name(&self) -> String {
                "count".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }

        fn get_outputs(id: u32) -> OutputPorts {
            OutputPorts {
                count: CountSender { id },
            }
        }

        #[derive(Debug)]
        pub struct Outputs {
            packets: ProviderOutput,
        }

        impl Outputs {
            pub fn count(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("count")
                    .ok_or_else(|| ComponentError::new("No packets for port 'count' found"))?;
                Ok(PortOutput::new("count".to_owned(), packets))
            }
        }

        impl From<ProviderOutput> for Outputs {
            fn from(packets: ProviderOutput) -> Self {
                Self { packets }
            }
        }
    }

    pub mod __multi__ {
        use super::Result;
        use crate::components::__multi__ as implementation;

        #[cfg(any(feature = "native"))]
        pub use vino_provider::native::prelude::*;
        #[cfg(any(feature = "wasm"))]
        pub use vino_provider::wasm::prelude::*;

        pub use vino_provider::prelude::*;
        #[derive(Default)]
        pub struct Component {}

        impl WapcComponent for Component {
            fn execute(&self, payload: &IncomingPayload) -> JobResult {
                let outputs = get_outputs(payload.id());
                let inputs = populate_inputs(payload)?;
                implementation::job(inputs, outputs)
            }
        }

        fn populate_inputs(payload: &IncomingPayload) -> Result<Vec<ComponentInputs>> {
            Ok(deserialize(payload.get("inputs")?)?)
        }

        #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
        pub enum ComponentInputs {
            Counter(super::counter::Inputs),
        }

        #[cfg(all(feature = "guest"))]
        #[allow(missing_debug_implementations)]
        pub enum ComponentOutputs {
            Counter(super::counter::Outputs),
        }

        #[derive(Debug)]
        pub struct OutputPorts {
            pub result: ResultSender,
        }

        #[derive(Debug, PartialEq, Clone)]
        pub struct ResultSender {
            id: u32,
        }

        impl PortSender for ResultSender {
            type PayloadType = bool;
            fn get_name(&self) -> String {
                "result".to_string()
            }
            fn get_id(&self) -> u32 {
                self.id
            }
        }

        fn get_outputs(id: u32) -> OutputPorts {
            OutputPorts {
                result: ResultSender { id },
            }
        }

        #[derive(Debug)]
        pub struct Outputs {
            packets: ProviderOutput,
        }

        impl Outputs {
            pub fn result(&mut self) -> Result<PortOutput> {
                let packets = self
                    .packets
                    .take("result")
                    .ok_or_else(|| ComponentError::new("No packets for port 'result' found"))?;
                Ok(PortOutput::new("result".to_owned(), packets))
            }
        }

        impl From<ProviderOutput> for Outputs {
            fn from(packets: ProviderOutput) -> Self {
                Self { packets }
            }
        }
    }
}
