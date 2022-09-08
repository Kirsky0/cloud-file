fn main()
{
    let a: String = String::from("data:application/wasm;base64,AGFzbQEAAAAFg4CAgAABABAGkYCAgAACfwBBgIDAAAt/AEGAgMAACwelgICAAAMGbWVtb3J5AgAKX19kYXRhX2VuZAMAC19faGVhcF9iYXNlAwE=");
    let b = base64::decode(a).unwrap();
}
