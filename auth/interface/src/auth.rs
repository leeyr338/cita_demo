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
pub struct Transaction {
    // message fields
    pub to: ::std::string::String,
    pub nonce: ::std::string::String,
    pub quota: u64,
    pub valid_until_block: u64,
    pub data: ::std::vec::Vec<u8>,
    pub value: u64,
    pub chain_id: u32,
    pub version: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transaction {}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(Transaction::new)
        }
    }

    // string to = 1;

    pub fn clear_to(&mut self) {
        self.to.clear();
    }

    // Param is passed by value, moved
    pub fn set_to(&mut self, v: ::std::string::String) {
        self.to = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }

    // Take field
    pub fn take_to(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.to, ::std::string::String::new())
    }

    pub fn get_to(&self) -> &str {
        &self.to
    }

    fn get_to_for_reflect(&self) -> &::std::string::String {
        &self.to
    }

    fn mut_to_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.to
    }

    // string nonce = 2;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::string::String) {
        self.nonce = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.nonce, ::std::string::String::new())
    }

    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &::std::string::String {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.nonce
    }

    // uint64 quota = 3;

    pub fn clear_quota(&mut self) {
        self.quota = 0;
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = v;
    }

    pub fn get_quota(&self) -> u64 {
        self.quota
    }

    fn get_quota_for_reflect(&self) -> &u64 {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut u64 {
        &mut self.quota
    }

    // uint64 valid_until_block = 4;

    pub fn clear_valid_until_block(&mut self) {
        self.valid_until_block = 0;
    }

    // Param is passed by value, moved
    pub fn set_valid_until_block(&mut self, v: u64) {
        self.valid_until_block = v;
    }

    pub fn get_valid_until_block(&self) -> u64 {
        self.valid_until_block
    }

    fn get_valid_until_block_for_reflect(&self) -> &u64 {
        &self.valid_until_block
    }

    fn mut_valid_until_block_for_reflect(&mut self) -> &mut u64 {
        &mut self.valid_until_block
    }

    // bytes data = 5;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // uint64 value = 6;

    pub fn clear_value(&mut self) {
        self.value = 0;
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u64) {
        self.value = v;
    }

    pub fn get_value(&self) -> u64 {
        self.value
    }

    fn get_value_for_reflect(&self) -> &u64 {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut u64 {
        &mut self.value
    }

    // uint32 chain_id = 7;

    pub fn clear_chain_id(&mut self) {
        self.chain_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_chain_id(&mut self, v: u32) {
        self.chain_id = v;
    }

    pub fn get_chain_id(&self) -> u32 {
        self.chain_id
    }

    fn get_chain_id_for_reflect(&self) -> &u32 {
        &self.chain_id
    }

    fn mut_chain_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.chain_id
    }

    // uint32 version = 8;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.to)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.nonce)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.valid_until_block = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.value = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.chain_id = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
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
        if !self.to.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.to);
        }
        if !self.nonce.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.nonce);
        }
        if self.quota != 0 {
            my_size += ::protobuf::rt::value_size(3, self.quota, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.valid_until_block != 0 {
            my_size += ::protobuf::rt::value_size(4, self.valid_until_block, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.data);
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::value_size(6, self.value, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.chain_id != 0 {
            my_size += ::protobuf::rt::value_size(7, self.chain_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(8, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.to.is_empty() {
            os.write_string(1, &self.to)?;
        }
        if !self.nonce.is_empty() {
            os.write_string(2, &self.nonce)?;
        }
        if self.quota != 0 {
            os.write_uint64(3, self.quota)?;
        }
        if self.valid_until_block != 0 {
            os.write_uint64(4, self.valid_until_block)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(5, &self.data)?;
        }
        if self.value != 0 {
            os.write_uint64(6, self.value)?;
        }
        if self.chain_id != 0 {
            os.write_uint32(7, self.chain_id)?;
        }
        if self.version != 0 {
            os.write_uint32(8, self.version)?;
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

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "to",
                    Transaction::get_to_for_reflect,
                    Transaction::mut_to_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonce",
                    Transaction::get_nonce_for_reflect,
                    Transaction::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    Transaction::get_quota_for_reflect,
                    Transaction::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "valid_until_block",
                    Transaction::get_valid_until_block_for_reflect,
                    Transaction::mut_valid_until_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Transaction::get_data_for_reflect,
                    Transaction::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "value",
                    Transaction::get_value_for_reflect,
                    Transaction::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "chain_id",
                    Transaction::get_chain_id_for_reflect,
                    Transaction::mut_chain_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    Transaction::get_version_for_reflect,
                    Transaction::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_to();
        self.clear_nonce();
        self.clear_quota();
        self.clear_valid_until_block();
        self.clear_data();
        self.clear_value();
        self.clear_chain_id();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnverifiedTransaction {
    // message fields
    pub transaction: ::protobuf::SingularPtrField<Transaction>,
    pub signature: ::std::vec::Vec<u8>,
    pub crypto: Crypto,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnverifiedTransaction {}

impl UnverifiedTransaction {
    pub fn new() -> UnverifiedTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnverifiedTransaction {
        static mut instance: ::protobuf::lazy::Lazy<UnverifiedTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnverifiedTransaction,
        };
        unsafe {
            instance.get(UnverifiedTransaction::new)
        }
    }

    // .Transaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: Transaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut Transaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        }
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> Transaction {
        self.transaction.take().unwrap_or_else(|| Transaction::new())
    }

    pub fn get_transaction(&self) -> &Transaction {
        self.transaction.as_ref().unwrap_or_else(|| Transaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<Transaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Transaction> {
        &mut self.transaction
    }

    // bytes signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // .Crypto crypto = 3;

    pub fn clear_crypto(&mut self) {
        self.crypto = Crypto::SECP;
    }

    // Param is passed by value, moved
    pub fn set_crypto(&mut self, v: Crypto) {
        self.crypto = v;
    }

    pub fn get_crypto(&self) -> Crypto {
        self.crypto
    }

    fn get_crypto_for_reflect(&self) -> &Crypto {
        &self.crypto
    }

    fn mut_crypto_for_reflect(&mut self) -> &mut Crypto {
        &mut self.crypto
    }
}

impl ::protobuf::Message for UnverifiedTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.transaction {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.crypto = tmp;
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
        if let Some(ref v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.signature);
        }
        if self.crypto != Crypto::SECP {
            my_size += ::protobuf::rt::enum_size(3, self.crypto);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(2, &self.signature)?;
        }
        if self.crypto != Crypto::SECP {
            os.write_enum(3, self.crypto.value())?;
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

impl ::protobuf::MessageStatic for UnverifiedTransaction {
    fn new() -> UnverifiedTransaction {
        UnverifiedTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnverifiedTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transaction>>(
                    "transaction",
                    UnverifiedTransaction::get_transaction_for_reflect,
                    UnverifiedTransaction::mut_transaction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    UnverifiedTransaction::get_signature_for_reflect,
                    UnverifiedTransaction::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Crypto>>(
                    "crypto",
                    UnverifiedTransaction::get_crypto_for_reflect,
                    UnverifiedTransaction::mut_crypto_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnverifiedTransaction>(
                    "UnverifiedTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnverifiedTransaction {
    fn clear(&mut self) {
        self.clear_transaction();
        self.clear_signature();
        self.clear_crypto();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnverifiedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnverifiedTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddUnverifyTxReq {
    // message fields
    pub untx: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddUnverifyTxReq {}

impl AddUnverifyTxReq {
    pub fn new() -> AddUnverifyTxReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddUnverifyTxReq {
        static mut instance: ::protobuf::lazy::Lazy<AddUnverifyTxReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddUnverifyTxReq,
        };
        unsafe {
            instance.get(AddUnverifyTxReq::new)
        }
    }

    // string untx = 1;

    pub fn clear_untx(&mut self) {
        self.untx.clear();
    }

    // Param is passed by value, moved
    pub fn set_untx(&mut self, v: ::std::string::String) {
        self.untx = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_untx(&mut self) -> &mut ::std::string::String {
        &mut self.untx
    }

    // Take field
    pub fn take_untx(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.untx, ::std::string::String::new())
    }

    pub fn get_untx(&self) -> &str {
        &self.untx
    }

    fn get_untx_for_reflect(&self) -> &::std::string::String {
        &self.untx
    }

    fn mut_untx_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.untx
    }
}

impl ::protobuf::Message for AddUnverifyTxReq {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.untx)?;
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
        if !self.untx.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.untx);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.untx.is_empty() {
            os.write_string(1, &self.untx)?;
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

impl ::protobuf::MessageStatic for AddUnverifyTxReq {
    fn new() -> AddUnverifyTxReq {
        AddUnverifyTxReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddUnverifyTxReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "untx",
                    AddUnverifyTxReq::get_untx_for_reflect,
                    AddUnverifyTxReq::mut_untx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddUnverifyTxReq>(
                    "AddUnverifyTxReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddUnverifyTxReq {
    fn clear(&mut self) {
        self.clear_untx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddUnverifyTxReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddUnverifyTxReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AddUnverifyTxRes {
    // message fields
    pub tx_res: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AddUnverifyTxRes {}

impl AddUnverifyTxRes {
    pub fn new() -> AddUnverifyTxRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddUnverifyTxRes {
        static mut instance: ::protobuf::lazy::Lazy<AddUnverifyTxRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddUnverifyTxRes,
        };
        unsafe {
            instance.get(AddUnverifyTxRes::new)
        }
    }

    // string tx_res = 1;

    pub fn clear_tx_res(&mut self) {
        self.tx_res.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_res(&mut self, v: ::std::string::String) {
        self.tx_res = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_res(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }

    // Take field
    pub fn take_tx_res(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tx_res, ::std::string::String::new())
    }

    pub fn get_tx_res(&self) -> &str {
        &self.tx_res
    }

    fn get_tx_res_for_reflect(&self) -> &::std::string::String {
        &self.tx_res
    }

    fn mut_tx_res_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }
}

impl ::protobuf::Message for AddUnverifyTxRes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tx_res)?;
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
        if !self.tx_res.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tx_res);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_res.is_empty() {
            os.write_string(1, &self.tx_res)?;
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

impl ::protobuf::MessageStatic for AddUnverifyTxRes {
    fn new() -> AddUnverifyTxRes {
        AddUnverifyTxRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddUnverifyTxRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tx_res",
                    AddUnverifyTxRes::get_tx_res_for_reflect,
                    AddUnverifyTxRes::mut_tx_res_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddUnverifyTxRes>(
                    "AddUnverifyTxRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddUnverifyTxRes {
    fn clear(&mut self) {
        self.clear_tx_res();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AddUnverifyTxRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddUnverifyTxRes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyBatchTxsReq {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyBatchTxsReq {}

impl VerifyBatchTxsReq {
    pub fn new() -> VerifyBatchTxsReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyBatchTxsReq {
        static mut instance: ::protobuf::lazy::Lazy<VerifyBatchTxsReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyBatchTxsReq,
        };
        unsafe {
            instance.get(VerifyBatchTxsReq::new)
        }
    }
}

impl ::protobuf::Message for VerifyBatchTxsReq {
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

impl ::protobuf::MessageStatic for VerifyBatchTxsReq {
    fn new() -> VerifyBatchTxsReq {
        VerifyBatchTxsReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyBatchTxsReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<VerifyBatchTxsReq>(
                    "VerifyBatchTxsReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyBatchTxsReq {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyBatchTxsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyBatchTxsReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcStatus {
    // message fields
    pub tx_res: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcStatus {}

impl RpcStatus {
    pub fn new() -> RpcStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcStatus {
        static mut instance: ::protobuf::lazy::Lazy<RpcStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcStatus,
        };
        unsafe {
            instance.get(RpcStatus::new)
        }
    }

    // string tx_res = 1;

    pub fn clear_tx_res(&mut self) {
        self.tx_res.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_res(&mut self, v: ::std::string::String) {
        self.tx_res = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_res(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }

    // Take field
    pub fn take_tx_res(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tx_res, ::std::string::String::new())
    }

    pub fn get_tx_res(&self) -> &str {
        &self.tx_res
    }

    fn get_tx_res_for_reflect(&self) -> &::std::string::String {
        &self.tx_res
    }

    fn mut_tx_res_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }
}

impl ::protobuf::Message for RpcStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tx_res)?;
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
        if !self.tx_res.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tx_res);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_res.is_empty() {
            os.write_string(1, &self.tx_res)?;
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

impl ::protobuf::MessageStatic for RpcStatus {
    fn new() -> RpcStatus {
        RpcStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tx_res",
                    RpcStatus::get_tx_res_for_reflect,
                    RpcStatus::mut_tx_res_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcStatus>(
                    "RpcStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcStatus {
    fn clear(&mut self) {
        self.clear_tx_res();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTxsHashesReq {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTxsHashesReq {}

impl GetTxsHashesReq {
    pub fn new() -> GetTxsHashesReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTxsHashesReq {
        static mut instance: ::protobuf::lazy::Lazy<GetTxsHashesReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTxsHashesReq,
        };
        unsafe {
            instance.get(GetTxsHashesReq::new)
        }
    }
}

impl ::protobuf::Message for GetTxsHashesReq {
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

impl ::protobuf::MessageStatic for GetTxsHashesReq {
    fn new() -> GetTxsHashesReq {
        GetTxsHashesReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTxsHashesReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetTxsHashesReq>(
                    "GetTxsHashesReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTxsHashesReq {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTxsHashesReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTxsHashesReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTxsHashesRes {
    // message fields
    pub tx_res: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTxsHashesRes {}

impl GetTxsHashesRes {
    pub fn new() -> GetTxsHashesRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTxsHashesRes {
        static mut instance: ::protobuf::lazy::Lazy<GetTxsHashesRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTxsHashesRes,
        };
        unsafe {
            instance.get(GetTxsHashesRes::new)
        }
    }

    // string tx_res = 1;

    pub fn clear_tx_res(&mut self) {
        self.tx_res.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_res(&mut self, v: ::std::string::String) {
        self.tx_res = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_res(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }

    // Take field
    pub fn take_tx_res(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tx_res, ::std::string::String::new())
    }

    pub fn get_tx_res(&self) -> &str {
        &self.tx_res
    }

    fn get_tx_res_for_reflect(&self) -> &::std::string::String {
        &self.tx_res
    }

    fn mut_tx_res_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }
}

impl ::protobuf::Message for GetTxsHashesRes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tx_res)?;
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
        if !self.tx_res.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tx_res);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_res.is_empty() {
            os.write_string(1, &self.tx_res)?;
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

impl ::protobuf::MessageStatic for GetTxsHashesRes {
    fn new() -> GetTxsHashesRes {
        GetTxsHashesRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTxsHashesRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tx_res",
                    GetTxsHashesRes::get_tx_res_for_reflect,
                    GetTxsHashesRes::mut_tx_res_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTxsHashesRes>(
                    "GetTxsHashesRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTxsHashesRes {
    fn clear(&mut self) {
        self.clear_tx_res();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTxsHashesRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTxsHashesRes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StoreTxsReq {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreTxsReq {}

impl StoreTxsReq {
    pub fn new() -> StoreTxsReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreTxsReq {
        static mut instance: ::protobuf::lazy::Lazy<StoreTxsReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreTxsReq,
        };
        unsafe {
            instance.get(StoreTxsReq::new)
        }
    }
}

impl ::protobuf::Message for StoreTxsReq {
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

impl ::protobuf::MessageStatic for StoreTxsReq {
    fn new() -> StoreTxsReq {
        StoreTxsReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreTxsReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StoreTxsReq>(
                    "StoreTxsReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreTxsReq {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StoreTxsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StoreTxsReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StoreTxsRes {
    // message fields
    pub tx_res: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreTxsRes {}

impl StoreTxsRes {
    pub fn new() -> StoreTxsRes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreTxsRes {
        static mut instance: ::protobuf::lazy::Lazy<StoreTxsRes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreTxsRes,
        };
        unsafe {
            instance.get(StoreTxsRes::new)
        }
    }

    // string tx_res = 1;

    pub fn clear_tx_res(&mut self) {
        self.tx_res.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_res(&mut self, v: ::std::string::String) {
        self.tx_res = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tx_res(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }

    // Take field
    pub fn take_tx_res(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.tx_res, ::std::string::String::new())
    }

    pub fn get_tx_res(&self) -> &str {
        &self.tx_res
    }

    fn get_tx_res_for_reflect(&self) -> &::std::string::String {
        &self.tx_res
    }

    fn mut_tx_res_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.tx_res
    }
}

impl ::protobuf::Message for StoreTxsRes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.tx_res)?;
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
        if !self.tx_res.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.tx_res);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.tx_res.is_empty() {
            os.write_string(1, &self.tx_res)?;
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

impl ::protobuf::MessageStatic for StoreTxsRes {
    fn new() -> StoreTxsRes {
        StoreTxsRes::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreTxsRes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tx_res",
                    StoreTxsRes::get_tx_res_for_reflect,
                    StoreTxsRes::mut_tx_res_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreTxsRes>(
                    "StoreTxsRes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreTxsRes {
    fn clear(&mut self) {
        self.clear_tx_res();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StoreTxsRes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StoreTxsRes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CleanTxsPoolReq {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CleanTxsPoolReq {}

impl CleanTxsPoolReq {
    pub fn new() -> CleanTxsPoolReq {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CleanTxsPoolReq {
        static mut instance: ::protobuf::lazy::Lazy<CleanTxsPoolReq> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CleanTxsPoolReq,
        };
        unsafe {
            instance.get(CleanTxsPoolReq::new)
        }
    }
}

impl ::protobuf::Message for CleanTxsPoolReq {
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

impl ::protobuf::MessageStatic for CleanTxsPoolReq {
    fn new() -> CleanTxsPoolReq {
        CleanTxsPoolReq::new()
    }

    fn descriptor_static(_: ::std::option::Option<CleanTxsPoolReq>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CleanTxsPoolReq>(
                    "CleanTxsPoolReq",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CleanTxsPoolReq {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CleanTxsPoolReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CleanTxsPoolReq {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Crypto {
    SECP = 0,
    SM2 = 1,
}

impl ::protobuf::ProtobufEnum for Crypto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Crypto> {
        match value {
            0 => ::std::option::Option::Some(Crypto::SECP),
            1 => ::std::option::Option::Some(Crypto::SM2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Crypto] = &[
            Crypto::SECP,
            Crypto::SM2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Crypto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Crypto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Crypto {
}

impl ::std::default::Default for Crypto {
    fn default() -> Self {
        Crypto::SECP
    }
}

impl ::protobuf::reflect::ProtobufValue for Crypto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nauth.proto\"\xd4\x01\n\x0bTransaction\x12\x0e\n\x02to\x18\x01\x20\
    \x01(\tR\x02to\x12\x14\n\x05nonce\x18\x02\x20\x01(\tR\x05nonce\x12\x14\n\
    \x05quota\x18\x03\x20\x01(\x04R\x05quota\x12*\n\x11valid_until_block\x18\
    \x04\x20\x01(\x04R\x0fvalidUntilBlock\x12\x12\n\x04data\x18\x05\x20\x01(\
    \x0cR\x04data\x12\x14\n\x05value\x18\x06\x20\x01(\x04R\x05value\x12\x19\
    \n\x08chain_id\x18\x07\x20\x01(\rR\x07chainId\x12\x18\n\x07version\x18\
    \x08\x20\x01(\rR\x07version\"\x86\x01\n\x15UnverifiedTransaction\x12.\n\
    \x0btransaction\x18\x01\x20\x01(\x0b2\x0c.TransactionR\x0btransaction\
    \x12\x1c\n\tsignature\x18\x02\x20\x01(\x0cR\tsignature\x12\x1f\n\x06cryp\
    to\x18\x03\x20\x01(\x0e2\x07.CryptoR\x06crypto\"&\n\x10AddUnverifyTxReq\
    \x12\x12\n\x04untx\x18\x01\x20\x01(\tR\x04untx\")\n\x10AddUnverifyTxRes\
    \x12\x15\n\x06tx_res\x18\x01\x20\x01(\tR\x05txRes\"\x13\n\x11VerifyBatch\
    TxsReq\"\"\n\tRpcStatus\x12\x15\n\x06tx_res\x18\x01\x20\x01(\tR\x05txRes\
    \"\x11\n\x0fGetTxsHashesReq\"(\n\x0fGetTxsHashesRes\x12\x15\n\x06tx_res\
    \x18\x01\x20\x01(\tR\x05txRes\"\r\n\x0bStoreTxsReq\"$\n\x0bStoreTxsRes\
    \x12\x15\n\x06tx_res\x18\x01\x20\x01(\tR\x05txRes\"\x11\n\x0fCleanTxsPoo\
    lReq*\x1b\n\x06Crypto\x12\x08\n\x04SECP\x10\0\x12\x07\n\x03SM2\x10\x012\
    \x83\x02\n\x04Auth\x127\n\rAddUnverifyTx\x12\x11.AddUnverifyTxReq\x1a\
    \x11.AddUnverifyTxRes\"\0\x122\n\x0eVerifyBatchTxs\x12\x12.VerifyBatchTx\
    sReq\x1a\n.RpcStatus\"\0\x124\n\x0cGetTxsHashes\x12\x10.GetTxsHashesReq\
    \x1a\x10.GetTxsHashesRes\"\0\x12(\n\x08StoreTxs\x12\x0c.StoreTxsReq\x1a\
    \x0c.StoreTxsRes\"\0\x12.\n\x0cCleanTxsPool\x12\x10.CleanTxsPoolReq\x1a\
    \n.RpcStatus\"\0J\xb2\x0e\n\x06\x12\x04\0\0I\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x05\0\x01\x12\
    \x03\x02\x05\x0b\n\x0b\n\x04\x05\0\x02\0\x12\x03\x03\x02\x0b\n\x0c\n\x05\
    \x05\0\x02\0\x01\x12\x03\x03\x02\x06\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\
    \x03\t\n\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x04\x02\n\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03\x04\x02\x05\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\
    \x04\x08\t\n\n\n\x02\x04\0\x12\x04\x07\0\x10\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x07\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x08\x02\x10\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04\x08\x02\x07\x15\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x08\t\x0b\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x08\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\t\x02\x13\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\t\x02\x08\x10\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\t\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\t\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\t\x11\x12\n\x0b\n\
    \x04\x04\0\x02\x02\x12\x03\n\x02\x13\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\
    \n\x02\t\x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\n\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\n\t\x0e\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\
    \n\x11\x12\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0b\x02\x1f\n\r\n\x05\x04\0\
    \x02\x03\x04\x12\x04\x0b\x02\n\x13\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\
    \x0b\x02\x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0b\t\x1a\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x03\x0b\x1d\x1e\n\x0b\n\x04\x04\0\x02\x04\x12\x03\
    \x0c\x02\x11\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x0c\x02\x0b\x1f\n\x0c\n\
    \x05\x04\0\x02\x04\x05\x12\x03\x0c\x02\x07\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03\x0c\x08\x0c\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0c\x0f\x10\n\
    \x0b\n\x04\x04\0\x02\x05\x12\x03\r\x02\x13\n\r\n\x05\x04\0\x02\x05\x04\
    \x12\x04\r\x02\x0c\x11\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\r\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03\r\t\x0e\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03\r\x11\x12\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x0e\x02\x16\n\r\
    \n\x05\x04\0\x02\x06\x04\x12\x04\x0e\x02\r\x13\n\x0c\n\x05\x04\0\x02\x06\
    \x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x0e\t\x11\
    \n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x0e\x14\x15\n\x0b\n\x04\x04\0\x02\
    \x07\x12\x03\x0f\x02\x15\n\r\n\x05\x04\0\x02\x07\x04\x12\x04\x0f\x02\x0e\
    \x16\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x07\x01\x12\x03\x0f\t\x10\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x0f\
    \x13\x14\n\n\n\x02\x04\x01\x12\x04\x12\0\x16\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\x12\x08\x1d\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x13\x02\x1e\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x13\x02\x12\x1f\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03\x13\x02\r\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x13\x0e\x19\
    \n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x13\x1c\x1d\n\x0b\n\x04\x04\x01\
    \x02\x01\x12\x03\x14\x02\x16\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x14\
    \x02\x13\x1e\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x14\x02\x07\n\x0c\n\
    \x05\x04\x01\x02\x01\x01\x12\x03\x14\x08\x11\n\x0c\n\x05\x04\x01\x02\x01\
    \x03\x12\x03\x14\x14\x15\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x15\x02\x14\
    \n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x15\x02\x14\x16\n\x0c\n\x05\x04\
    \x01\x02\x02\x06\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\
    \x03\x15\t\x0f\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x15\x12\x13\n\n\n\
    \x02\x04\x02\x12\x04\x19\0\x1c\x01\n\n\n\x03\x04\x02\x01\x12\x03\x19\x08\
    \x18\n.\n\x04\x04\x02\x02\0\x12\x03\x1b\x02\x12\x1a!\x20UnverifiedTransa\
    ction\x20untx\x20=\x201;\n\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1b\x02\
    \x19\x1a\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x1b\t\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\
    \x1b\x10\x11\n\n\n\x02\x04\x03\x12\x04\x1e\0\x20\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03\x1e\x08\x18\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1f\x02\x14\n\
    \r\n\x05\x04\x03\x02\0\x04\x12\x04\x1f\x02\x1e\x1a\n\x0c\n\x05\x04\x03\
    \x02\0\x05\x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1f\
    \t\x0f\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\x12\x13\n\n\n\x02\x04\
    \x04\x12\x04\"\0$\x01\n\n\n\x03\x04\x04\x01\x12\x03\"\x08\x19\n\n\n\x02\
    \x04\x05\x12\x04&\0(\x01\n\n\n\x03\x04\x05\x01\x12\x03&\x08\x11\n\x0b\n\
    \x04\x04\x05\x02\0\x12\x03'\x02\x14\n\r\n\x05\x04\x05\x02\0\x04\x12\x04'\
    \x02&\x13\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03'\x02\x08\n\x0c\n\x05\x04\
    \x05\x02\0\x01\x12\x03'\t\x0f\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03'\x12\
    \x13\n\n\n\x02\x04\x06\x12\x04*\0,\x01\n\n\n\x03\x04\x06\x01\x12\x03*\
    \x08\x17\n\n\n\x02\x04\x07\x12\x04.\00\x01\n\n\n\x03\x04\x07\x01\x12\x03\
    .\x08\x17\n\x0b\n\x04\x04\x07\x02\0\x12\x03/\x02\x14\n\r\n\x05\x04\x07\
    \x02\0\x04\x12\x04/\x02.\x19\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03/\x02\
    \x08\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03/\t\x0f\n\x0c\n\x05\x04\x07\
    \x02\0\x03\x12\x03/\x12\x13\n\n\n\x02\x04\x08\x12\x042\04\x01\n\n\n\x03\
    \x04\x08\x01\x12\x032\x08\x13\n\n\n\x02\x04\t\x12\x046\08\x01\n\n\n\x03\
    \x04\t\x01\x12\x036\x08\x13\n\x0b\n\x04\x04\t\x02\0\x12\x037\x02\x14\n\r\
    \n\x05\x04\t\x02\0\x04\x12\x047\x026\x15\n\x0c\n\x05\x04\t\x02\0\x05\x12\
    \x037\x02\x08\n\x0c\n\x05\x04\t\x02\0\x01\x12\x037\t\x0f\n\x0c\n\x05\x04\
    \t\x02\0\x03\x12\x037\x12\x13\n\n\n\x02\x04\n\x12\x04:\0<\x01\n\n\n\x03\
    \x04\n\x01\x12\x03:\x08\x17\n\n\n\x02\x06\0\x12\x04?\0I\x01\n\n\n\x03\
    \x06\0\x01\x12\x03?\x08\x0c\n\x0b\n\x04\x06\0\x02\0\x12\x03@\x02D\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03@\x06\x13\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03@\x15%\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03@0@\n\x0b\n\x04\x06\0\x02\
    \x01\x12\x03B\x02?\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03B\x06\x14\n\x0c\
    \n\x05\x06\0\x02\x01\x02\x12\x03B\x16'\n\x0c\n\x05\x06\0\x02\x01\x03\x12\
    \x03B2;\n\x0b\n\x04\x06\0\x02\x02\x12\x03D\x02A\n\x0c\n\x05\x06\0\x02\
    \x02\x01\x12\x03D\x06\x12\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03D\x14#\n\
    \x0c\n\x05\x06\0\x02\x02\x03\x12\x03D.=\n\x0b\n\x04\x06\0\x02\x03\x12\
    \x03F\x025\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03F\x06\x0e\n\x0c\n\x05\
    \x06\0\x02\x03\x02\x12\x03F\x10\x1b\n\x0c\n\x05\x06\0\x02\x03\x03\x12\
    \x03F&1\n\x0b\n\x04\x06\0\x02\x04\x12\x03H\x02;\n\x0c\n\x05\x06\0\x02\
    \x04\x01\x12\x03H\x06\x12\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03H\x14#\n\
    \x0c\n\x05\x06\0\x02\x04\x03\x12\x03H.7b\x06proto3\
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
