use rangerkit_core::SpiritManifest;

fn main() {
    println!("Hello, spirits!");

    let manifest = SpiritManifest::default();
    println!("There are {} spirits awaiting your call.", manifest.spirits.len());
}
