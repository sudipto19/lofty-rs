#![allow(missing_docs)]
//! Cross-platform tag creation benchmarks using Criterion
//! These complement the existing iai-callgrind benchmarks for Linux
//! and provide Windows/macOS developers the ability to run benchmarks locally

use lofty::ape::ApeTag;
use lofty::config::WriteOptions;
use lofty::id3::v1::Id3v1Tag;
use lofty::id3::v2::{FrameId, Id3v2Tag};
use lofty::iff::aiff::AiffTextChunks;
use lofty::iff::wav::RiffInfoList;
use lofty::mp4::Ilst;
use lofty::ogg::VorbisComments;
use lofty::picture::{MimeType, Picture, PictureType};
use lofty::tag::{Accessor, TagExt};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::borrow::Cow;

const ENCODER: &str = "Lavf57.56.101";

fn benchmark_aiff_text_chunks(c: &mut Criterion) {
    c.bench_function("aiff_text_chunks", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            let mut tag = AiffTextChunks::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

fn benchmark_apev2(c: &mut Criterion) {
    c.bench_function("apev2", |b| {
        b.iter(|| {
            use lofty::ape::ApeItem;
            use lofty::tag::ItemValue;

            let mut v = Vec::new();
            let mut tag = ApeTag::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));

            let picture = Picture::new_unchecked(
                PictureType::CoverFront,
                Some(MimeType::Jpeg),
                None,
                include_bytes!("./assets/cover.jpg").to_vec(),
            );

            tag.insert(
                ApeItem::new(
                    String::from("Cover (Front)"),
                    ItemValue::Binary(picture.as_ape_bytes()),
                )
                .unwrap(),
            );
            tag.insert(
                ApeItem::new(
                    String::from("Encoder"),
                    ItemValue::Text(String::from(ENCODER)),
                )
                .unwrap(),
            );
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

fn benchmark_id3v2(c: &mut Criterion) {
    c.bench_function("id3v2", |b| {
        b.iter(|| {
            use lofty::TextEncoding;
            use lofty::id3::v2::{Frame, TextInformationFrame};

            let mut v = Vec::new();
            let mut tag = Id3v2Tag::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));

            let picture = Picture::new_unchecked(
                PictureType::CoverFront,
                Some(MimeType::Jpeg),
                None,
                include_bytes!("./assets/cover.jpg").to_vec(),
            );

            tag.insert_picture(picture);
            tag.insert(Frame::Text(TextInformationFrame::new(
                FrameId::Valid(Cow::Borrowed("TSSE")),
                TextEncoding::Latin1,
                String::from(ENCODER),
            )));
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

fn benchmark_id3v1(c: &mut Criterion) {
    c.bench_function("id3v1", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            let mut tag = Id3v1Tag::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

fn benchmark_ilst(c: &mut Criterion) {
    c.bench_function("ilst", |b| {
        b.iter(|| {
            use lofty::mp4::{Atom, AtomData, AtomIdent};

            let mut v = Vec::new();
            let mut tag = Ilst::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));

            let picture = Picture::new_unchecked(
                PictureType::CoverFront,
                Some(MimeType::Jpeg),
                None,
                include_bytes!("./assets/cover.jpg").to_vec(),
            );

            tag.insert_picture(picture);
            tag.insert(Atom::new(
                AtomIdent::Fourcc(*b"\xa9too"),
                AtomData::UTF8(String::from(ENCODER)),
            ));
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

fn benchmark_riff_info(c: &mut Criterion) {
    c.bench_function("riff_info", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            let mut tag = RiffInfoList::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));
            tag.insert(String::from("ISFT"), String::from(ENCODER));
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

fn benchmark_vorbis_comments(c: &mut Criterion) {
    c.bench_function("vorbis_comments", |b| {
        b.iter(|| {
            use lofty::ogg::OggPictureStorage;

            let mut v = Vec::new();
            let mut tag = VorbisComments::default();

            tag.set_artist(String::from("Dave Eddy"));
            tag.set_title(String::from("TempleOS Hymn Risen (Remix)"));
            tag.set_album(String::from("Summer"));
            tag.set_year(2017);
            tag.set_track(1);
            tag.set_genre(String::from("Electronic"));

            let picture = Picture::new_unchecked(
                PictureType::CoverFront,
                Some(MimeType::Jpeg),
                None,
                include_bytes!("./assets/cover.jpg").to_vec(),
            );

            let _ = tag.insert_picture(picture, None).unwrap();
            tag.push(String::from("ENCODER"), String::from(ENCODER));
            
            black_box(tag.dump_to(&mut v, WriteOptions::default()).unwrap());
            black_box(v);
        })
    });
}

criterion_group!(
    benches,
    benchmark_aiff_text_chunks,
    benchmark_apev2,
    benchmark_id3v2,
    benchmark_id3v1,
    benchmark_ilst,
    benchmark_riff_info,
    benchmark_vorbis_comments
);
criterion_main!(benches);