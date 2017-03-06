
#[derive(Debug)]
pub enum ChunkType {
    Hyphen,
    Slash,
    Star,
    Null,
}

#[derive(Debug)]
pub struct TimeChunk {
    Start: u32,
    End: u32,
    Step: u32,
    Type: ChunkType,
}

impl FromStr for ChunkType{
    type Err = String;

    fn from_str(s: &str) -> Result<ChunkType, Self::Err> {
        match s {
            "-" => Ok(ChunkType::Hyphen),
            "/" => Ok(ChunkType::Slash),
            _ => Err(format!("unknown time chunk type: {}", s)),
        }
    }
}

impl ChunkType {
    fn from(s: &str)-> Option<ChunkType> {
        if s.contains("-") {
            ChunkType::Hyphen 
        }else if s.contains("/") {
            ChunkType::Slash 
        }else if s.contains("*") {
            ChunkType::Star
        }else{
            ChunkType::Null
        }
    }
}

impl TimeChunk {
    fn from_str(s: &str) -> Result<TimeChunk, String> {
        for c in s.chars() {
            if "1234567890/*-".contains(c) return Err("invalid char in time chunk")
        }

        let tye = ChunkType::from(s);

        Err(String::new())
    }
}
