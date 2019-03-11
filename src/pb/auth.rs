// This file is generated by rust-protobuf 2.4.0. Do not edit
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
pub struct User {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub password: ::std::vec::Vec<u8>,
    pub roles: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl User {
    pub fn new() -> User {
        ::std::default::Default::default()
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    // bytes password = 2;

    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::vec::Vec<u8>) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.password, ::std::vec::Vec::new())
    }

    pub fn get_password(&self) -> &[u8] {
        &self.password
    }

    // repeated string roles = 3;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.roles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_roles(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.roles
    }

    // Take field
    pub fn take_roles(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.roles, ::protobuf::RepeatedField::new())
    }

    pub fn get_roles(&self) -> &[::std::string::String] {
        &self.roles
    }
}

impl ::protobuf::Message for User {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.password)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.roles)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.password);
        }
        for value in &self.roles {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        if !self.password.is_empty() {
            os.write_bytes(2, &self.password)?;
        }
        for v in &self.roles {
            os.write_string(3, &v)?;
        };
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

    fn new() -> User {
        User::new()
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
                    "name",
                    |m: &User| { &m.name },
                    |m: &mut User| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "password",
                    |m: &User| { &m.password },
                    |m: &mut User| { &mut m.password },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "roles",
                    |m: &User| { &m.roles },
                    |m: &mut User| { &mut m.roles },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<User>(
                    "User",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static User {
        static mut instance: ::protobuf::lazy::Lazy<User> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const User,
        };
        unsafe {
            instance.get(User::new)
        }
    }
}

impl ::protobuf::Clear for User {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_password();
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for User {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for User {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Permission {
    // message fields
    pub permType: Permission_Type,
    pub key: ::std::vec::Vec<u8>,
    pub range_end: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Permission {
    pub fn new() -> Permission {
        ::std::default::Default::default()
    }

    // .authpb.Permission.Type permType = 1;

    pub fn clear_permType(&mut self) {
        self.permType = Permission_Type::READ;
    }

    // Param is passed by value, moved
    pub fn set_permType(&mut self, v: Permission_Type) {
        self.permType = v;
    }

    pub fn get_permType(&self) -> Permission_Type {
        self.permType
    }

    // bytes key = 2;

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

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    // bytes range_end = 3;

    pub fn clear_range_end(&mut self) {
        self.range_end.clear();
    }

    // Param is passed by value, moved
    pub fn set_range_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.range_end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.range_end
    }

    // Take field
    pub fn take_range_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.range_end, ::std::vec::Vec::new())
    }

    pub fn get_range_end(&self) -> &[u8] {
        &self.range_end
    }
}

impl ::protobuf::Message for Permission {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.permType, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.range_end)?;
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
        if self.permType != Permission_Type::READ {
            my_size += ::protobuf::rt::enum_size(1, self.permType);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.range_end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.range_end);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.permType != Permission_Type::READ {
            os.write_enum(1, self.permType.value())?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.range_end.is_empty() {
            os.write_bytes(3, &self.range_end)?;
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

    fn new() -> Permission {
        Permission::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Permission_Type>>(
                    "permType",
                    |m: &Permission| { &m.permType },
                    |m: &mut Permission| { &mut m.permType },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    |m: &Permission| { &m.key },
                    |m: &mut Permission| { &mut m.key },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "range_end",
                    |m: &Permission| { &m.range_end },
                    |m: &mut Permission| { &mut m.range_end },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Permission>(
                    "Permission",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Permission {
        static mut instance: ::protobuf::lazy::Lazy<Permission> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Permission,
        };
        unsafe {
            instance.get(Permission::new)
        }
    }
}

impl ::protobuf::Clear for Permission {
    fn clear(&mut self) {
        self.clear_permType();
        self.clear_key();
        self.clear_range_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Permission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Permission_Type {
    READ = 0,
    WRITE = 1,
    READWRITE = 2,
}

impl ::protobuf::ProtobufEnum for Permission_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Permission_Type> {
        match value {
            0 => ::std::option::Option::Some(Permission_Type::READ),
            1 => ::std::option::Option::Some(Permission_Type::WRITE),
            2 => ::std::option::Option::Some(Permission_Type::READWRITE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Permission_Type] = &[
            Permission_Type::READ,
            Permission_Type::WRITE,
            Permission_Type::READWRITE,
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
                ::protobuf::reflect::EnumDescriptor::new("Permission_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Permission_Type {
}

impl ::std::default::Default for Permission_Type {
    fn default() -> Self {
        Permission_Type::READ
    }
}

impl ::protobuf::reflect::ProtobufValue for Permission_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Role {
    // message fields
    pub name: ::std::vec::Vec<u8>,
    pub keyPermission: ::protobuf::RepeatedField<Permission>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl Role {
    pub fn new() -> Role {
        ::std::default::Default::default()
    }

    // bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    // repeated .authpb.Permission keyPermission = 2;

    pub fn clear_keyPermission(&mut self) {
        self.keyPermission.clear();
    }

    // Param is passed by value, moved
    pub fn set_keyPermission(&mut self, v: ::protobuf::RepeatedField<Permission>) {
        self.keyPermission = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keyPermission(&mut self) -> &mut ::protobuf::RepeatedField<Permission> {
        &mut self.keyPermission
    }

    // Take field
    pub fn take_keyPermission(&mut self) -> ::protobuf::RepeatedField<Permission> {
        ::std::mem::replace(&mut self.keyPermission, ::protobuf::RepeatedField::new())
    }

    pub fn get_keyPermission(&self) -> &[Permission] {
        &self.keyPermission
    }
}

impl ::protobuf::Message for Role {
    fn is_initialized(&self) -> bool {
        for v in &self.keyPermission {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keyPermission)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.name);
        }
        for value in &self.keyPermission {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_bytes(1, &self.name)?;
        }
        for v in &self.keyPermission {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Role {
        Role::new()
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
                    "name",
                    |m: &Role| { &m.name },
                    |m: &mut Role| { &mut m.name },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Permission>>(
                    "keyPermission",
                    |m: &Role| { &m.keyPermission },
                    |m: &mut Role| { &mut m.keyPermission },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Role>(
                    "Role",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Role {
        static mut instance: ::protobuf::lazy::Lazy<Role> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Role,
        };
        unsafe {
            instance.get(Role::new)
        }
    }
}

impl ::protobuf::Clear for Role {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_keyPermission();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Role {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nauth.proto\x12\x06authpb\"=\n\x04User\x12\x0e\n\x04name\x18\x01\x20\
    \x01(\x0cB\0\x12\x12\n\x08password\x18\x02\x20\x01(\x0cB\0\x12\x0f\n\x05\
    roles\x18\x03\x20\x03(\tB\0:\0\"\x8d\x01\n\nPermission\x12+\n\x08permTyp\
    e\x18\x01\x20\x01(\x0e2\x17.authpb.Permission.TypeB\0\x12\r\n\x03key\x18\
    \x02\x20\x01(\x0cB\0\x12\x13\n\trange_end\x18\x03\x20\x01(\x0cB\0\",\n\
    \x04Type\x12\x08\n\x04READ\x10\0\x12\t\n\x05WRITE\x10\x01\x12\r\n\tREADW\
    RITE\x10\x02\x1a\0:\0\"E\n\x04Role\x12\x0e\n\x04name\x18\x01\x20\x01(\
    \x0cB\0\x12+\n\rkeyPermission\x18\x02\x20\x03(\x0b2\x12.authpb.Permissio\
    nB\0:\0B\0b\x06proto3\
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
