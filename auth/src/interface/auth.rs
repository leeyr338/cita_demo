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
pub struct Transation {
    // message fields
    pub tx: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transation {}

impl Transation {
    pub fn new() -> Transation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transation {
        static mut instance: ::protobuf::lazy::Lazy<Transation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transation,
        };
        unsafe {
            instance.get(Transation::new)
        }
    }

    // string tx = 1;

    pub fn clear_tx(&mut self) {
        self.tx.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx(&mut self, v: ::std::string::String) {
        self.tx = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx(&mut self) -> &mut ::std::string::String {
        &mut self.tx
    }

    // Take field
    pub fn take_tx(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tx, ::std::string::String::new())
    }

    pub fn get_tx(&self) -> &str {
        &self.tx
    }

    fn get_tx_for_reflect(&self) -> &::std::string::String {
        &self.tx
    }

    fn mut_tx_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tx
    }
}

impl ::protobuf::Message for Transation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tx)?;
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
        if !self.tx.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tx);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx.is_empty() {
            os.write_string(1, &self.tx)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Transation {
    fn new() -> Transation {
        Transation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tx",
                    Transation::get_tx_for_reflect,
                    Transation::mut_tx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transation>(
                    "Transation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transation {
    fn clear(&mut self) {
        self.clear_tx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct State {
    // message fields
    pub state: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for State {}

impl State {
    pub fn new() -> State {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static State {
        static mut instance: ::protobuf::lazy::Lazy<State> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const State,
        };
        unsafe {
            instance.get(State::new)
        }
    }

    // string state = 1;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state(&mut self) -> &mut ::std::string::String {
        &mut self.state
    }

    // Take field
    pub fn take_state(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.state, ::std::string::String::new())
    }

    pub fn get_state(&self) -> &str {
        &self.state
    }

    fn get_state_for_reflect(&self) -> &::std::string::String {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.state
    }
}

impl ::protobuf::Message for State {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.state)?;
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
        if !self.state.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.state);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.state.is_empty() {
            os.write_string(1, &self.state)?;
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
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for State {
    fn new() -> State {
        State::new()
    }

    fn descriptor_static(_: ::std::option::Option<State>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "state",
                    State::get_state_for_reflect,
                    State::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<State>(
                    "State",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for State {
    fn clear(&mut self) {
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nauth.proto\x12\x04auth\"\x1c\n\nTransation\x12\x0e\n\x02tx\x18\x01\
    \x20\x01(\tR\x02tx\"\x1d\n\x05State\x12\x14\n\x05state\x18\x01\x20\x01(\
    \tR\x05state24\n\tauthority\x12'\n\x04auth\x12\x10.auth.Transation\x1a\
    \x0b.auth.State\"\0J\xa7\x02\n\x06\x12\x04\x01\0\x0f\x01\n\x08\n\x01\x0c\
    \x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\x03\x03\x08\x0c\n\n\n\x02\x06\0\
    \x12\x04\x05\0\x07\x01\n\n\n\x03\x06\0\x01\x12\x03\x05\x08\x11\n\x0b\n\
    \x04\x06\0\x02\0\x12\x03\x06\x02*\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    \x06\x06\n\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x06\x0c\x16\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03\x06!&\n\n\n\x02\x04\0\x12\x04\t\0\x0b\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\t\x08\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x02\
    \x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\n\x02\t\x14\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\n\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\t\x0b\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\x0e\x0f\n\n\n\x02\x04\x01\x12\x04\r\
    \0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\r\x08\r\n\x0b\n\x04\x04\x01\x02\
    \0\x12\x03\x0e\x02\x13\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x0e\x02\r\x0f\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x0e\t\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0e\
    \x11\x12b\x06proto3\
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
