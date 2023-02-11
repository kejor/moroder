pub enum MidiMessage {
    KeyDown {
        channel: u8,
        key: u8,
        velocity: u8,
    },
    KeyUp {
        channel: u8,
        key: u8,
        velocity: u8,
    },
    ControllerChange {
        channel: u8,
        controller: u8,
        value: u8,
    },
}

impl From<Vec<u8>> for MidiMessage {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes.into_iter();
        let status_byte = bytes.next().unwrap();
        let channel = status_byte & 0b1111;
        let status = status_byte >> 4;

        match status {
            0b1000 => Self::KeyUp {
                channel,
                key: bytes.next().unwrap(),
                velocity: bytes.next().unwrap(),
            },
            0b1001 => Self::KeyDown {
                channel,
                key: bytes.next().unwrap(),
                velocity: bytes.next().unwrap(),
            },
            0b1011 => Self::ControllerChange {
                channel,
                controller: bytes.next().unwrap(),
                value: bytes.next().unwrap(),
            },
            _ => unimplemented!(),
        }
    }
}
