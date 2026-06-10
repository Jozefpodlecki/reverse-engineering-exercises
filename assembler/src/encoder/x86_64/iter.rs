use crate::{Parser, encoder::x86_64, error::AssemblerError};

pub struct EncoderIter<'a> {
    parser: Parser<'a>,
    current_offset: u64,
}

impl<'a> EncoderIter<'a> {
    pub fn new(parser: Parser<'a>) -> Self {
        Self {
            parser,
            current_offset: 0,
        }
    }
}

// impl<'a> Iterator for EncoderIter<'a> {
//     type Item = Result<Vec<u8>, AssemblerError>;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.parser.next() {
//             Some(Ok(instr)) => {
//                 let encoded = x86_64::Encoder::encode(&instr);
//                 self.current_offset += encoded.len() as u64;
//                 Some(Ok(encoded))
//             }
//             Some(Err(e)) => Some(Err(AssemblerError::ParserError(e))),
//             None => None,
//         }
//     }
// }