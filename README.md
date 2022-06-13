# tag_edit

Personal project for editing tag from mp3 and flac files
***
A dependency-free library written in Rust that allows you to edit and create tag

Currently only ID3.v2.3 and Flac supported
***

## Examples

### Reading and writting tag from mp3 file

```rust
use tag_edit::ID3TAG;


let mut metadata = ID3TAG::from_path("file_test/mp3/1-01 Dark seeks light.mp3").unwrap();
if let Some(_artist) = metadata.artist(){
    // do something
}
if let Some(_year) = metadata.year(){
    // do somthing else
}

metadata.set_bpm(100);

metadata.set_album("An album".into());

let _ = metadata.overwrite_tag();

```

### Create an tag and replace the old mp3 tag

```rust
use tag_edit::ID3TagBuilder;
use tag_edit::ID3TEXTFRAMEID;

let mut tag_builder = ID3TagBuilder::new();
tag_builder
.set_artist("An artist")
.set_album("An album")
.add_text_frame(ID3TEXTFRAMEID::TIT2, "A title")
.replace_tag("file_path");
```

### Reading and writting tag from flac file

```rust
use tag_edit::FlacTag;
let mut flac_tag = FlacTag::from_path("file_test/flac/01. DO IT, DO IT (24bit-48kHz).flac").unwrap();
if let Some(_artist) = flac_tag.artist(){
    // do something
}

if let Some(_album) = flac_tag.album(){
    // do something else
}

flac_tag.set_disc(1);


let _ = flac_tag.overwrite_flac();

```

## Roadmap (at least try)

- [x] ID3.v2.3
- [x] FLAC
- [ ] Publish a crate version of the library
- [ ] ID3.v2.4
- [ ] M4A

## Warning

- This library is not very efficient since all the content of the audio file is loaded into a vector
