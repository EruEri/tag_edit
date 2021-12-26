//! A library to read and write tag from audio files.
//! Currently, only ID3v2.3 supported


mod tag;
mod util;
pub mod metadata;



#[cfg(test)]
mod test {
    const INPUT_FILE : &'static str = "file_test/02 VANISHING POINT.mp3";

    use crate::metadata::Metadata;

    #[test]
    fn it_work(){
        assert_eq!(true, !false);
    }

    // #[test]
    // fn check_size_integrity(){
    //     if let Some(mut metadata) = Metadata::new(INPUT_FILE){
    //         //metadata.set_artist("Foo".into());
    //         //metadata.set_bpm(97);
    //         //metadata.set_publisher("BAR".into());
    //         assert_eq!(metadata.tag().get_size() as usize, metadata.tag().as_bytes().len())
    //     }else {
    //         panic!("Cannot create metadata")
    //     }
    // }
    #[test]
    fn comment_frame_test(){
        if let Some(metadata) = Metadata::new(INPUT_FILE){
            //metadata.set_artist("Foo".into());
            //metadata.set_bpm(97);
            //metadata.set_publisher("BAR".into());
            assert_eq!(metadata.artist(), Some("Maon Kurosaki".to_string()));
            assert_eq!(metadata.tag().get_size() as usize, metadata.tag().as_bytes().len())
        }else {
            panic!("Cannot create metadata")
        }
    }
    
}