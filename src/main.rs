use ceph::ceph::*;
use ceph::cmd::{osd_pool_get, PoolOption};
use ceph::rados::{Struct_rados_cluster_stat_t, Struct_rados_pool_stat_t};
use std::str::FromStr;

#[derive(Debug)]
struct PoolInfo {
    pub name: String,
    pub usage: Struct_rados_pool_stat_t,
    pub pool_size: u32,
}

#[derive(Debug)]
struct UsageInfo {
    pub cluster_usage: Struct_rados_cluster_stat_t,
    pub pool_usage: Vec<PoolInfo>,
}

fn get_pool_size(handle: &Rados, pool: &str) -> Result<u32, String> {
    let pool_size_str =
        osd_pool_get(handle, &pool, &PoolOption::Size).map_err(|e| e.to_string())?;
    if let Some(s) = pool_size_str.split_whitespace().last() {
        let pool_size = u32::from_str(&s).map_err(|e| e.to_string())?;
        println!("pool_size: {}", pool_size);
        return Ok(pool_size);
    }
    Err(format!(
        "Invalid size string returned from librados: {}",
        pool_size_str,
    ))
}

fn main() {
    let mut pool_usage: Vec<PoolInfo> = Vec::new();
    let c = connect_to_ceph("admin", "/opt/ceph.conf");
    match c {
        Ok(c) => {
            let pools = c.rados_pools().map_err(|e| e.to_string()).unwrap();
            println!("pools: {:?}", pools);

            for p in pools {
                println!("Getting an ioctx to: {}", p);
                let i = c.get_rados_ioctx(&p).map_err(|e| e.to_string()).unwrap();
                println!("Running stat against the pool");
                let pool_stats = i.rados_stat_pool().map_err(|e| e.to_string()).unwrap();
                pool_usage.push(PoolInfo {
                    name: p.clone(),
                    usage: pool_stats,
                    pool_size: get_pool_size(&c, &p).unwrap(),
                });
            }
        }
        Err(e) => println!("{}", e),
    }
}
