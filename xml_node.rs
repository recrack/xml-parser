use std::str;

#[inline]
/// Escapes unallowed character //TODO CHECK WHICH
pub fn escape(input: &str) -> ~str {
    let mut result = str::with_capacity(input.len());

    for c in input.iter() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\'' => result.push_str("&apos;"),
            '"' => result.push_str("&quot;"),
            o => result.push_char(o)
        }
    }
    result
}

#[inline]
/// Unescapes all valid XML entities in a string.
pub fn unescape(input: &str) -> ~str {
    let tmp = str::replace(input, "&quot;", "\"");
    let tmp = str::replace(tmp, "&apos;", "'");
    let tmp = str::replace(tmp, "&gt;", ">");
    let tmp = str::replace(tmp, "&lt;", "<");
    str::replace(tmp, "&amp;", "&")
}

#[deriving(Clone,Eq)]
/// A struct representing an XML root document
pub struct XmlDoc {
    // The document's root
    root: ~XmlElem,
    // The document's processing instructions
    pi: ~[PINode]
}

#[deriving(Clone,Eq)]
/// A struct representing an XML processing instruction
pub struct PINode {
    /// The processing instruction's target
    target: ~str,
    /// The processing instruction's value
    /// Must not contain ?>
    value: ~str
}

#[deriving(Clone,Eq)]
/// A struct representing an XML element
pub struct XmlElem {
    /// The element's name
    name: ~str,
    /// The element's namespace
    namespace: ~XmlNS,
    /// The element's `Attribute`s
    attributes: ~[XmlAttr],
    /// The element's child `XmlNode` nodes
    children: ~[XmlNode]
}


#[deriving(Clone,Eq)]
/// A struct representing an XML attribute
pub struct XmlAttr {
    /// The attribute's name
    name: ~str,
    /// The attribute's value
    value: ~str,
    /// The attribute's namespace
    namespace: ~XmlNS
}

#[deriving(Clone,Eq)]
/// A struct that models an XML namespace
pub struct XmlNS {
    /// The namespace's shorthand name
    name: ~str,
    /// The namespace's uri value
    uri: ~str
}


// General types
#[deriving(Clone,Eq)]
/// An Enum describing a XML Node
pub enum XmlNode {
    /// An XML Element
    XmlElem(~XmlElem),
    /// Character Data
    XmlText(~str),
    /// CDATA
    XmlCDATA(~str),
    /// A XML Comment
    XmlComment(~str),
    /// Processing Information
    PINode(~PINode)
}


#[deriving(Eq)]
/// Events returned by the Parser
pub enum Events {
    Document,
    /// Event indicating a start tag was found
    ElementStart {
        name: ~str, 
        attributes : ~[XmlAttr],
        namespace : ~XmlNS
    },
    /// Event indicating an end tag was found
    ElementEnd {
        name: ~str
    },
    /// Event indicating processing information was found
    ProcessInstruction(~str),
    /// Event indicating character data was found
    Text (~str),
    /// Event indicating CDATA was found
    CDATA (~str),
    /// Event indicating a comment was found
    Comment (~str)
    //EndOfFile
}


#[deriving(Eq)]
/// If an error occurs while parsing some XML, this is the structure which is
/// returned
pub struct Error {
    /// The line number at which the error occurred
    line: uint,
    /// The column number at which the error occurred
    col: uint,
    /// A message describing the type of the error
    msg: @~str
}

fn main() {
    
}

impl XmlDoc {
    pub fn new() -> XmlDoc {
        XmlDoc {
            root: ~XmlElem {
                    name:~"",
                    namespace:~XmlNS{name: ~"", uri: ~""}, 
                    attributes: ~[],
                    children: ~[]
            },
            pi: ~[]
        }
    }

    pub fn to_str(&self) -> ~str {
        let mut ret = ~"";
        for e in self.pi.iter() {
            ret = ret + e.to_str();
        }
        ret
    }
}


impl PINode {
    pub fn to_str(&self) -> ~str {
       fmt!("<?%s %s ?>", self.target, self.value)
    }
}


impl XmlNS {
    pub fn to_str() -> ~str {
        ~""
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_pi_to_str(){
        let pi = ~PINode { target: ~"php", value: ~"echo"};
        assert_eq!(~"<?php echo ?>",pi.to_str())
    }
}
