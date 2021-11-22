use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ID3FRAMEID {
   APIC,
// [[#sec4.20|Audio encryption]]
   AENC,
//[#sec4.11 Comments]
   COMM,
//[#sec4.25 Commercial frame]
   COMR,
//[#sec4.26 Encryption method registration]
   ENCR,
//[#sec4.13 Equalization]
   EQUA,
//[#sec4.6 Event timing codes]
   ETCO,
//[#sec4.16 General encapsulated object]
   GEOB,
//[#sec4.27 Group identification registration]
   GRID,
//[#sec4.4 Involved people list]
   IPLS,
//[#sec4.21 Linked information]
   LINK,
//[#sec4.5 Music CD identifier]
   MCDI,
//[#sec4.7 MPEG location lookup table]
   MLLT,
//[#sec4.24 Ownership frame]
   OWNE,
//[#sec4.17 Play counter]
   PRIV,
//[#sec4.18 Popularimeter]
   PCNT,
//[#sec4.22 Position synchronisation frame]
   POPM,
   POSS,
   RBUF,    //[#sec4.19 Recommended buffer size]
   RVAD,    //[#sec4.12 Relative volume adjustment]
   RVRB,    //[#sec4.14 Reverb]
   SYLT,    //[#sec4.10 Synchronized lyric/text]
   SYTC,    //[#sec4.8 Synchronized tempo codes]
   TCMP,
   TALB,    //[#TALB Album/Movie/Show title]
   TBPM,    //[#TBPM BPM (beats per minute)]
   TCOM,    //[#TCOM Composer]
   TCON,    //[#TCON Content type]
   TCOP,    //[#TCOP Copyright message]
   TDAT,    //[#TDAT Date]
   TDLY,    //[#TDLY Playlist delay]
   TENC,    //[#TENC Encoded by]
   TEXT,    //[#TEXT Lyricist/Text writer]
   TFLT,    //[#TFLT File type]
   TIME,    //[#TIME Time]
   TIT1,    //[#TIT1 Content group description]
   TIT2,    //[#TIT2 Title/songname/content description]
   TIT3,    //[#TIT3 Subtitle/Description refinement]
   TKEY,    //[#TKEY Initial key]
   TLAN,    //[#TLAN Language(s)]
   TLEN,    //[#TLEN Length]
   TSOT,    // Extra
   TSO2,
   TMED,    //[#TMED Media type]
   TOAL,    //[#TOAL Original album/movie/show title]
   TOFN,    //[#TOFN Original filename]
   TOLY,    //[#TOLY Original lyricist(s)/text writer(s)]
   TOPE,    //[#TOPE Original artist(s)/performer(s)]
   TORY,    //[#TORY Original release year]
   TOWN,    //[#TOWN File owner/licensee]
   TPE1,    //[#TPE1 Lead performer(s)/Soloist(s)]
   TPE2,    //[#TPE2 Band/orchestra/accompaniment]
   TPE3,    //[#TPE3 Conductor/performer refinement]
   TPE4,    //[#TPE4 Interpreted, remixed, or otherwise modified by]
   TPOS,    //[#TPOS Part of a set]
   TPUB,    //[#TPUB Publisher]
   TRCK,    //[#TRCK Track number/Position in set]
   TRDA,    //[#TRDA Recording dates]
   TRSN,    //[#TRSN Internet radio station name]
   TRSO,    //[#TRSO Internet radio station owner]
   TSIZ,
//[#TSIZ Size]
   TSRC,
//[#TSRC ISRC (international standard recording code)]
   TSSE,
//[#TSEE Software/Hardware and settings used for encoding]
   TYER,
//[#TYER Year]
   TXXX,
//[#TXXX User defined text information frame]
   UFID,
//[#sec4.1 Unique file identifier]
   USER,
//[#sec4.23 Terms of use]
   USLT,
//[#sec4.9 Unsychronized lyric/text transcription]
   WCOM,
//[#WCOM Commercial information]
   WCOP,
//[#WCOP Copyright/Legal information]
   WOAF,
//[#WOAF Official audio file webpage]
   WOAR,
//[#WOAS Official audio source webpage]
   WOAS,
//[#WOAR Official artist/performer webpage]
   WORS,
//[#WPAY Payment]
   WPAY,
//[#WPUB Publishers official webpage]
   WPUB,
//[#WXXX User defined URL link frame]
   WXXX
}

impl FromStr for ID3FRAMEID {
   type Err = ();

   fn from_str(s: &str) -> Result<Self, Self::Err> {

      match s {
         "APIC" => Ok(ID3FRAMEID::APIC),
         "AENC" => Ok(ID3FRAMEID::AENC),
         "COMM" => Ok(ID3FRAMEID::COMM),
         "COMR" => Ok(ID3FRAMEID::COMR),
         "ENCR" => Ok(ID3FRAMEID::ENCR),
         "EQUA" => Ok(ID3FRAMEID::EQUA),
         "ETCO" => Ok(ID3FRAMEID::ETCO),
         "GEOB" => Ok(ID3FRAMEID::GEOB),
         "GRID" => Ok(ID3FRAMEID::GRID),
         "IPLS" => Ok(ID3FRAMEID::IPLS),
         "LINK" => Ok(ID3FRAMEID::LINK),
         "MCDI" => Ok(ID3FRAMEID::MCDI),
         "MLLT" => Ok(ID3FRAMEID::MLLT),
         "OWNE" => Ok(ID3FRAMEID::OWNE),
         "PRIV" => Ok(ID3FRAMEID::PRIV),
         "PCNT" => Ok(ID3FRAMEID::PCNT),
         "POPM" => Ok(ID3FRAMEID::POPM),
         "POSS" => Ok(ID3FRAMEID::POSS),
         "RBUF" => Ok(ID3FRAMEID::RBUF),
         "RVAD" => Ok(ID3FRAMEID::RVAD),
         "RVRB" => Ok(ID3FRAMEID::RVRB),
         "SYLT" => Ok(ID3FRAMEID::SYLT),
         "SYTC" => Ok(ID3FRAMEID::SYTC),
         "TCMP" => Ok(ID3FRAMEID::TCMP),
         "TALB" => Ok(ID3FRAMEID::TALB),
         "TBPM" => Ok(ID3FRAMEID::TBPM),
         "TCOM" => Ok(ID3FRAMEID::TCOM),
         "TCON" => Ok(ID3FRAMEID::TCON),
         "TCOP" => Ok(ID3FRAMEID::TCOP),
         "TDAT" => Ok(ID3FRAMEID::TDAT),
         "TDLY" => Ok(ID3FRAMEID::TDLY),
         "TENC" => Ok(ID3FRAMEID::TENC),
         "TEXT" => Ok(ID3FRAMEID::TEXT),
         "TFLT" => Ok(ID3FRAMEID::TFLT),
         "TIME" => Ok(ID3FRAMEID::TIME),
         "TIT1" => Ok(ID3FRAMEID::TIT1),
         "TIT2" => Ok(ID3FRAMEID::TIT2),
         "TIT3" => Ok(ID3FRAMEID::TIT3),
         "TKEY" => Ok(ID3FRAMEID::TKEY),
         "TLAN" => Ok(ID3FRAMEID::TLAN),
         "TLEN" => Ok(ID3FRAMEID::TLEN),
         "TSOT" => Ok(ID3FRAMEID::TSOT),
         "TSO2" => Ok(ID3FRAMEID::TSO2),
         "TMED" => Ok(ID3FRAMEID::TMED),
         "TOAL" => Ok(ID3FRAMEID::TOAL),
         "TOFN" => Ok(ID3FRAMEID::TOFN),
         "TOLY" => Ok(ID3FRAMEID::TOLY),
         "TOPE" => Ok(ID3FRAMEID::TOPE),
         "TORY" => Ok(ID3FRAMEID::TORY),
         "TOWN" => Ok(ID3FRAMEID::TOWN),
         "TPE1" => Ok(ID3FRAMEID::TPE1),
         "TPE2" => Ok(ID3FRAMEID::TPE2),
         "TPE3" => Ok(ID3FRAMEID::TPE3),
         "TPE4" => Ok(ID3FRAMEID::TPE4),
         "TPOS" => Ok(ID3FRAMEID::TPOS),
         "TPUB" => Ok(ID3FRAMEID::TPUB),
         "TRCK" => Ok(ID3FRAMEID::TRCK),
         "TRDA" => Ok(ID3FRAMEID::TRDA),
         "TRSN" => Ok(ID3FRAMEID::TRSN),
         "TRSO" => Ok(ID3FRAMEID::TRSO),
         "TSIZ" => Ok(ID3FRAMEID::TSIZ),
         "TSRC" => Ok(ID3FRAMEID::TSRC),
         "TSSE" => Ok(ID3FRAMEID::TSSE),
         "TYER" => Ok(ID3FRAMEID::TYER),
         "TXXX" => Ok(ID3FRAMEID::TXXX),
         "UFID" => Ok(ID3FRAMEID::UFID),
         "USER" => Ok(ID3FRAMEID::USER),
         "USLT" => Ok(ID3FRAMEID::USLT),
         "WCOM" => Ok(ID3FRAMEID::WCOM),
         "WCOP" => Ok(ID3FRAMEID::WCOP),
         "WOAF" => Ok(ID3FRAMEID::WOAF),
         "WOAR" => Ok(ID3FRAMEID::WOAR),
         "WOAS" => Ok(ID3FRAMEID::WOAS),
         "WORS" => Ok(ID3FRAMEID::WORS),
         "WPAY" => Ok(ID3FRAMEID::WPAY),
         "WPUB" => Ok(ID3FRAMEID::WPUB),
         "WXXX" => Ok(ID3FRAMEID::WXXX),
         _ => Err(())
      }
   }
}

impl Display for ID3FRAMEID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ID3FRAMEID::APIC => "APIC",
            ID3FRAMEID::AENC => "AENC",
            ID3FRAMEID::COMM => "COMM",
            ID3FRAMEID::COMR => "COMR",
            ID3FRAMEID::ENCR => "ENCR",
            ID3FRAMEID::EQUA => "EQUA",
            ID3FRAMEID::ETCO => "ETCO",
            ID3FRAMEID::GEOB => "GEOB",
            ID3FRAMEID::GRID => "GRID",
            ID3FRAMEID::IPLS => "IPLS",
            ID3FRAMEID::LINK => "LINK",
            ID3FRAMEID::MCDI => "MCDI",
            ID3FRAMEID::MLLT => "MLLT",
            ID3FRAMEID::OWNE => "OWNE",
            ID3FRAMEID::PRIV => "PRIV",
            ID3FRAMEID::PCNT => "PCNT",
            ID3FRAMEID::POPM => "POPM",
            ID3FRAMEID::POSS => "POSS",
            ID3FRAMEID::RBUF => "RBUF",
            ID3FRAMEID::RVAD => "RVAD",
            ID3FRAMEID::RVRB => "RVRB",
            ID3FRAMEID::SYLT => "SYLT",
            ID3FRAMEID::SYTC => "SYTC",
            ID3FRAMEID::TALB => "TALB",
            ID3FRAMEID::TBPM => "TBPM",
            ID3FRAMEID::TCOM => "TCOM",
            ID3FRAMEID::TCON => "TCON",
            ID3FRAMEID::TCOP => "TCOP",
            ID3FRAMEID::TDAT => "TDAT",
            ID3FRAMEID::TDLY => "TDLY",
            ID3FRAMEID::TENC => "TENC",
            ID3FRAMEID::TEXT => "TEXT",
            ID3FRAMEID::TFLT => "TFLT",
            ID3FRAMEID::TIME => "TIME",
            ID3FRAMEID::TIT1 => "TIT1",
            ID3FRAMEID::TIT2 => "TIT2",
            ID3FRAMEID::TIT3 => "TIT3",
            ID3FRAMEID::TKEY => "TKEY",
            ID3FRAMEID::TLAN => "TLAN",
            ID3FRAMEID::TLEN => "TLEN",
            ID3FRAMEID::TSOT => "TSOT",
            ID3FRAMEID::TSO2 => "TSO2",
            ID3FRAMEID::TMED => "TMED",
            ID3FRAMEID::TOAL => "TOAL",
            ID3FRAMEID::TOFN => "TOFN",
            ID3FRAMEID::TOLY => "TOLY",
            ID3FRAMEID::TOPE => "TOPE",
            ID3FRAMEID::TORY => "TORY",
            ID3FRAMEID::TOWN => "TOWN",
            ID3FRAMEID::TPE1 => "TPE1",
            ID3FRAMEID::TPE2 => "TPE2",
            ID3FRAMEID::TPE3 => "TPE3",
            ID3FRAMEID::TPE4 => "TPE4",
            ID3FRAMEID::TPOS => "TPOS",
            ID3FRAMEID::TPUB => "TPUB",
            ID3FRAMEID::TRCK => "TRCK",
            ID3FRAMEID::TRDA => "TRDA",
            ID3FRAMEID::TRSN => "TRSN",
            ID3FRAMEID::TRSO => "TRSO",
            ID3FRAMEID::TSIZ => "TSIZ",
            ID3FRAMEID::TSRC => "TSRC",
            ID3FRAMEID::TSSE => "TSSE",
            ID3FRAMEID::TYER => "TYER",
            ID3FRAMEID::TXXX => "TXXX",
            ID3FRAMEID::UFID => "UFID",
            ID3FRAMEID::USER => "USER",
            ID3FRAMEID::USLT => "USLT",
            ID3FRAMEID::WCOM => "WCOM",
            ID3FRAMEID::WCOP => "WCOP",
            ID3FRAMEID::WOAF => "WOAF",
            ID3FRAMEID::WOAR => "WOAR",
            ID3FRAMEID::WOAS => "WOAS",
            ID3FRAMEID::WORS => "WORS",
            ID3FRAMEID::WPAY => "WPAY",
            ID3FRAMEID::WPUB => "WPUB",
            ID3FRAMEID::WXXX => "WXXX",
            ID3FRAMEID::TCMP => "TCMP",
        };
        write!(f, "{}", s)
    }
}

impl ID3FRAMEID {
    pub(crate) fn is_text_frame(&self) -> bool {
       let frame_name = self.to_string();
       frame_name.starts_with("T") && frame_name != "TXXX" && frame_name != "TCMP"
    }
}