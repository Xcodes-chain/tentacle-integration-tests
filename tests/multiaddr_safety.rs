use std::borrow::Cow;

use tentacle::multiaddr::{Multiaddr, Protocol};

fn invalid_p2p_protocol() -> Protocol<'static> {
    Protocol::P2P(Cow::Owned(vec![0]))
}

#[test]
#[ignore = "PR #460 regression case; run explicitly on the target PR branch"]
#[should_panic(expected = "invalid p2p multihash bytes")]
fn invalid_p2p_is_rejected_by_safe_constructors() {
    let _ = Multiaddr::from(invalid_p2p_protocol());
}

#[test]
fn valid_p2p_round_trips_through_constructors() {
    let source = "/p2p/QmNQ4jky6uVqLDrPU7snqxARuNGWNLgSrTnssbRuy3ij2W";
    let mut parsed: Multiaddr = source.parse().unwrap();
    let p2p = parsed.pop().unwrap();

    assert_eq!(Multiaddr::from(p2p.clone()).to_string(), source);

    let mut pushed: Multiaddr = "/ip4/127.0.0.1/tcp/10000".parse().unwrap();
    pushed.push(p2p);
    assert!(pushed.to_string().ends_with(source));
}
