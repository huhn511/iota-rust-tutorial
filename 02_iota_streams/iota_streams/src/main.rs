use failure::{ensure, Fallible};
use iota_streams_app::message::HasLink;
use iota_streams_app_channel::{
    api::tangle::{Author, Subscriber},
    message,
};
use iota_streams_core::tbits::Tbits;
use iota_streams_protobuf3::types::Trytes;
use std::str::FromStr;

fn example() -> Fallible<()> {
    let mut author = Author::new("AGDXXACSRTFNNJAFT9TNAABRUHRKUEHZALMUVQ9Z9UNO9VCGN9NJIYVFFXQ9QHEKSN9DNEVGFGYQOB9MV", 2, false);
    let mut subscriber = Subscriber::new("SGDXXACSRTFNNJAFT9TNAABRUHRKUEHZALMUVQ9Z9UNO9VCGN9NJIYVFFXQ9QHEKSN9DNEVGFGYQOB9MA", false);

    println!("announce");
    let announcement = author.announce()?;

    {
        let preparsed = announcement.parse_header()?;
        ensure!(preparsed.check_content_type(message::announce::TYPE));
        subscriber.unwrap_announcement(preparsed)?;
        ensure!(subscriber
            .channel_address()
            .map_or(false, |appinst| appinst == announcement.link().base()));
        ensure!(subscriber
            .author_mss_public_key()
            .as_ref()
            .map_or(false, |pk| pk.tbits() == announcement.link().base().tbits()));
    }

    println!("share keyload for everyone");
    let keyload = author.share_keyload_for_everyone(announcement.link())?;

    {
        let preparsed = keyload.parse_header()?;
        ensure!(preparsed.check_content_type(message::keyload::TYPE));
        subscriber.unwrap_keyload(preparsed)?;
    }

    let public_payload = Trytes(Tbits::from_str("PUBLICPAYLOAD").unwrap());
    let masked_payload = Trytes(Tbits::from_str("MASKEDPAYLOAD").unwrap());

    println!("sign packet");
    let signed_packet =
        author.sign_packet(announcement.link(), &public_payload, &masked_payload)?;

    {
        let preparsed = signed_packet.parse_header()?;
        ensure!(preparsed.check_content_type(message::signed_packet::TYPE));
        let (unwrapped_public, unwrapped_masked) = subscriber.unwrap_signed_packet(preparsed)?;
        ensure!(public_payload == unwrapped_public);
        ensure!(masked_payload == unwrapped_masked);
    }

    println!("tag packet");
    let tagged_packet = author.tag_packet(announcement.link(), &public_payload, &masked_payload)?;

    {
        let preparsed = tagged_packet.parse_header()?;
        ensure!(preparsed.check_content_type(message::tagged_packet::TYPE));
        let (unwrapped_public, unwrapped_masked) = subscriber.unwrap_tagged_packet(preparsed)?;
        ensure!(public_payload == unwrapped_public);
        ensure!(masked_payload == unwrapped_masked);
    }

    /*
    println!("change key");
    let change_key = author.change_key(announcement.link())?;
    {
        println!("change key try parse header");
        let preparsed = change_key.parse_header()?;
        ensure!(preparsed.check_content_type(message::change_key::TYPE));
        println!("unwrap change key");
        subscriber.unwrap_change_key(preparsed)?;
    }
     */

    Ok(())
}

fn main() {
    let _r = dbg!(example());
}