use crate::{
    bytebuf::{packet_id::Packet, ByteBuffer},
    ClientPacket, VarInt,
};

pub struct CRegistryData<'a> {
    registry_id: &'a str,
    entries: &'a [RegistryEntry<'a>],
}

impl<'a> CRegistryData<'a> {
    pub fn new(registry_id: &'a str, entries: &'a [RegistryEntry]) -> Self {
        Self {
            registry_id,
            entries,
        }
    }
}

pub struct RegistryEntry<'a> {
    pub entry_id: &'a str,
    pub data: Vec<u8>,
}

impl<'a> Packet for CRegistryData<'a> {
    const PACKET_ID: VarInt = 0x07;
}

impl<'a> ClientPacket for CRegistryData<'a> {
    fn write(&self, bytebuf: &mut ByteBuffer) {
        bytebuf.put_string(self.registry_id);
        bytebuf.put_list::<RegistryEntry>(self.entries, |p, v| {
            p.put_string(v.entry_id);
            p.put_bool(!v.data.is_empty());
            p.put_slice(&v.data);
        });
    }
}
