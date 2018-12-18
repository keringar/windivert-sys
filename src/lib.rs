#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_char;

use winapi::shared::ntdef::{HANDLE, PVOID};
use winapi::shared::minwindef::{UINT, BOOL};
use winapi::shared::basetsd::*;
use winapi::um::minwinbase::LPOVERLAPPED;

#[macro_use]
mod macros;

/****************************************************************************/
/* WINDIVERT API                                                            */
/****************************************************************************/

/*
 * Divert address.
 */
STRUCT!{ struct WINDIVERT_ADDRESS {
    Timestamp: INT64,
    IfIdx: UINT32,
    SubIfIdx: UINT32,
    bitfield0: UINT8,
}}

BITFIELD!(WINDIVERT_ADDRESS bitfield0: UINT8 [
    Direction set_Direction[0..1],
    Loopback set_Loopback[1..2],
    Impostor set_Impostor[2..3],
    PseudoIPChecksum set_PseudoIPChecksum[3..4],
    PseudoTCPChecksum set_PseudoTCPChecksum[4..5],
    PseudoUDPChecksum set_PseudoUDPChecksum[5..6],
    Reserved set_Reserved[6..8],
]);

pub type PWINDIVERT_ADDRESS = *mut WINDIVERT_ADDRESS;

pub const WINDIVERT_DIRECTION_OUTBOUND: UINT8 = 0;
pub const WINDIVERT_DIRECTION_INBOUND: UINT8 = 1;

/*
 * Divert layers.
 */
ENUM!{enum WINDIVERT_LAYER {
    WINDIVERT_LAYER_NETWORK = 0,
    WINDIVERT_LAYER_NETWORK_FORWARD = 1,
}}

pub type PWINDIVERT_LAYER = *mut WINDIVERT_LAYER;

/*
 * Divert flags.
 */
pub const WINDIVERT_FLAG_SNIFF: u64 = 1;
pub const WINDIVERT_FLAG_DROP: u64 = 2;
pub const WINDIVERT_FLAG_DEBUG: u64 = 4;

/*
 * Divert parameters.
 */
ENUM!{ enum WINDIVERT_PARAM {
    WINDIVERT_PARAM_QUEUE_LEN  = 0,
    WINDIVERT_PARAM_QUEUE_TIME = 1,
    WINDIVERT_PARAM_QUEUE_SIZE = 2,
}}

pub const WINDIVERT_PARAM_MAX: WINDIVERT_PARAM = WINDIVERT_PARAM_QUEUE_SIZE;

extern "C" {
    pub fn WinDivertOpen(filter: *const c_char, layer: WINDIVERT_LAYER, priority: INT16, flags: UINT64) -> HANDLE;
    pub fn WinDivertRecv(handle: HANDLE, pPacket: PVOID, packetLen: UINT, pAddr: PWINDIVERT_ADDRESS, readLen: *mut UINT) -> BOOL;
    pub fn WinDivertRecvEx(handle: HANDLE, pPacket: PVOID, packetLen: UINT, flags: UINT64, pAddr: PWINDIVERT_ADDRESS, readLen: *mut UINT, lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn WinDivertSend(handle: HANDLE, pPacket: PVOID, packetLen: UINT, pAddr: PWINDIVERT_ADDRESS, writeLen: *mut UINT) -> BOOL;
    pub fn WinDivertSendEx(handle: HANDLE, pPacket: PVOID, packetLen: UINT, flags: UINT64, pAddr: PWINDIVERT_ADDRESS, writeLen: *mut UINT, lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn WinDivertClose(handle: HANDLE) -> BOOL;
    pub fn WinDivertSetParam(handle: HANDLE, param: WINDIVERT_PARAM, value: UINT64) -> BOOL;
    pub fn WinDivertGetParam(handle: HANDLE, param: WINDIVERT_PARAM, pValue: *mut UINT64) -> BOOL;
}

/****************************************************************************/
/* WINDIVERT HELPER API                                                     */
/****************************************************************************/

/*
 * IPv4/IPv6/ICMP/ICMPv6/TCP/UDP header definitions.
 */
STRUCT!{ struct WINDIVERT_IPHDR {
    bitfield0: UINT8,
    TOS: UINT8,
    Length: UINT16,
    Id: UINT16,
    FragOff0: UINT16,
    TTL: UINT8,
    Protocol: UINT8,
    Checksum: UINT16,
    SrcAddr: UINT32,
    DstAddr: UINT32,
}}

BITFIELD!(WINDIVERT_IPHDR bitfield0: UINT8 [
    HdrLength set_HdrLength[0..4],
    Version set_Version[4..8],
]);

pub type PWINDIVERT_IPHDR = *mut WINDIVERT_IPHDR;

#[inline]
pub fn WINDIVERT_IPHDR_GET_FRAGOFF(hdr: WINDIVERT_IPHDR) -> UINT16 {
    hdr.FragOff0 & 0xFF1F
}

#[inline]
pub fn WINDIVERT_IPHDR_GET_MF(hdr: WINDIVERT_IPHDR) -> BOOL {
    ((hdr.FragOff0 & 0x0020) != 0) as BOOL
}

#[inline]
pub fn WINDIVERT_IPHDR_GET_DF(hdr: WINDIVERT_IPHDR) -> BOOL {
    ((hdr.FragOff0 & 0x0040) != 0) as BOOL
}

#[inline]
pub fn WINDIVERT_IPHDR_GET_RESERVED(hdr: WINDIVERT_IPHDR) -> BOOL {
    ((hdr.FragOff0 & 0x0080) != 0) as BOOL
}

#[inline]
pub fn WINDIVERT_IPHDR_SET_FRAGOFF(hdr: &mut WINDIVERT_IPHDR, val: UINT16) {
    hdr.FragOff0 = (hdr.FragOff0 & 0x00E0) | (val & 0xFF1F);
}

#[inline]
pub fn WINDIVERT_IPHDR_SET_MF(hdr: &mut WINDIVERT_IPHDR, val: UINT16) {
    hdr.FragOff0 = (hdr.FragOff0 & 0xFFDF) | ((val & 0x0001) << 5);
}

#[inline]
pub fn WINDIVERT_IPHDR_SET_DF(hdr: &mut WINDIVERT_IPHDR, val: UINT16) {
    hdr.FragOff0 = (hdr.FragOff0 & 0xFFBF) | ((val & 0x0001) << 6);
}

#[inline]
pub fn WINDIVERT_IPHDR_SET_RESERVED(hdr: &mut WINDIVERT_IPHDR, val: UINT16) {
    hdr.FragOff0 = (hdr.FragOff0 & 0xFF7F) | ((val & 0x0001) << 7);
}

STRUCT!{ struct WINDIVERT_IPV6HDR {
    bitfield0: UINT16,
    FlowLabel1: UINT16,
    Length: UINT16,
    NextHdr: UINT8,
    HopLimit: UINT8,
    SrcAddr: [UINT32; 4],
    DstAddr: [UINT32; 4],
}}

// TODO: These functions should really operate on UINT8's not UINT16's
BITFIELD!(WINDIVERT_IPV6HDR bitfield0: UINT16 [
    TrafficClass0 set_TrafficClass0[0..4],
    Version set_Version[4..8],
    FlowLabel0 set_FlowLabel0[8..12],
    TrafficClass1 set_TrafficClass1[12..16],
]);

pub type PWINDIVERT_IPV6HDR = *mut WINDIVERT_IPV6HDR;

#[inline]
pub fn WINDIVERT_IPV6HDR_GET_TRAFFICCLASS(hdr: WINDIVERT_IPV6HDR) -> UINT8 {
    ((hdr.TrafficClass0() << 4) as UINT8 | hdr.TrafficClass1() as UINT8) as UINT8
}

#[inline]
pub fn WINDIVERT_IPV6HDR_GET_FLOWLABEL(hdr: WINDIVERT_IPV6HDR) -> UINT32{
    (hdr.FlowLabel0() as UINT32) << 16 | hdr.FlowLabel1 as UINT32
}

#[inline]
pub fn WINDIVERT_IPV6HDR_SET_TRAFFICCLASS(hdr: &mut WINDIVERT_IPV6HDR, val: UINT8) {
    hdr.set_TrafficClass0(val as UINT16 >> 4);
    hdr.set_TrafficClass1(val as UINT16);
}

#[inline]
pub fn WINDIVERT_IPV6HDR_SET_FLOWLABEL(hdr: &mut WINDIVERT_IPV6HDR, val: UINT32) {
    hdr.set_FlowLabel0((val >> 16) as UINT16);
    hdr.FlowLabel1 = val as UINT16;
}

STRUCT!{ struct WINDIVERT_ICMPHDR {
    Type: UINT8,
    Code: UINT8,
    Checksum: UINT16,
    Body: UINT32,
}}

pub type PWINDIVERT_ICMPHDR = *mut WINDIVERT_ICMPHDR;

STRUCT!{ struct WINDIVERT_ICMPV6HDR {
    Type: UINT8,
    Code: UINT8,
    Checksum: UINT16,
    Body: UINT32,
}}

pub type PWINDIVERT_ICMPV6HDR = *mut WINDIVERT_ICMPV6HDR;

STRUCT!{ struct WINDIVERT_TCPHDR {
    SrcPort: UINT16,
    DstPort: UINT16,
    SeqNum: UINT32,
    AckNum: UINT32,
    bitfield0: UINT16,
    Window: UINT16,
    Checksum: UINT16,
    UrgPtr: UINT16,
}}

BITFIELD!{WINDIVERT_TCPHDR bitfield0: UINT16 [
    Reserved1 set_Reserved1[0..4],
    HdrLength set_HdrLength[4..8],
    Fin set_Fin[8..9],
    Syn set_Syn[9..10],
    Rst set_Rst[10..11],
    Psh set_Psh[11..12],
    Ack set_Ack[12..13],
    Urg set_Urg[13..14],
    Reserved2 set_Reserved2[14..16],
]}

pub type PWINDIVERT_TCPHDR = *mut WINDIVERT_TCPHDR;

STRUCT!{ struct WINDIVERT_UDPHDR {
    SrcPort: UINT16,
    DstPort: UINT16,
    Length: UINT16,
    Checksum: UINT16,
}}

pub type PWINDIVERT_UDPHDR = *mut WINDIVERT_UDPHDR;

/*
 * Flags for WinDivertHelperCalcChecksums()
 */
pub const WINDIVERT_HELPER_NO_IP_CHECKSUM: UINT64 = 1;
pub const WINDIVERT_HELPER_NO_ICMP_CHECKSUM: UINT64 = 2;
pub const WINDIVERT_HELPER_NO_ICMPV6_CHECKSUM: UINT64 = 4;
pub const WINDIVERT_HELPER_NO_TCP_CHECKSUM: UINT64 = 8;
pub const WINDIVERT_HELPER_NO_UDP_CHECKSUM: UINT64 = 16;

extern "C" {
    pub fn WinDivertHelperParsePacket(pPacket: PVOID, packetLen: UINT, ppIpHdr: *mut PWINDIVERT_IPHDR, ppIpv6Hdr: *mut PWINDIVERT_IPV6HDR, ppIcmpHdr: *mut PWINDIVERT_ICMPHDR, ppIcmpv6Hdr: *mut PWINDIVERT_ICMPV6HDR, ppTcpHdr: *mut PWINDIVERT_TCPHDR, ppUdpHdr: *mut PWINDIVERT_UDPHDR, ppData: *mut PVOID, pDataLen: *mut UINT) -> BOOL;
    pub fn WinDivertHelperParseIPv4Address(addrStr: *const c_char, pAddr: *mut UINT32) -> BOOL;
    pub fn WinDivertHelperParseIPv6Address(addrStr: *const c_char, pAddr: *mut UINT32) -> BOOL;
    pub fn WinDivertHelperCalcChecksums(pPacket: PVOID, packetLen: UINT, pAddr: PWINDIVERT_ADDRESS, flags: UINT64) -> UINT;
    pub fn WinDivertHelperCheckFilter(filter: *const c_char, layer: WINDIVERT_LAYER, errorStr: *mut *const c_char, errorPos: *mut UINT) -> BOOL;
    pub fn WinDivertHelperEvalFilter(filter: *const c_char, layer: WINDIVERT_LAYER, pPacket: PVOID, packetLen: UINT, pAddr: PWINDIVERT_ADDRESS) -> BOOL;
}