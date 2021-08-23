use usbenum::list_connected_usb_drives;

fn main() {
    match list_connected_usb_drives() {
        Ok(l) => {
            if l.is_empty() {
                println!("No currently connected usb drives")
            } else {
                println!("{:#?}", l)
            }
        },
        Err(e) => println!("{:?}", e),
    }
}
