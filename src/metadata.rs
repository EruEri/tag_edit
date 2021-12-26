//! Library's entry point.
//! Allow to read and write Tag metadata from MP3 and FLAC files

use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Error, Write};
use std::string::FromUtf8Error;
use crate::tag::audio_type::AudioType;
use crate::tag::audio_type::AudioType::{FLAC, MP3, OTHER};
use crate::tag::id3::id3_tag::ID3TAG;
use crate::tag::tag::Tag;
use super::tag::id3::id3_header_flag::ID3HeaderFLAG;
use super::util::function::unsynchsafe;

/// Tag's wrapper making the abstraction of the Tag source file (MP3, FLAC)
pub struct Metadata{
    filename : String,
    _file_type : AudioType,
    tag : Tag,
    music_data : Vec<u8>
}

fn is_id3(s: &String) -> bool {
    s == "ID3"
}

fn is_flac(s: &String) -> bool{
    s == "fLaC"
}

fn read_type_audio_file(file: &mut File) -> Result<(AudioType, usize), FromUtf8Error> {
    let mut buffer = [0,0,0,0,0,0,0,0,0,0];
    let _ = file.read(&mut buffer);
    //let flac = String::from_utf8(buffer[0..4].into_vec());
    let id3 = String::from_utf8(buffer[0..3].to_vec())?;
    if is_id3(&id3) { 
        let flag = buffer[5];
        let unsync_flag = ID3HeaderFLAG::Unsynchronisation as u8;
        let size = if (flag & unsync_flag) != unsync_flag {
            //println!("To unsynchsafe");
            unsynchsafe(u32::from_be_bytes(buffer[6..].try_into().unwrap()))
        }else {
            //println!("unsynchsafe");
            u32::from_be_bytes(buffer[6..].try_into().unwrap())
        };
        return Ok( (MP3, size as usize)); 
    }
    if is_flac(&String::from_utf8(buffer[0..4].to_vec())?){ return Ok( (FLAC, 0) );  }
    Ok((OTHER, 0))
}

impl Metadata {

    /// Retrieves the tag metadata from a file
    pub fn new(file_path: &str) -> Option<Self>{
        let mut file = OpenOptions::new().create(false).read(true).write(true).open(file_path).ok()?;
        let (audio_type, size) = read_type_audio_file(&mut file).ok()?;
        match audio_type {
            FLAC => todo!("FLac to implement"),
            MP3 => {
                let mut buffer = vec![0u8; size];
                let mut music_data = vec![];
                let _ = file.seek(SeekFrom::Start(0));
                
                let _ = file.read_exact(&mut buffer);
                let _ = file.read_to_end(&mut music_data);
                let tag = ID3TAG::new(&mut buffer).ok()?;
                Some( Metadata {
                    filename : file_path.to_string(),
                    _file_type: MP3,
                    tag : Tag::ID3(tag),
                    music_data
                }   )
            }
            OTHER => todo!("Other not implemented"),
        }
    }

    pub fn write_tag(&self, path : &str) -> Result<(), Error> {
        let mut file = OpenOptions::new()
        .create(true).read(false).write(true).truncate(true)
        .open(path)?;
        let _ = file.write(self.tag.as_bytes().as_slice())?;
        let _ = file.write(self.music_data.as_slice())?;
        Ok(())
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// Retrieves all the pictures contained in the tag. An empty `Vec`
    /// if the tag doesn'n contain any picture
    pub fn attached_pictures(&self) -> Vec<&Vec<u8>> {
        self.tag.attached_pictures()
    }
    /// Removes all the pictures contains in the tag
    /// 
    /// # Examples
    /// ```
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert!(!metadata.attached_pictures().is_empty());
    /// metadata.remove_all_attached_pictures();
    /// assert!(metadata.attached_pictures().is_empty())
    /// ```
    pub fn remove_all_attached_pictures(&mut self){
        self.tag.remove_all_attached_pictures()
    }
    /// Returns the song artist (TPE1)
    /// 
    /// # Examples
    /// ```
    /// use tag_editor::metadata::Metadata;
    /// let metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.artist(), Some("Maon Kurosaki".to_string()));
    /// 
    /// ```
    pub fn artist(&self) -> Option<String>{
        self.tag.artist()
    }
    /// Set the song artist (TPE1)
    /// 
    /// # Examples
    /// ```
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_artist("Song performer".into());
    /// assert_eq!(metadata.artist(), Some("Song performer".to_string()));
    /// 
    /// ```
    pub fn set_artist(&mut self, name : String) {
        self.tag.set_artist(name)
    } 
    /// Revome the song artist (TPE1)
    /// 
    /// # Examples
    /// ```
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.artist().is_some());
    /// metadata.remove_artist();
    /// assert!(metadata.artist().is_none());
    /// 
    /// ```
    pub fn remove_artist(&mut self) {
        self.tag.remove_artist()
    }
    /// Returns the album's artist (TPE2)
    /// 
    /// # Examples
    /// ```
    /// use tag_editor::metadata::Metadata;
    /// let metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.album_artist().unwrap(), "\u{feff}黒崎真音\u{0}".to_string());
    /// 
    /// ```
    pub fn album_artist(&self) -> Option<String> {
        self.tag.album_artist()
    }
    /// Set the album's artist (TPE2)
    /// 
    /// # Examples
    /// ```
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_artist("Artist album".to_string());
    /// assert_eq!(metadata.artist().unwrap(), "Artist album".to_string());
    /// 
    /// ```
    pub fn set_album_artist(&mut self, artist : String) {
        self.tag.set_album_artist(artist)
    }
    /// Remove the album artist (TPE2)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.album_artist().is_some());
    /// metadata.remove_album_artist();
    /// assert!(metadata.album_artist().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_album_artist(&mut self){
        self.tag.remove_album_artist()
    }
    /// Returns the album name (TABL)
    /// 
    /// # Example 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.album().unwrap(), "Butterfly Effect".to_string())
    /// ```
    pub fn album(&self) -> Option<String>{
        self.tag.album()
    }
    /// Set the album name (TALB)
    /// 
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_album("An album Name".into());
    /// assert_eq!(metadata.album().unwrap(), "An album Name".to_string());
    /// ```
    pub fn set_album(&mut self, album: String) {
        self.tag.set_album(album)
    }
    /// Remove the album  (TALB)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.album().is_some());
    /// metadata.remove_album();
    /// assert!(metadata.album().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_album(&mut self){
        self.tag.remove_album()
    }

    /// Returns the genre (TCON)
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.genre().unwrap(), "J-Pop".to_string());
    /// ```
    pub fn genre(&self) -> Option<String> {
        self.tag.genre()
    }
    /// Returns the genre (TCON)
    /// 
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_genre("A Genre".into());
    /// assert_eq!(metadata.genre().unwrap(), "A Genre".to_string());
    /// ```
    pub fn set_genre(&mut self, genre: String) {
        self.tag.set_genre(genre)
    }
    /// Remove the genre (TCON)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.genre().is_some());
    /// metadata.remove_genre();
    /// assert!(metadata.genre().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_genre(&mut self){
        self.tag.remove_genre()
    }
    /// Returns the publisher (TPUB)
    /// 
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// let publisher = metadata.publisher();
    /// assert!(publisher.is_none());
    /// ```
    pub fn publisher(&self) -> Option<String> {
        self.tag.publisher()
    }
    /// Set the publisher (TPUB)
    /// 
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_publisher("Some Publisher".into());
    /// assert!(metadata.publisher().is_some());
    /// ```
    pub fn set_publisher(&mut self, publisher : String) {
        self.tag.set_publisher(publisher)
    }

     /// Remove the publisher (TPUB)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_publisher();
    /// assert!(metadata.publisher().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_publisher(&mut self){
        self.tag.remove_publisher()
    }
    /// Returns the beats per minutes of the song (TBPM)
    /// 
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// let bpm = metadata.bpm();
    /// assert!(bpm.is_none())
    /// ```
    pub fn bpm(&self) -> Option<String> {
        self.tag.bpm()
    }
    /// Returns the beats per minutes of the song (TBPM)
    /// 
    /// # Examples 
    /// ``` 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_bpm(100);
    /// assert_eq!(metadata.bpm().unwrap().parse::<u16>().unwrap(), 100)
    /// ```
    pub fn set_bpm(&mut self, bpm : u16){
        self.tag.set_bpm(bpm)
    }
    /// Remove the track's BPM (TBPM)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_bpm();
    /// assert!(metadata.bpm().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_bpm(&mut self){
        self.tag.remove_bpm()
    }
    /// Returns the copyright message (TCOP)
    pub fn copyright(&self) -> Option<String> {
        self.tag.copyright()
    }
    pub fn date(&self) -> Option<String> {
        self.tag.date()
    }
    /// Remove the date (TDAT)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_date();
    /// assert!(metadata.date().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_date(&mut self){
        self.tag.remove_date()
    }
    pub fn encoded_by(&self) -> Option<String> {
        self.tag.encoded_by()
    }
    pub fn set_encoder(&mut self, encoder : String){
        self.tag.set_encoder(encoder)
    }
    /// Remove the encoder (TENC)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_encoder();
    /// assert!(metadata.encoded_by().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_encoder(&mut self){
        self.tag.remove_encoder()
    }
    pub fn file_type(&self) -> Option<String> {
        self.tag.file_type()   
    }
    pub fn time(&self) -> Option<String> {
        self.tag.time()
    }
    /// Remove the tracl's time recording (TIME)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_time();
    /// assert!(metadata.time().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_time(&mut self){
        self.tag.remove_time()
    }
    pub fn title(&self) -> Option<String> {
        self.tag.title()
    }
    pub fn set_title(&mut self, title : String){
        self.tag.set_title(title)
    }
    /// Remove the title (TIT2)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_title();
    /// assert!(metadata.title().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_title(&mut self){
        self.tag.remove_title()
    }
    pub fn music_len(&self) -> Option<usize> {
        self.tag.music_len()
    }
    /// Remove the music's length (TLEN)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_encoder();
    /// assert!(metadata.encoded_by().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_music_len(&mut self){
        self.tag.remove_music_len()
    }
    pub fn year(&self) -> Option<i16> {
        self.tag.year()
    }
    pub fn set_year(&mut self, year: i16){
        self.tag.set_year(year)
    }
    /// Remove the year (TYER)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_year();
    /// assert!(metadata.year().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_year(&mut self){
        self.tag.remove_year()
    }
    pub fn track_position(&self) -> Option<String> {
     self.tag.track_position()
    }
    pub fn set_track_position(&mut self, track_pos: u16, out_of: Option<u16>){
        self.tag.set_track_position(track_pos, out_of)
    }
    /// Remove the track position in the album (TRCK)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_track_position();
    /// assert!(metadata.track_position().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_track_position(&mut self){
        self.tag.remove_track_position()
    }
    pub fn album_part(&self) -> Option<String> {
        self.tag.album_part()
    }
    pub fn set_album_part(&mut self, album_part : u16, out_of: Option<u16> ){
        self.tag.set_album_part(album_part, out_of)
    }
    /// Remove the track's disc position (TPOS)
    /// 
    /// # Examples
    /// 
    /// ```
    /// 
    /// use tag_editor::metadata::Metadata;
    /// let mut metadata = Metadata::new("file_test/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_album_part();
    /// assert!(metadata.album_part().is_none());
    /// 
    /// 
    /// ```
    pub fn remove_album_part(&mut self){
        self.tag.remove_album_part()
    }
    pub fn lyrics(&self) -> Option<Vec<String>> {
        self.tag.lyrics()
    }
    pub fn comments(&self) -> Option<Vec<String>> {
        Some(
            self.tag.comments()?
            .iter()
            .map( |(_, text)| text.clone())
            .collect::<Vec<String>>()
        )
    }

}