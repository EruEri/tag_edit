use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ID3FRAMEID {
   
    TEXTFRAME(ID3TEXTFRAMEID),
    APIC,
    // [[#sec4.20|Audio encryption]]
    AENC,
    /// sec4.11 Comments]
    COMM,
    /// sec4.25 Commercial frame]
    COMR,
    /// sec4.26 Encryption method registration]
    ENCR,
    /// sec4.13 Equalization]
    EQUA,
    /// sec4.6 Event timing codes]
    ETCO,
    /// sec4.16 General encapsulated object]
    GEOB,
    /// sec4.27 Group identification registration]
    GRID,
    /// sec4.4 Involved people list]
    IPLS,
    /// sec4.21 Linked information]
    LINK,
    /// sec4.5 Music CD identifier]
    MCDI,
    /// sec4.7 MPEG location lookup table]
    MLLT,
    /// sec4.24 Ownership frame]
    OWNE,
    /// sec4.17 Play counter]
    PRIV,
    /// sec4.18 Popularimeter]
    PCNT,
    /// sec4.22 Position synchronisation frame]
    POPM,
    POSS,
    RBUF, /// sec4.19 Recommended buffer size]
    RVAD, /// sec4.12 Relative volume adjustment]
    RVRB, /// sec4.14 Reverb]
    SYLT, /// sec4.10 Synchronized lyric/text]
    SYTC, /// sec4.8 Synchronized tempo codes]
    TCMP,
    
    TXXX,
    /// TXXX User defined text information frame]
    UFID,
    /// sec4.1 Unique file identifier]
    USER,
    /// sec4.23 Terms of use]
    USLT,
    /// sec4.9 Unsychronized lyric/text transcription]
    WCOM,
    /// WCOM Commercial information]
    WCOP,
    /// WCOP Copyright/Legal information]
    WOAF,
    /// WOAF Official audio file webpage]
    WOAR,
    /// WOAS Official audio source webpage]
    WOAS,
    /// WOAR Official artist/performer webpage]
    WORS,
    /// WPAY Payment]
    WPAY,
    /// WPUB Publishers official webpage]
    WPUB,
    /// WXXX User defined URL link frame]
    WXXX,
}

impl FromStr for ID3FRAMEID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      if let Some(tf) = ID3TEXTFRAMEID::from_str(s).ok(){
           Ok (Self::TEXTFRAME(tf))
      }else {
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
            _ => Err(()),
        }

      }
    }
}

impl Display for ID3FRAMEID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ID3FRAMEID::APIC => "APIC".to_string(),
            ID3FRAMEID::AENC => "AENC".to_string(),
            ID3FRAMEID::COMM => "COMM".to_string(),
            ID3FRAMEID::COMR => "COMR".to_string(),
            ID3FRAMEID::ENCR => "ENCR".to_string(),
            ID3FRAMEID::EQUA => "EQUA".to_string(),
            ID3FRAMEID::ETCO => "ETCO".to_string(),
            ID3FRAMEID::GEOB => "GEOB".to_string(),
            ID3FRAMEID::GRID => "GRID".to_string(),
            ID3FRAMEID::IPLS => "IPLS".to_string(),
            ID3FRAMEID::LINK => "LINK".to_string(),
            ID3FRAMEID::MCDI => "MCDI".to_string(),
            ID3FRAMEID::MLLT => "MLLT".to_string(),
            ID3FRAMEID::OWNE => "OWNE".to_string(),
            ID3FRAMEID::PRIV => "PRIV".to_string(),
            ID3FRAMEID::PCNT => "PCNT".to_string(),
            ID3FRAMEID::POPM => "POPM".to_string(),
            ID3FRAMEID::POSS => "POSS".to_string(),
            ID3FRAMEID::RBUF => "RBUF".to_string(),
            ID3FRAMEID::RVAD => "RVAD".to_string(),
            ID3FRAMEID::RVRB => "RVRB".to_string(),
            ID3FRAMEID::SYLT => "SYLT".to_string(),
            ID3FRAMEID::SYTC => "SYTC".to_string(),
            ID3FRAMEID::TXXX => "TXXX".to_string(),
            ID3FRAMEID::UFID => "UFID".to_string(),
            ID3FRAMEID::USER => "USER".to_string(),
            ID3FRAMEID::USLT => "USLT".to_string(),
            ID3FRAMEID::WCOM => "WCOM".to_string(),
            ID3FRAMEID::WCOP => "WCOP".to_string(),
            ID3FRAMEID::WOAF => "WOAF".to_string(),
            ID3FRAMEID::WOAR => "WOAR".to_string(),
            ID3FRAMEID::WOAS => "WOAS".to_string(),
            ID3FRAMEID::WORS => "WORS".to_string(),
            ID3FRAMEID::WPAY => "WPAY".to_string(),
            ID3FRAMEID::WPUB => "WPUB".to_string(),
            ID3FRAMEID::WXXX => "WXXX".to_string(),
            ID3FRAMEID::TCMP => "TCMP".to_string(),
            ID3FRAMEID::TEXTFRAME(frame) => frame.to_string(),
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ID3TEXTFRAMEID {
    /// TALB Album/Movie/Show title
    TALB, 
    /// TBPM BPM (beats per minute)]
    TBPM, 
    /// TCOM Composer]
    TCOM, 
    /// TCON Content type]
    TCON, 
    /// TCOP Copyright message]
    TCOP, 
    /// TDAT Date]
    TDAT, 
    /// TDLY Playlist delay]
    TDLY, 
    /// TENC Encoded by]
    TENC,
    /// TEXT Lyricist/Text writer] 
    TEXT,
    /// TFLT File type] 
    TFLT, 
    /// TIME Time]
    TIME,
    /// TIT1 Content group description] 
    TIT1, 
    /// TIT2 Title/songname/content description]
    TIT2, 
    /// TIT3 Subtitle/Description refinement]
    TIT3, 
    /// TKEY Initial key]
    TKEY, 
    /// TLAN Language(s)]
    TLAN, 
    /// TLEN Length]
    TLEN, 
    // Extra
    TSOT, 
    TSO2,
    /// TMED Media type]
    TMED, 
    /// TOAL Original album/movie/show title]
    TOAL, 
    /// TOFN Original filename]
    TOFN, 
    /// TOLY Original lyricist(s)/text writer(s)]
    TOLY, 
    /// TOPE Original artist(s)/performer(s)]
    TOPE, 
    /// TORY Original release year]
    TORY, 
    /// TOWN File owner/licensee]
    TOWN, 
    /// TPE1 Lead performer(s)/Soloist(s)]
    TPE1, 
    /// TPE2 Band/orchestra/accompaniment]
    TPE2, 
    /// TPE3 Conductor/performer refinement]
    TPE3, 
    /// TPE4 Interpreted, remixed, or otherwise modified by]
    TPE4, 
     /// TPOS Part of a set]
    TPOS, 
   /// TPUB Publisher]
    TPUB, 
    /// TRCK Track number/Position in set]
    TRCK, 
    /// TRDA Recording dates]
    TRDA, 
    /// TRSN Internet radio station name]
    TRSN, 
    /// TRSO Internet radio station owner]
    TRSO, 
    /// TSIZ Size]
    TSIZ,
    /// TSRC ISRC (international standard recording code)]
    TSRC,
    /// TSEE Software/Hardware and settings used for encoding]
    TSSE,
    /// TYER Year
    TYER,
}

impl FromStr for ID3TEXTFRAMEID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TALB" => Ok(Self::TALB),
            "TBPM" => Ok(Self::TBPM),
            "TCOM" => Ok(Self::TCOM),
            "TCON" => Ok(Self::TCON),
            "TCOP" => Ok(Self::TCOP),
            "TDAT" => Ok(Self::TDAT),
            "TDLY" => Ok(Self::TDLY),
            "TENC" => Ok(Self::TENC),
            "TEXT" => Ok(Self::TEXT),
            "TFLT" => Ok(Self::TFLT),
            "TIME" => Ok(Self::TIME),
            "TIT1" => Ok(Self::TIT1),
            "TIT2" => Ok(Self::TIT2),
            "TIT3" => Ok(Self::TIT3),
            "TKEY" => Ok(Self::TKEY),
            "TLAN" => Ok(Self::TLAN),
            "TLEN" => Ok(Self::TLEN),
            "TSOT" => Ok(Self::TSOT),
            "TSO2" => Ok(Self::TSO2),
            "TMED" => Ok(Self::TMED),
            "TOAL" => Ok(Self::TOAL),
            "TOFN" => Ok(Self::TOFN),
            "TOLY" => Ok(Self::TOLY),
            "TOPE" => Ok(Self::TOPE),
            "TORY" => Ok(Self::TORY),
            "TOWN" => Ok(Self::TOWN),
            "TPE1" => Ok(Self::TPE1),
            "TPE2" => Ok(Self::TPE2),
            "TPE3" => Ok(Self::TPE3),
            "TPE4" => Ok(Self::TPE4),
            "TPOS" => Ok(Self::TPOS),
            "TPUB" => Ok(Self::TPUB),
            "TRCK" => Ok(Self::TRCK),
            "TRDA" => Ok(Self::TRDA),
            "TRSN" => Ok(Self::TRSN),
            "TRSO" => Ok(Self::TRSO),
            "TSIZ" => Ok(Self::TSIZ),
            "TSRC" => Ok(Self::TSRC),
            "TSSE" => Ok(Self::TSSE),
            "TYER" => Ok(Self::TYER),
            _ => Err(()),
        }
    }
}

impl Display for ID3TEXTFRAMEID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
         Self::TALB => "TALB",
         Self::TBPM => "TBPM",
         Self::TCOM => "TCOM",
         Self::TCON => "TCON",
         Self::TCOP => "TCOP",
         Self::TDAT => "TDAT",
         Self::TDLY => "TDLY",
         Self::TENC => "TENC",
         Self::TEXT => "TEXT",
         Self::TFLT => "TFLT",
         Self::TIME => "TIME",
         Self::TIT1 => "TIT1",
         Self::TIT2 => "TIT2",
         Self::TIT3 => "TIT3",
         Self::TKEY => "TKEY",
         Self::TLAN => "TLAN",
         Self::TLEN => "TLEN",
         Self::TSOT => "TSOT",
         Self::TSO2 => "TSO2",
         Self::TMED => "TMED",
         Self::TOAL => "TOAL",
         Self::TOFN => "TOFN",
         Self::TOLY => "TOLY",
         Self::TOPE => "TOPE",
         Self::TORY => "TORY",
         Self::TOWN => "TOWN",
         Self::TPE1 => "TPE1",
         Self::TPE2 => "TPE2",
         Self::TPE3 => "TPE3",
         Self::TPE4 => "TPE4",
         Self::TPOS => "TPOS",
         Self::TPUB => "TPUB",
         Self::TRCK => "TRCK",
         Self::TRDA => "TRDA",
         Self::TRSN => "TRSN",
         Self::TRSO => "TRSO",
         Self::TSIZ => "TSIZ",
         Self::TSRC => "TSRC",
         Self::TSSE => "TSSE",
         Self::TYER => "TYER",
        };
        write!(f, "{}", s)
    }
}
