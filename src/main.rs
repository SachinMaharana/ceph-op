use ceph::ceph as ceph_helpers;
use ceph::ceph::*;
use ceph::rados::Struct_rados_cluster_stat_t;

#[derive(Debug)]
struct UsageInfo {
    pub cluster_usage: Struct_rados_cluster_stat_t,
}

fn main() {
    let c = connect_to_ceph("admin", "/opt/ceph.conf");

    match c {
        Ok(c) => {
            let rados_version = ceph_helpers::rados_libversion();
            println!("Librados version: {:?}", rados_version);

            println!("Getting cluster fsid");
            let fsid = c.rados_fsid();
            println!("rados_cluster_fsid {:?}", fsid.unwrap());

            let pools = c.rados_pools().unwrap();
            println!("pools: {:?}", pools);
        }
        Err(e) => println!("{}", e),
    }
}
