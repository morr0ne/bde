use bde::Value;

fn test_torrent(original_torrent: &[u8]) {
    let deserialized: Value =
        bde::from_bytes(original_torrent).expect("Failed to deserialize torrent");

    let serialized = bde::to_bytes(&deserialized).expect("Failed to serialize torrent");

    assert_eq!(original_torrent, serialized);
}

#[test]
fn archlinux() {
    test_torrent(include_bytes!("torrents/archlinux.torrent"))
}

#[test]
fn big_buck_bunny() {
    test_torrent(include_bytes!("torrents/big-buck-bunny.torrent"))
}

#[test]
fn debian() {
    test_torrent(include_bytes!("torrents/debian.torrent"))
}

#[test]
fn endeavour() {
    test_torrent(include_bytes!("torrents/endeavour.torrent"))
}
#[test]
fn fedora() {
    test_torrent(include_bytes!("torrents/fedora.torrent"))
}
#[test]
fn lubuntu() {
    test_torrent(include_bytes!("torrents/lubuntu.torrent"))
}
