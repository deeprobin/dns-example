use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::Cursor;

use crate::label::DNSLabel;

#[derive(Debug, Copy, Clone)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

#[derive(Debug, Clone)]
pub struct DNSQuestion {
    pub qname: Vec<DNSLabel>,
    pub qtype: u16,
    pub qclass: u16,
}

#[derive(Debug, Clone)]
pub struct DNSRequest {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
}

impl DNSRequest {
    pub fn parse(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(data);

        let header = DNSHeader {
            id: cursor.read_u16::<NetworkEndian>()?,
            flags: cursor.read_u16::<NetworkEndian>()?,
            qdcount: cursor.read_u16::<NetworkEndian>()?,
            ancount: cursor.read_u16::<NetworkEndian>()?,
            nscount: cursor.read_u16::<NetworkEndian>()?,
            arcount: cursor.read_u16::<NetworkEndian>()?,
        };

        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let mut qname = Vec::new();
            loop {
                let len = cursor.read_u8()?;
                if len == 0 {
                    break;
                }

                let mut label_str = String::new();
                for _ in 0..len {
                    let c = cursor.read_u8()? as char;
                    label_str.push(c);
                }

                let label = label_str.into();
                qname.push(label);
            }

            let qtype = cursor.read_u16::<NetworkEndian>()?;
            let qclass = cursor.read_u16::<NetworkEndian>()?;

            questions.push(DNSQuestion {
                qname,
                qtype,
                qclass,
            });
        }

        Ok(DNSRequest { header, questions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{NetworkEndian, WriteBytesExt};

    #[test]
    fn test_dns_request_parse() -> Result<(), Box<dyn std::error::Error>> {
        // Aufbau eines einfachen DNS-Anfragepakets
        let mut packet = Vec::new();
        // Header
        packet.write_u16::<NetworkEndian>(0x1234)?; // ID
        packet.write_u16::<NetworkEndian>(0x0100)?; // Flags
        packet.write_u16::<NetworkEndian>(1)?; // QDCOUNT
        packet.write_u16::<NetworkEndian>(0)?; // ANCOUNT
        packet.write_u16::<NetworkEndian>(0)?; // NSCOUNT
        packet.write_u16::<NetworkEndian>(0)?; // ARCOUNT
                                               // Frage
        packet.push(3); // Länge des Labels "www"
        packet.extend_from_slice(b"www"); // Label "www"
        packet.push(7); // Länge des Labels "example"
        packet.extend_from_slice(b"example"); // Label "example"
        packet.push(3); // Länge des Labels "com"
        packet.extend_from_slice(b"com"); // Label "com"
        packet.push(0); // Ende des QNAME
        packet.write_u16::<NetworkEndian>(1)?; // QTYPE
        packet.write_u16::<NetworkEndian>(1)?; // QCLASS

        // Parsen des Pakets
        let request = DNSRequest::parse(&packet)?;

        // Überprüfen der Header-Werte
        assert_eq!(request.header.id, 0x1234);
        assert_eq!(request.header.flags, 0x0100);
        assert_eq!(request.header.qdcount, 1);
        assert_eq!(request.header.ancount, 0);
        assert_eq!(request.header.nscount, 0);
        assert_eq!(request.header.arcount, 0);

        // Überprüfen der Frage
        assert_eq!(request.questions.len(), 1);
        let question = &request.questions[0];
        assert_eq!(question.qname.len(), 3);
        assert_eq!(Into::<String>::into(question.qname[0].clone()), "www");
        assert_eq!(Into::<String>::into(question.qname[1].clone()), "example");
        assert_eq!(Into::<String>::into(question.qname[2].clone()), "com");
        assert_eq!(question.qtype, 1);
        assert_eq!(question.qclass, 1);

        Ok(())
    }
}
