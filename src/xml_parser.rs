#[allow(dead_code)]
use xml;
use xml::reader::events::XmlEvent;
use std::borrow::Cow;

enum XmlParserState {
    Idle,
    MethodName,
    Parami4
}

pub struct XmlRequest<'a> {
    method_name: Cow<'a, str>,
}

impl<'a> XmlRequest<'a> {
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
                                    self.method_name = Cow::Owned(name.local_name);
                                    XmlParserState::MethodName
                                }
                                "i4" => XmlParserState::Parami4,
                                // TODO: handle all opening tags
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
                            //self.method_name = text;
                            XmlParserState::MethodName
                        },
                        _ => {
                            XmlParserState::Idle
                        }
                    }
                }
                XmlParserState::Parami4 => {
                    match parser.next() {
                        XmlEvent::Characters(text) => {
                            println!("Param i4: {}", text);
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
    use std::borrow::Cow;

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
        let mut xml_request_parser = XmlRequest { method_name: Cow::Borrowed("")};
        xml_request_parser.parse_xmlrpc_request(request_str);
        assert_eq!(xml_request_parser.method_name, "life");
    }
}
