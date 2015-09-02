#[allow(dead_code)]
use xml;
use xml::reader::events::XmlEvent;
use std::borrow::Cow;

#[derive(Debug, PartialEq)]
pub enum XmlParam {
    Integer(i32),
    Double(i32),
    XmlString(String),
    Boolean(bool),
    Array(Vec<XmlParam>)
    // Not handling dateTime.iso8601 base64 or struct types
}

enum XmlParserState {
    Idle,
    MethodName,
    ParamInt
}

pub struct XmlRequest<'a> {
    method_name: Cow<'a, str>,
    params: Vec<XmlParam>
}

impl<'a> XmlRequest<'a> {
    pub fn new() -> XmlRequest<'a> {
        XmlRequest { method_name: Cow::Borrowed(""), params: Vec::new() }
    }

    pub fn parse_xmlrpc_request(&mut self, request_str: &str) {
        let mut parser = xml::EventReader::from_str(request_str);
        let mut state = XmlParserState::Idle;

        loop {
            state = match state {
                XmlParserState::Idle => {
                    match parser.next() {
                        XmlEvent::EndDocument | XmlEvent::Error(_) => break,
                        XmlEvent::StartDocument { .. } => XmlParserState::Idle,
                        XmlEvent::StartElement { name, .. } => {
                            match name.local_name.as_ref() {
                                "methodName" => {
                                    XmlParserState::MethodName
                                }
                                "i4" | "int" => XmlParserState::ParamInt,
                                //TODO: handle all opening tags
                                _ => XmlParserState::Idle
                            }
                        },
                        // TODO: match the closing element.
                        _ => {
                            println!("Non start element");
                            XmlParserState::Idle
                        }
                    }
                }
                XmlParserState::MethodName => {
                    match parser.next() {
                        XmlEvent::Characters(text) => {
                            println!("Method name characters: {}", text);
                            self.method_name = Cow::Owned(text);
                            XmlParserState::MethodName
                        },
                        _ => {
                            XmlParserState::Idle
                        }
                    }
                }
                XmlParserState::ParamInt => {
                    match parser.next() {
                        XmlEvent::Characters(text) => {
                            println!("Param Int: {}", text);
                            self.params.push(XmlParam::Integer(text.parse::<i32>().unwrap()));
                            XmlParserState::Idle
                        }
                        _ => XmlParserState::Idle
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::XmlRequest;
    use super::XmlParam;

    #[test]
    fn test_parse_xml() {
        let request_str= "\
        <?xml version=\"1.0\"?>\n\
        <methodCall>\n\
          <methodName>life</methodName>\n\
          <params>\n\
            <param>\n\
              <value><i4>42</i4></value>\n\
            </param>\n\
          </params>\n\
        </methodCall>\n";
        let mut xml_request_parser = XmlRequest::new();
        xml_request_parser.parse_xmlrpc_request(request_str);
        assert_eq!(xml_request_parser.method_name, "life");
        assert_eq!(xml_request_parser.params, [XmlParam::Integer(42)]);
    }
}
