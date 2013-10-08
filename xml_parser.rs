use xml_node::*;
use std::io::*;
use util::*;
use xml_lexer::*;


mod xml_node;
mod util;
mod xml_lexer;

enum State {
    OutsideTag,
    TagOpened,
    InProcessingInstructions,
    InTagName,
    InCloseTagName,
    InTag,
    InAttrName,
    InAttrValue,
    ExpectDelimiter,
    ExpectClose,
    ExpectSpaceOrClose,
    InExclamationMark,
    InCDATAOpening,
    InCDATA,
    InCommentOpening,
    InComment1,
    InComment2,
    InDoctype,
    Namespace
}



#[deriving(Eq)]
pub enum ParseResult {
    NoNode,
    ParseError(XmlError),
    ParseNode(XNode)
}

pub struct XmlParser {
    line: uint,
    col: uint,
    depth: uint,
    elem: Option<XmlElem>,
    priv lexer: XmlLexer,
    priv name: ~str,
    priv attrName: ~str,
    priv attributes: ~[XmlAttr],
    priv state: State

}

impl Iterator<Result<XNode,XmlError>> for XmlParser {
    /// This method pulls tokens until it reaches a fully formed XML node
    /// once it's finished a node, it stops returning said node or error
    /// if it encountered one.
    ///
    /// This method should be used similar to an outer iterator.
    fn next(&mut self)
            -> Option<Result<XNode,XmlError>>{
        let mut node = NoNode;
        None

    }
}

impl XmlParser {
    /// Constructs a new XmlParser from Reader `data`
    /// The Xmlparser will use the given string as the source for parsing.
    /// Best used for small examples.
    /// ~~~
    /// let mut p = XmlParser::from_read(stdin)
    /// p.parse_doc() => XmlDoc { root: XmlElem {name: "root"} ... }
    /// ~~~
    pub fn from_reader(data : @Reader)
                     -> XmlParser {
        XmlParser {
            line: 1,
            col: 0,
            depth: 0,
            elem: None,
            lexer: XmlLexer::from_reader(data),
            name: ~"",
            attrName: ~"",
            attributes: ~[],
            state: OutsideTag
        }
    }

    /// This method will parse entire document into memory as a tree of
    /// XmlElem. It retuns an XmlDoc if it parses correctly or an Error
    /// if the parsing wasn't succesful.
    // TODO IMPLEMENT
    pub fn parse_doc(&mut self)
                     -> Result<XmlDoc,XmlError> {
        Ok(XmlDoc::new())
    }

}


pub fn main() {
    error!("This is an error log");
    warn!("This is a warn log");
    info!("this is an info log");
    debug!("This is a debug log");
}

/*
#[cfg(test)]
mod tests{
    use super::*;
    use std::io::*;
    use xml_node::*;

    #[test]
    fn parse_simple(){
        let r1 = @BytesReader {
                bytes : "<a>".as_bytes(),
                pos: @mut 0
        } as ~Reader;

        let mut parser = XmlParser::from_reader(r1);
        let node = parser.next();
        match node {
            Some(Ok(a)) => {
                println(fmt!("PRINT: %?", a));
                assert_eq!(XElem(~XmlElem::new(~"a")), a);
            }
            _ => {
                fail!(~"No element found");
            }
        }

    }


}
*/
