use std::num::ParseIntError;


pub fn split_vsock(addr: &String) -> Result<Option<(u32, u32)>, ParseIntError> {
    let opt = addr.split_once(':').map(|(cid, port)|{
       cid.parse::<u32>().and_then(|cid| {
          port.parse::<u32>().map(|port| (cid, port))
      })
    });
 
    opt.map_or(Ok(None), |r| r.map(Some))
 }