// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `nacos_grpc_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Metadata {
    // message fields
    pub field_type: ::std::string::String,
    pub clientIp: ::std::string::String,
    pub headers: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Metadata {
    fn default() -> &'a Metadata {
        <Metadata as ::protobuf::Message>::default_instance()
    }
}

impl Metadata {
    pub fn new() -> Metadata {
        ::std::default::Default::default()
    }

    // string type = 3;


    pub fn get_field_type(&self) -> &str {
        &self.field_type
    }
    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        &mut self.field_type
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.field_type, ::std::string::String::new())
    }

    // string clientIp = 8;


    pub fn get_clientIp(&self) -> &str {
        &self.clientIp
    }
    pub fn clear_clientIp(&mut self) {
        self.clientIp.clear();
    }

    // Param is passed by value, moved
    pub fn set_clientIp(&mut self, v: ::std::string::String) {
        self.clientIp = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientIp(&mut self) -> &mut ::std::string::String {
        &mut self.clientIp
    }

    // Take field
    pub fn take_clientIp(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.clientIp, ::std::string::String::new())
    }

    // repeated .Metadata.headers_MapEntry headers = 7;


    pub fn get_headers(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.headers
    }
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.headers, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for Metadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.field_type)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.clientIp)?;
                },
                7 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.headers)?;
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
        if !self.field_type.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.field_type);
        }
        if !self.clientIp.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.clientIp);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(7, &self.headers);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.field_type.is_empty() {
            os.write_string(3, &self.field_type)?;
        }
        if !self.clientIp.is_empty() {
            os.write_string(8, &self.clientIp)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(7, &self.headers, os)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Metadata {
        Metadata::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "type",
                |m: &Metadata| { &m.field_type },
                |m: &mut Metadata| { &mut m.field_type },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "clientIp",
                |m: &Metadata| { &m.clientIp },
                |m: &mut Metadata| { &mut m.clientIp },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "headers",
                |m: &Metadata| { &m.headers },
                |m: &mut Metadata| { &mut m.headers },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Metadata>(
                "Metadata",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Metadata {
        static instance: ::protobuf::rt::LazyV2<Metadata> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Metadata::new)
    }
}

impl ::protobuf::Clear for Metadata {
    fn clear(&mut self) {
        self.field_type.clear();
        self.clientIp.clear();
        self.headers.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Metadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Metadata {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Payload {
    // message fields
    pub metadata: ::protobuf::SingularPtrField<Metadata>,
    pub body: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Payload {
    fn default() -> &'a Payload {
        <Payload as ::protobuf::Message>::default_instance()
    }
}

impl Payload {
    pub fn new() -> Payload {
        ::std::default::Default::default()
    }

    // .Metadata metadata = 2;


    pub fn get_metadata(&self) -> &Metadata {
        self.metadata.as_ref().unwrap_or_else(|| <Metadata as ::protobuf::Message>::default_instance())
    }
    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: Metadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut Metadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> Metadata {
        self.metadata.take().unwrap_or_else(|| Metadata::new())
    }

    // .google.protobuf.Any body = 3;


    pub fn get_body(&self) -> &::protobuf::well_known_types::Any {
        self.body.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Any as ::protobuf::Message>::default_instance())
    }
    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::protobuf::well_known_types::Any) {
        self.body = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::protobuf::well_known_types::Any {
        self.body.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }
}

impl ::protobuf::Message for Payload {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.body {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.body)?;
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
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.body.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.body.as_ref() {
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Payload {
        Payload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Metadata>>(
                "metadata",
                |m: &Payload| { &m.metadata },
                |m: &mut Payload| { &mut m.metadata },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                "body",
                |m: &Payload| { &m.body },
                |m: &mut Payload| { &mut m.body },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Payload>(
                "Payload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Payload {
        static instance: ::protobuf::rt::LazyV2<Payload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Payload::new)
    }
}

impl ::protobuf::Clear for Payload {
    fn clear(&mut self) {
        self.metadata.clear();
        self.body.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Payload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18nacos_grpc_service.proto\x1a\x19google/protobuf/any.proto\x1a\x1fg\
    oogle/protobuf/timestamp.proto\"\xb4\x01\n\x08Metadata\x12\x14\n\x04type\
    \x18\x03\x20\x01(\tR\x04typeB\0\x12\x1c\n\x08clientIp\x18\x08\x20\x01(\t\
    R\x08clientIpB\0\x126\n\x07headers\x18\x07\x20\x03(\x0b2\x1a.Metadata.he\
    aders_MapEntryR\x07headersB\0\x1a:\n\x10headers_MapEntry\x12\x0e\n\x03ke\
    y\x18\x01(\tR\x03key\x12\x12\n\x05value\x18\x02(\tR\x05value:\x028\x01:\
    \0\"`\n\x07Payload\x12'\n\x08metadata\x18\x02\x20\x01(\x0b2\t.MetadataR\
    \x08metadataB\0\x12*\n\x04body\x18\x03\x20\x01(\x0b2\x14.google.protobuf\
    .AnyR\x04bodyB\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}