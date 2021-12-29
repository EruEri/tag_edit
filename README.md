# tag_edit
Personal project for editing tag from mp3 and flac files
*** 
A dependency-free library written in Rust that allows you to edit and create tag

Currently only ID3.v2.3 supported
***
## Examples

### Reading tag from mp3 file
```rust
use tag_edit::Metadata;


let mut metadata = Metadata::from_path("file_test/mp3/1-01 Dark seeks light.mp3").unwrap();
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

### Create an tag and replace the old tag
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


