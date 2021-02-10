use crate::error::Error;
use crate::{
    crypto::CryptoCore,
    engine::{Hash, PeerData, TimeSource},
    messages::NodeInfo,
    table::ClaimTable,
    traffic::TrafficStats,
    types::{Address, NodeId, RangeList},
    util::MsgBuffer
};
use parking_lot::Mutex;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

pub struct SharedPeerCrypto {
    peers: Arc<Mutex<HashMap<SocketAddr, Option<Arc<CryptoCore>>, Hash>>>
}

impl SharedPeerCrypto {
    pub fn sync(&mut self) {
        // TODO sync if needed
    }

    pub fn encrypt_for(&mut self, peer: SocketAddr, data: &mut MsgBuffer) -> Result<(), Error> {
        let mut peers = self.peers.lock();
        match peers.get_mut(&peer) {
            None => Err(Error::InvalidCryptoState("No crypto found for peer")),
            Some(None) => Ok(()),
            Some(Some(crypto)) => Ok(crypto.encrypt(data))
        }       
    }

    pub fn for_each(&mut self, mut callback: impl FnMut(SocketAddr, Option<Arc<CryptoCore>>) -> Result<(), Error>) -> Result<(), Error> {
        let mut peers = self.peers.lock();
        for (k, v) in peers.iter_mut() {
            callback(*k, v.clone())?
        }
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.peers.lock().len()
    }
}


pub struct SharedTraffic {
    traffic: Arc<Mutex<TrafficStats>>
}

impl SharedTraffic {
    pub fn sync(&mut self) {
        // TODO sync if needed
    }

    pub fn count_out_traffic(&self, peer: SocketAddr, bytes: usize) {
        self.traffic.lock().count_out_traffic(peer, bytes);
    }

    pub fn count_in_traffic(&self, peer: SocketAddr, bytes: usize) {
        self.traffic.lock().count_in_traffic(peer, bytes);
    }

    pub fn count_out_payload(&self, remote: Address, local: Address, bytes: usize) {
        self.traffic.lock().count_out_payload(remote, local, bytes);
    }

    pub fn count_in_payload(&self, remote: Address, local: Address, bytes: usize) {
        self.traffic.lock().count_in_payload(remote, local, bytes);
    }

    pub fn count_dropped_payload(&self, bytes: usize) {
        self.traffic.lock().count_dropped_payload(bytes);
    }

    pub fn count_invalid_protocol(&self, bytes: usize) {
        self.traffic.lock().count_invalid_protocol(bytes);
    }
}


pub struct SharedTable<TS: TimeSource> {
    table: Arc<Mutex<ClaimTable<TS>>>
}

impl<TS: TimeSource> SharedTable<TS> {
    pub fn sync(&mut self) {
        // TODO sync if needed
    }

    pub fn lookup(&self, addr: Address) -> Option<SocketAddr> {
        self.table.lock().lookup(addr)
    }

    pub fn set_claims(&self, peer: SocketAddr, claims: RangeList) {
        self.table.lock().set_claims(peer, claims)
    }

    pub fn remove_claims(&self, peer: SocketAddr) {
        self.table.lock().remove_claims(peer)
    }

    pub fn cache(&self, addr: Address, peer: SocketAddr) {
        self.table.lock().cache(addr, peer)
    }
}
