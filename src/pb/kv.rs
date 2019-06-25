// This file is generated by rust-protobuf 2.6.2. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct KeyValue {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub create_revision: i64,
    pub mod_revision: i64,
    pub version: i64,
    pub value: ::std::vec::Vec<u8>,
    pub lease: i64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a KeyValue {
    fn default() -> &'a KeyValue {
        <KeyValue as ::protobuf::Message>::default_instance()
    }
}

impl KeyValue {
    pub fn new() -> KeyValue {
        ::std::default::Default::default()
    }

    // bytes key = 1;


    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    // int64 create_revision = 2;


    pub fn get_create_revision(&self) -> i64 {
        self.create_revision
    }
    pub fn clear_create_revision(&mut self) {
        self.create_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_create_revision(&mut self, v: i64) {
        self.create_revision = v;
    }

    // int64 mod_revision = 3;


    pub fn get_mod_revision(&self) -> i64 {
        self.mod_revision
    }
    pub fn clear_mod_revision(&mut self) {
        self.mod_revision = 0;
    }

    // Param is passed by value, moved
    pub fn set_mod_revision(&mut self, v: i64) {
        self.mod_revision = v;
    }

    // int64 version = 4;


    pub fn get_version(&self) -> i64 {
        self.version
    }
    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i64) {
        self.version = v;
    }

    // bytes value = 5;


    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    // int64 lease = 6;


    pub fn get_lease(&self) -> i64 {
        self.lease
    }
    pub fn clear_lease(&mut self) {
        self.lease = 0;
    }

    // Param is passed by value, moved
    pub fn set_lease(&mut self, v: i64) {
        self.lease = v;
    }
}

impl ::protobuf::Message for KeyValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.create_revision = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mod_revision = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.version = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lease = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if self.create_revision != 0 {
            my_size += ::protobuf::rt::value_size(2, self.create_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mod_revision != 0 {
            my_size += ::protobuf::rt::value_size(3, self.mod_revision, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.value);
        }
        if self.lease != 0 {
            my_size += ::protobuf::rt::value_size(6, self.lease, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if self.create_revision != 0 {
            os.write_int64(2, self.create_revision)?;
        }
        if self.mod_revision != 0 {
            os.write_int64(3, self.mod_revision)?;
        }
        if self.version != 0 {
            os.write_int64(4, self.version)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(5, &self.value)?;
        }
        if self.lease != 0 {
            os.write_int64(6, self.lease)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> KeyValue {
        KeyValue::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    |m: &KeyValue| { &m.key },
                    |m: &mut KeyValue| { &mut m.key },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "create_revision",
                    |m: &KeyValue| { &m.create_revision },
                    |m: &mut KeyValue| { &mut m.create_revision },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "mod_revision",
                    |m: &KeyValue| { &m.mod_revision },
                    |m: &mut KeyValue| { &mut m.mod_revision },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "version",
                    |m: &KeyValue| { &m.version },
                    |m: &mut KeyValue| { &mut m.version },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    |m: &KeyValue| { &m.value },
                    |m: &mut KeyValue| { &mut m.value },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lease",
                    |m: &KeyValue| { &m.lease },
                    |m: &mut KeyValue| { &mut m.lease },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyValue>(
                    "KeyValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static KeyValue {
        static mut instance: ::protobuf::lazy::Lazy<KeyValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyValue,
        };
        unsafe {
            instance.get(KeyValue::new)
        }
    }
}

impl ::protobuf::Clear for KeyValue {
    fn clear(&mut self) {
        self.key.clear();
        self.create_revision = 0;
        self.mod_revision = 0;
        self.version = 0;
        self.value.clear();
        self.lease = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event {
    // message fields
    pub field_type: Event_EventType,
    pub kv: ::protobuf::SingularPtrField<KeyValue>,
    pub prev_kv: ::protobuf::SingularPtrField<KeyValue>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Event {
    fn default() -> &'a Event {
        <Event as ::protobuf::Message>::default_instance()
    }
}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    // .mvccpb.Event.EventType type = 1;


    pub fn get_field_type(&self) -> Event_EventType {
        self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type = Event_EventType::PUT;
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Event_EventType) {
        self.field_type = v;
    }

    // .mvccpb.KeyValue kv = 2;


    pub fn get_kv(&self) -> &KeyValue {
        self.kv.as_ref().unwrap_or_else(|| KeyValue::default_instance())
    }
    pub fn clear_kv(&mut self) {
        self.kv.clear();
    }

    pub fn has_kv(&self) -> bool {
        self.kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kv(&mut self, v: KeyValue) {
        self.kv = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kv(&mut self) -> &mut KeyValue {
        if self.kv.is_none() {
            self.kv.set_default();
        }
        self.kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_kv(&mut self) -> KeyValue {
        self.kv.take().unwrap_or_else(|| KeyValue::new())
    }

    // .mvccpb.KeyValue prev_kv = 3;


    pub fn get_prev_kv(&self) -> &KeyValue {
        self.prev_kv.as_ref().unwrap_or_else(|| KeyValue::default_instance())
    }
    pub fn clear_prev_kv(&mut self) {
        self.prev_kv.clear();
    }

    pub fn has_prev_kv(&self) -> bool {
        self.prev_kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prev_kv(&mut self, v: KeyValue) {
        self.prev_kv = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prev_kv(&mut self) -> &mut KeyValue {
        if self.prev_kv.is_none() {
            self.prev_kv.set_default();
        }
        self.prev_kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_prev_kv(&mut self) -> KeyValue {
        self.prev_kv.take().unwrap_or_else(|| KeyValue::new())
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        for v in &self.kv {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.prev_kv {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.field_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.kv)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prev_kv)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.field_type != Event_EventType::PUT {
            my_size += ::protobuf::rt::enum_size(1, self.field_type);
        }
        if let Some(ref v) = self.kv.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.prev_kv.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.field_type != Event_EventType::PUT {
            os.write_enum(1, self.field_type.value())?;
        }
        if let Some(ref v) = self.kv.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.prev_kv.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Event_EventType>>(
                    "type",
                    |m: &Event| { &m.field_type },
                    |m: &mut Event| { &mut m.field_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyValue>>(
                    "kv",
                    |m: &Event| { &m.kv },
                    |m: &mut Event| { &mut m.kv },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyValue>>(
                    "prev_kv",
                    |m: &Event| { &m.prev_kv },
                    |m: &mut Event| { &mut m.prev_kv },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(Event::new)
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.field_type = Event_EventType::PUT;
        self.kv.clear();
        self.prev_kv.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Event_EventType {
    PUT = 0,
    DELETE = 1,
}

impl ::protobuf::ProtobufEnum for Event_EventType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Event_EventType> {
        match value {
            0 => ::std::option::Option::Some(Event_EventType::PUT),
            1 => ::std::option::Option::Some(Event_EventType::DELETE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Event_EventType] = &[
            Event_EventType::PUT,
            Event_EventType::DELETE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Event_EventType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Event_EventType {
}

impl ::std::default::Default for Event_EventType {
    fn default() -> Self {
        Event_EventType::PUT
    }
}

impl ::protobuf::reflect::ProtobufValue for Event_EventType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x08kv.proto\x12\x06mvccpb\"\x83\x01\n\x08KeyValue\x12\r\n\x03key\x18\
    \x01\x20\x01(\x0cB\0\x12\x19\n\x0fcreate_revision\x18\x02\x20\x01(\x03B\
    \0\x12\x16\n\x0cmod_revision\x18\x03\x20\x01(\x03B\0\x12\x11\n\x07versio\
    n\x18\x04\x20\x01(\x03B\0\x12\x0f\n\x05value\x18\x05\x20\x01(\x0cB\0\x12\
    \x0f\n\x05lease\x18\x06\x20\x01(\x03B\0:\0\"\x9b\x01\n\x05Event\x12'\n\
    \x04type\x18\x01\x20\x01(\x0e2\x17.mvccpb.Event.EventTypeB\0\x12\x1e\n\
    \x02kv\x18\x02\x20\x01(\x0b2\x10.mvccpb.KeyValueB\0\x12#\n\x07prev_kv\
    \x18\x03\x20\x01(\x0b2\x10.mvccpb.KeyValueB\0\"\"\n\tEventType\x12\x07\n\
    \x03PUT\x10\0\x12\n\n\x06DELETE\x10\x01\x1a\0:\0B\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
