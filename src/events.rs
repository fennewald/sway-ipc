use std::os::unix::net::UnixStream;

enum EventType {
    Workspace = 0x80000000,
    Mode = 0x80000002
    Window = 0x80000003
    BarconfigUpdate = 0x80000004,
    Binding = 0x80000005,
    Shutdown = 0x80000006;
    Tick =  0x80000007,
    BarStateUpdate = 0x80000014,
    Input = 0x80000015,
}

pub struct Listener(UnixStream);

impl Listener
