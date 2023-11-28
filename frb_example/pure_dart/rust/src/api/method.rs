use flutter_rust_bridge::{spawn, StreamSink};

#[derive(Debug, Clone)]
pub struct Log2TwinNormal {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinNormal {
    pub a: String,
}

impl ConcatenateWith {
    pub fn new_twin_normal(a: String) -> ConcatenateWith {
        ConcatenateWith { a }
    }

    pub fn concatenate_twin_normal(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    pub fn concatenate_static_twin_normal(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    pub fn handle_some_stream_sink_twin_normal(&self, key: u32, max: u32, sink: StreamSink<Log2>) {
        let a = self.a.clone();
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: format!("{a}{i}"),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_stream_sink_at_1_twin_normal(&self, sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink_twin_normal(key: u32, max: u32, sink: StreamSink<Log2>) {
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: i.to_string(),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink_single_arg_twin_normal(sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }
}

pub struct SumWithTwinNormal {
    pub x: u32,
}

impl SumWith {
    pub fn sum_twin_normal(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

pub fn get_sum_struct_twin_normal() -> SumWith {
    SumWith { x: 21 }
}

pub fn get_sum_array_twin_normal(a: u32, b: u32, c: u32) -> [SumWith; 3] {
    [SumWith { x: a }, SumWith { x: b }, SumWith { x: c }]
}
