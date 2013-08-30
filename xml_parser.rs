use xml_node::*;
use std::io::ReaderUtil;

mod xml_node;


enum Source <'self>{
    String(&'self str),
    ReaderUtil(&'self ReaderUtil)
}

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


pub struct XmlParser<'self> {
    line: uint,
    col: uint,
    depth: uint,
    elem: Option<XmlElem>,
    priv source: Source<'self>,
    priv buf: ~str,
    priv name: ~str,
    priv attrName: ~str,
    priv attributes: ~[XmlAttr],
    priv delim: char,
    priv state: State

}

impl<'self> XmlParser<'self> {
    /// Constructs a new XmlParser from string `data`
    /// The Xmlparser will use the given string as the source for parsing.
    /// Best used for small examples.
    /// ~~~
    /// let mut p = XmlParser::from_str("<root/>")
    /// p.parse_doc() => XmlDoc { root: XmlElem {name: "root"} ... }
    /// ~~~
    pub fn from_str(data : &'self str)
                    -> XmlParser<'self>{
        XmlParser {
            line: 1,
            col: 0,
            buf: ~"",
            name: ~"",
            elem: None,
            source: String(data),
            attrName: ~"",
            attributes: ~[],
            delim: 0 as char,
            state: OutsideTag,
            depth: 0
        }
    }

    /// Constructs a new XmlParser from Reader `data`
    /// The Xmlparser will use the given string as the source for parsing.
    /// Best used for small examples.
    /// ~~~
    /// let mut p = XmlParser::from_read(stdin)
    /// p.parse_doc() => XmlDoc { root: XmlElem {name: "root"} ... }
    /// ~~~
    pub fn from_read(data : &'self ReaderUtil)
                     -> XmlParser<'self> {
        XmlParser {
            line: 1,
            col: 0,
            buf: ~"",
            name: ~"",
            elem: None,
            source: ReaderUtil(data),
            attrName: ~"",
            attributes: ~[],
            delim: 0 as char,
            state: OutsideTag,
            depth: 0
        }
    }
    /// Constructs a neww XmlParser from string `data`
    /// The Xmlparser will use the given string as the source for parsing.
    /// Best used for small examples.
    /// ~~~
    /// let mut p = XmlParser::new("<root/>")
    /// p.parse_doc() => XmlDoc { root: XmlElem {name: "root"} ... }
    /// ~~~

    /// This method will parse entire document into memory as a tree of 
    /// XmlElem. It retuns an XmlDoc if it parses correctly or an Error
    /// if the parsing wasn't succesful.
    // TODO IMPLEMENT
    pub fn parse_doc(&mut self) 
                     -> Result<XmlDoc,Error> { 
        Ok(XmlDoc::new())
    }

    pub fn next(&mut self) -> Result<XmlNode,Error>{
        //TODO IMPLEMENT
        Ok(XmlCDATA(~"CDATA"))
    }

    pub fn next_call(&mut self, cb: &fn (Result<Events,Error>)) -> Result<XmlNode,Error>{
        //TODO IMPLEMENT
        Ok(XmlCDATA(~"CDATA"))
    }

}


pub fn main() {
    error!("This is an error log");
    warn!("This is a warn log");
    info!("this is an info log");
    debug!("This is a debug log");
}
