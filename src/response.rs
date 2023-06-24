use std::{
    io::Write,
    net::{Ipv4Addr, Ipv6Addr},
};

use crate::{label::DNSLabel, request::DNSQuestion, resourcerecord::ResourceRecordType};
use byteorder::{NetworkEndian, WriteBytesExt};

// Die verschiedenen Typen von Ressourcendatensätzen, die in einer DNS-Antwort enthalten sein können
#[derive(Debug)]
pub enum DnsRecordData {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    MX(u16, String),
    NS(String),
    MD(String),
    MF(String),
    MB(String),
    MG(String),
    MR(String),
    PTR(String),
    SOA {
        mname: String,
        rname: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: String,
    },
    TXT(Vec<String>),
}

impl DnsRecordData {
    pub fn to_type(&self) -> ResourceRecordType {
        match self {
            DnsRecordData::A(_) => ResourceRecordType::A,
            DnsRecordData::AAAA(_) => ResourceRecordType::AAAA,
            DnsRecordData::CNAME(_) => ResourceRecordType::CNAME,
            DnsRecordData::MX(_, _) => ResourceRecordType::MX,
            DnsRecordData::NS(_) => ResourceRecordType::NS,
            DnsRecordData::MD(_) => ResourceRecordType::MD,
            DnsRecordData::MF(_) => ResourceRecordType::MF,
            DnsRecordData::MB(_) => ResourceRecordType::MB,
            DnsRecordData::MG(_) => ResourceRecordType::MG,
            DnsRecordData::MR(_) => ResourceRecordType::MR,
            DnsRecordData::PTR(_) => ResourceRecordType::PTR,
            DnsRecordData::SOA {
                mname: _,
                rname: _,
                serial: _,
                refresh: _,
                retry: _,
                expire: _,
                minimum: _,
            } => ResourceRecordType::SOA,
            DnsRecordData::SRV {
                priority: _,
                weight: _,
                port: _,
                target: _,
            } => ResourceRecordType::SRV,
            DnsRecordData::TXT(_) => ResourceRecordType::TXT,
        }
    }
}

// Ein DNS-Ressourcendatensatz, der in der Antwortsektion einer DNS-Antwort enthalten ist
#[derive(Debug)]
pub struct DnsResourceRecord {
    pub name: Vec<DNSLabel>,
    pub rtype: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdata: DnsRecordData,
}

#[derive(Debug, Copy, Clone)]
pub struct DnsResponseHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

// Die DNS-Antwortstruktur
#[derive(Debug)]
pub struct DnsResponse {
    pub header: DnsResponseHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DnsResourceRecord>,
    pub authority: Vec<DnsResourceRecord>,
    pub additional: Vec<DnsResourceRecord>,
}

impl DnsResponse {
    pub fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        // Header
        buffer.write_u16::<NetworkEndian>(self.header.id)?;
        buffer.write_u16::<NetworkEndian>(self.header.flags)?;
        buffer.write_u16::<NetworkEndian>(self.header.qdcount)?;
        buffer.write_u16::<NetworkEndian>(self.header.ancount)?;
        buffer.write_u16::<NetworkEndian>(self.header.nscount)?;
        buffer.write_u16::<NetworkEndian>(self.header.arcount)?;

        // Questions
        for question in &self.questions {
            for label in &question.qname {
                buffer.push(label.0.len() as u8);
                buffer.extend_from_slice(label.0.as_bytes());
            }
            buffer.push(0); // End of QNAME
            buffer.write_u16::<NetworkEndian>(question.qtype)?;
            buffer.write_u16::<NetworkEndian>(question.qclass)?;
        }

        // Answers
        for answer in &self.answers {
            for label in &answer.name {
                buffer.push(label.0.len() as u8);
                buffer.extend_from_slice(label.0.as_bytes());
            }
            buffer.push(0); // End of NAME
            buffer.write_u16::<NetworkEndian>(answer.rtype)?;
            buffer.write_u16::<NetworkEndian>(answer.class)?;
            buffer.write_u32::<NetworkEndian>(answer.ttl)?;
            match &answer.rdata {
                DnsRecordData::A(addr) => {
                    buffer.write_u16::<NetworkEndian>(4)?; // RDATA length for IPv4 address
                    buffer.write_u32::<NetworkEndian>(u32::from(*addr))?;
                }
                DnsRecordData::AAAA(addr) => {
                    buffer.write_u16::<NetworkEndian>(16)?; // RDATA length for IPv6 address
                    for segment in &addr.segments() {
                        buffer.write_u16::<NetworkEndian>(*segment)?;
                    }
                }
                DnsRecordData::CNAME(cname) => {
                    buffer.write_u16::<NetworkEndian>(cname.len() as u16)?; // RDATA length for CNAME
                    buffer.write_all(cname.as_bytes())?;
                }
                DnsRecordData::MX(preference, exchange) => {
                    buffer.write_u16::<NetworkEndian>(2 + exchange.len() as u16)?; // RDATA length for MX
                    buffer.write_u16::<NetworkEndian>(*preference)?;
                    buffer.write_all(exchange.as_bytes())?;
                }
                DnsRecordData::NS(_) => todo!(),
                DnsRecordData::MD(_) => todo!(),
                DnsRecordData::MF(_) => todo!(),
                DnsRecordData::MB(_) => todo!(),
                DnsRecordData::MG(_) => todo!(),
                DnsRecordData::MR(_) => todo!(),
                DnsRecordData::PTR(_) => todo!(),
                DnsRecordData::SOA {
                    mname,
                    rname,
                    serial,
                    refresh,
                    retry,
                    expire,
                    minimum,
                } => todo!(),
                DnsRecordData::SRV {
                    priority,
                    weight,
                    port,
                    target,
                } => todo!(),
                DnsRecordData::TXT(_) => todo!(),
            }
        }

        // For this simple example, we ignore the Authority and Additional sections

        Ok(buffer)
    }
}
