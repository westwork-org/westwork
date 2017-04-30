// This file is generated. Do not edit
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
pub struct Empty {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Empty {}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Empty {
        static mut instance: ::protobuf::lazy::Lazy<Empty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Empty,
        };
        unsafe {
            instance.get(Empty::new)
        }
    }
}

impl ::protobuf::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Empty {
    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static(_: ::std::option::Option<Empty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Empty>(
                    "Empty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Empty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Name {
    // message fields
    pub first: ::std::string::String,
    pub last: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Name {}

impl Name {
    pub fn new() -> Name {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Name {
        static mut instance: ::protobuf::lazy::Lazy<Name> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Name,
        };
        unsafe {
            instance.get(Name::new)
        }
    }

    // string first = 1;

    pub fn clear_first(&mut self) {
        self.first.clear();
    }

    // Param is passed by value, moved
    pub fn set_first(&mut self, v: ::std::string::String) {
        self.first = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_first(&mut self) -> &mut ::std::string::String {
        &mut self.first
    }

    // Take field
    pub fn take_first(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.first, ::std::string::String::new())
    }

    pub fn get_first(&self) -> &str {
        &self.first
    }

    fn get_first_for_reflect(&self) -> &::std::string::String {
        &self.first
    }

    fn mut_first_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.first
    }

    // string last = 2;

    pub fn clear_last(&mut self) {
        self.last.clear();
    }

    // Param is passed by value, moved
    pub fn set_last(&mut self, v: ::std::string::String) {
        self.last = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_last(&mut self) -> &mut ::std::string::String {
        &mut self.last
    }

    // Take field
    pub fn take_last(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.last, ::std::string::String::new())
    }

    pub fn get_last(&self) -> &str {
        &self.last
    }

    fn get_last_for_reflect(&self) -> &::std::string::String {
        &self.last
    }

    fn mut_last_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.last
    }
}

impl ::protobuf::Message for Name {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.first)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.last)?;
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
        if !self.first.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.first);
        };
        if !self.last.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.last);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.first.is_empty() {
            os.write_string(1, &self.first)?;
        };
        if !self.last.is_empty() {
            os.write_string(2, &self.last)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Name {
    fn new() -> Name {
        Name::new()
    }

    fn descriptor_static(_: ::std::option::Option<Name>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "first",
                    Name::get_first_for_reflect,
                    Name::mut_first_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "last",
                    Name::get_last_for_reflect,
                    Name::mut_last_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Name>(
                    "Name",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Name {
    fn clear(&mut self) {
        self.clear_first();
        self.clear_last();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Name {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Name {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserName {
    // message fields
    pub username: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserName {}

impl UserName {
    pub fn new() -> UserName {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserName {
        static mut instance: ::protobuf::lazy::Lazy<UserName> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserName,
        };
        unsafe {
            instance.get(UserName::new)
        }
    }

    // string username = 1;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    fn get_username_for_reflect(&self) -> &::std::string::String {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }
}

impl ::protobuf::Message for UserName {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
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
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.username);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.username.is_empty() {
            os.write_string(1, &self.username)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserName {
    fn new() -> UserName {
        UserName::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserName>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    UserName::get_username_for_reflect,
                    UserName::mut_username_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserName>(
                    "UserName",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserName {
    fn clear(&mut self) {
        self.clear_username();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserName {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WifiList {
    // message fields
    networks: ::protobuf::RepeatedField<Network>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WifiList {}

impl WifiList {
    pub fn new() -> WifiList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WifiList {
        static mut instance: ::protobuf::lazy::Lazy<WifiList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WifiList,
        };
        unsafe {
            instance.get(WifiList::new)
        }
    }

    // repeated .Network networks = 1;

    pub fn clear_networks(&mut self) {
        self.networks.clear();
    }

    // Param is passed by value, moved
    pub fn set_networks(&mut self, v: ::protobuf::RepeatedField<Network>) {
        self.networks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_networks(&mut self) -> &mut ::protobuf::RepeatedField<Network> {
        &mut self.networks
    }

    // Take field
    pub fn take_networks(&mut self) -> ::protobuf::RepeatedField<Network> {
        ::std::mem::replace(&mut self.networks, ::protobuf::RepeatedField::new())
    }

    pub fn get_networks(&self) -> &[Network] {
        &self.networks
    }

    fn get_networks_for_reflect(&self) -> &::protobuf::RepeatedField<Network> {
        &self.networks
    }

    fn mut_networks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Network> {
        &mut self.networks
    }
}

impl ::protobuf::Message for WifiList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.networks)?;
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
        for value in &self.networks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.networks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WifiList {
    fn new() -> WifiList {
        WifiList::new()
    }

    fn descriptor_static(_: ::std::option::Option<WifiList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Network>>(
                    "networks",
                    WifiList::get_networks_for_reflect,
                    WifiList::mut_networks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WifiList>(
                    "WifiList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WifiList {
    fn clear(&mut self) {
        self.clear_networks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WifiList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WifiList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Network {
    // message fields
    pub SSID: ::std::string::String,
    pub strength: i32,
    pub encryption: Network_Encryption,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Network {}

impl Network {
    pub fn new() -> Network {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Network {
        static mut instance: ::protobuf::lazy::Lazy<Network> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Network,
        };
        unsafe {
            instance.get(Network::new)
        }
    }

    // string SSID = 1;

    pub fn clear_SSID(&mut self) {
        self.SSID.clear();
    }

    // Param is passed by value, moved
    pub fn set_SSID(&mut self, v: ::std::string::String) {
        self.SSID = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_SSID(&mut self) -> &mut ::std::string::String {
        &mut self.SSID
    }

    // Take field
    pub fn take_SSID(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.SSID, ::std::string::String::new())
    }

    pub fn get_SSID(&self) -> &str {
        &self.SSID
    }

    fn get_SSID_for_reflect(&self) -> &::std::string::String {
        &self.SSID
    }

    fn mut_SSID_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.SSID
    }

    // int32 strength = 2;

    pub fn clear_strength(&mut self) {
        self.strength = 0;
    }

    // Param is passed by value, moved
    pub fn set_strength(&mut self, v: i32) {
        self.strength = v;
    }

    pub fn get_strength(&self) -> i32 {
        self.strength
    }

    fn get_strength_for_reflect(&self) -> &i32 {
        &self.strength
    }

    fn mut_strength_for_reflect(&mut self) -> &mut i32 {
        &mut self.strength
    }

    // .Network.Encryption encryption = 3;

    pub fn clear_encryption(&mut self) {
        self.encryption = Network_Encryption::NONE;
    }

    // Param is passed by value, moved
    pub fn set_encryption(&mut self, v: Network_Encryption) {
        self.encryption = v;
    }

    pub fn get_encryption(&self) -> Network_Encryption {
        self.encryption
    }

    fn get_encryption_for_reflect(&self) -> &Network_Encryption {
        &self.encryption
    }

    fn mut_encryption_for_reflect(&mut self) -> &mut Network_Encryption {
        &mut self.encryption
    }
}

impl ::protobuf::Message for Network {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.SSID)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.strength = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.encryption = tmp;
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
        if !self.SSID.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.SSID);
        };
        if self.strength != 0 {
            my_size += ::protobuf::rt::value_size(2, self.strength, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.encryption != Network_Encryption::NONE {
            my_size += ::protobuf::rt::enum_size(3, self.encryption);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.SSID.is_empty() {
            os.write_string(1, &self.SSID)?;
        };
        if self.strength != 0 {
            os.write_int32(2, self.strength)?;
        };
        if self.encryption != Network_Encryption::NONE {
            os.write_enum(3, self.encryption.value())?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Network {
    fn new() -> Network {
        Network::new()
    }

    fn descriptor_static(_: ::std::option::Option<Network>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "SSID",
                    Network::get_SSID_for_reflect,
                    Network::mut_SSID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "strength",
                    Network::get_strength_for_reflect,
                    Network::mut_strength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Network_Encryption>>(
                    "encryption",
                    Network::get_encryption_for_reflect,
                    Network::mut_encryption_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Network>(
                    "Network",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Network {
    fn clear(&mut self) {
        self.clear_SSID();
        self.clear_strength();
        self.clear_encryption();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Network {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Network {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Network_Encryption {
    NONE = 0,
    WEP = 1,
    WPA = 2,
    WPA2 = 3,
}

impl ::protobuf::ProtobufEnum for Network_Encryption {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Network_Encryption> {
        match value {
            0 => ::std::option::Option::Some(Network_Encryption::NONE),
            1 => ::std::option::Option::Some(Network_Encryption::WEP),
            2 => ::std::option::Option::Some(Network_Encryption::WPA),
            3 => ::std::option::Option::Some(Network_Encryption::WPA2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Network_Encryption] = &[
            Network_Encryption::NONE,
            Network_Encryption::WEP,
            Network_Encryption::WPA,
            Network_Encryption::WPA2,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Network_Encryption>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Network_Encryption", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Network_Encryption {
}

impl ::std::default::Default for Network_Encryption {
    fn default() -> Self {
        Network_Encryption::NONE
    }
}

impl ::protobuf::reflect::ProtobufValue for Network_Encryption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Domain {
    // message fields
    pub domain: ::std::string::String,
    pub tld: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Domain {}

impl Domain {
    pub fn new() -> Domain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Domain {
        static mut instance: ::protobuf::lazy::Lazy<Domain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Domain,
        };
        unsafe {
            instance.get(Domain::new)
        }
    }

    // string domain = 1;

    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        &mut self.domain
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.domain, ::std::string::String::new())
    }

    pub fn get_domain(&self) -> &str {
        &self.domain
    }

    fn get_domain_for_reflect(&self) -> &::std::string::String {
        &self.domain
    }

    fn mut_domain_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.domain
    }

    // string tld = 2;

    pub fn clear_tld(&mut self) {
        self.tld.clear();
    }

    // Param is passed by value, moved
    pub fn set_tld(&mut self, v: ::std::string::String) {
        self.tld = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tld(&mut self) -> &mut ::std::string::String {
        &mut self.tld
    }

    // Take field
    pub fn take_tld(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tld, ::std::string::String::new())
    }

    pub fn get_tld(&self) -> &str {
        &self.tld
    }

    fn get_tld_for_reflect(&self) -> &::std::string::String {
        &self.tld
    }

    fn mut_tld_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tld
    }
}

impl ::protobuf::Message for Domain {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.domain)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tld)?;
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
        if !self.domain.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.domain);
        };
        if !self.tld.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.tld);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.domain.is_empty() {
            os.write_string(1, &self.domain)?;
        };
        if !self.tld.is_empty() {
            os.write_string(2, &self.tld)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Domain {
    fn new() -> Domain {
        Domain::new()
    }

    fn descriptor_static(_: ::std::option::Option<Domain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "domain",
                    Domain::get_domain_for_reflect,
                    Domain::mut_domain_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tld",
                    Domain::get_tld_for_reflect,
                    Domain::mut_tld_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Domain>(
                    "Domain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Domain {
    fn clear(&mut self) {
        self.clear_domain();
        self.clear_tld();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Domain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Domain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x73, 0x2f, 0x62, 0x6f, 0x6f, 0x74,
    0x73, 0x74, 0x72, 0x61, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x07, 0x0a, 0x05, 0x45,
    0x6d, 0x70, 0x74, 0x79, 0x22, 0x30, 0x0a, 0x04, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05,
    0x66, 0x69, 0x72, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x66, 0x69, 0x72,
    0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x61, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6c, 0x61, 0x73, 0x74, 0x22, 0x26, 0x0a, 0x08, 0x55, 0x73, 0x65, 0x72, 0x4e, 0x61,
    0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x30,
    0x0a, 0x08, 0x57, 0x69, 0x66, 0x69, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x24, 0x0a, 0x08, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x08, 0x2e, 0x4e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x52, 0x08, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73,
    0x22, 0xa2, 0x01, 0x0a, 0x07, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x12, 0x0a, 0x04,
    0x53, 0x53, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x53, 0x53, 0x49, 0x44,
    0x12, 0x1a, 0x0a, 0x08, 0x73, 0x74, 0x72, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x08, 0x73, 0x74, 0x72, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x33, 0x0a, 0x0a,
    0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x13, 0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x45, 0x6e, 0x63, 0x72, 0x79,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x22, 0x32, 0x0a, 0x0a, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x08, 0x0a, 0x04, 0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x57, 0x45, 0x50,
    0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x57, 0x50, 0x41, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x57,
    0x50, 0x41, 0x32, 0x10, 0x03, 0x22, 0x32, 0x0a, 0x06, 0x44, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12,
    0x16, 0x0a, 0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x06, 0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x6c, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74, 0x6c, 0x64, 0x32, 0x82, 0x01, 0x0a, 0x09, 0x42, 0x6f,
    0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x12, 0x18, 0x0a, 0x07, 0x53, 0x65, 0x74, 0x4e, 0x61,
    0x6d, 0x65, 0x12, 0x05, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x1a, 0x06, 0x2e, 0x45, 0x6d, 0x70, 0x74,
    0x79, 0x12, 0x20, 0x0a, 0x0b, 0x53, 0x65, 0x74, 0x55, 0x73, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x09, 0x2e, 0x55, 0x73, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x1a, 0x06, 0x2e, 0x45, 0x6d,
    0x70, 0x74, 0x79, 0x12, 0x1c, 0x0a, 0x07, 0x47, 0x65, 0x74, 0x57, 0x69, 0x66, 0x69, 0x12, 0x06,
    0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x1a, 0x09, 0x2e, 0x57, 0x69, 0x66, 0x69, 0x4c, 0x69, 0x73,
    0x74, 0x12, 0x1b, 0x0a, 0x07, 0x53, 0x65, 0x74, 0x57, 0x69, 0x66, 0x69, 0x12, 0x08, 0x2e, 0x4e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x1a, 0x06, 0x2e, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x4a, 0xe3,
    0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x29, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x04, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x06, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x06, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x04, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x04, 0x06, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x04, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x08, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x08, 0x04, 0x07, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x08, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x12, 0x13, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0b, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x0c, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c,
    0x04, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x04,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0b, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x16, 0x17, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x0f, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x10,
    0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x0d, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x15, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x13, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x14, 0x04, 0x13, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x15, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x15, 0x04, 0x14, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x0a, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x15, 0x16, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x16, 0x04, 0x1b, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x17, 0x0f, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x18, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x19, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x1a, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x1a, 0x08, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x1a, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03,
    0x1c, 0x04, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1c, 0x04,
    0x1b, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1c, 0x04, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x0f, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x1f, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x1f, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x20, 0x04,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x20, 0x04, 0x1f, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x21, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x21, 0x04, 0x20, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x21, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21,
    0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x11, 0x12,
    0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x24, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x03, 0x24, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x25, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x25, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x25, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x20, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x26, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x26, 0x15, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x26, 0x28, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x27, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x27, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x21, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x03, 0x28, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x28, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x28, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x28, 0x23, 0x28, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

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
