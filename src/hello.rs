// This file is generated by rust-protobuf 2.24.1. Do not edit
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
//! Generated file from `hello.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    pub Message: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Request {
    fn default() -> &'a Request {
        <Request as ::protobuf::Message>::default_instance()
    }
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    // string Message = 1;


    pub fn get_Message(&self) -> &str {
        &self.Message
    }
    pub fn clear_Message(&mut self) {
        self.Message.clear();
    }

    // Param is passed by value, moved
    pub fn set_Message(&mut self, v: ::std::string::String) {
        self.Message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Message(&mut self) -> &mut ::std::string::String {
        &mut self.Message
    }

    // Take field
    pub fn take_Message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.Message, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.Message)?;
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
        if !self.Message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.Message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.Message.is_empty() {
            os.write_string(1, &self.Message)?;
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

    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "Message",
                |m: &Request| { &m.Message },
                |m: &mut Request| { &mut m.Message },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Request>(
                "Request",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Request {
        static instance: ::protobuf::rt::LazyV2<Request> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Request::new)
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.Message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    pub Message: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    // string Message = 1;


    pub fn get_Message(&self) -> &str {
        &self.Message
    }
    pub fn clear_Message(&mut self) {
        self.Message.clear();
    }

    // Param is passed by value, moved
    pub fn set_Message(&mut self, v: ::std::string::String) {
        self.Message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Message(&mut self) -> &mut ::std::string::String {
        &mut self.Message
    }

    // Take field
    pub fn take_Message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.Message, ::std::string::String::new())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.Message)?;
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
        if !self.Message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.Message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.Message.is_empty() {
            os.write_string(1, &self.Message)?;
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

    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "Message",
                |m: &Response| { &m.Message },
                |m: &mut Response| { &mut m.Message },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Response>(
                "Response",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Response {
        static instance: ::protobuf::rt::LazyV2<Response> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Response::new)
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.Message.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bhello.proto\x12\x05types\x1a\x14gogoproto/gogo.proto\x1a\x1fgoogle\
    /protobuf/timestamp.proto\"#\n\x07Request\x12\x18\n\x07Message\x18\x01\
    \x20\x01(\tR\x07Message\"$\n\x08Response\x12\x18\n\x07Message\x18\x01\
    \x20\x01(\tR\x07MessageB\x0c\xd0\xe2\x1e\x01\xc8\xe1\x1e\0\xc8\xe2\x1e\
    \x01J\xf7\x02\n\x06\x12\x04\0\0\x12\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\
    \n\x08\n\x01\x02\x12\x03\x01\x08\r\n\t\n\x02\x03\0\x12\x03\x03\x07\x1d\n\
    \t\n\x02\x03\x01\x12\x03\x04\x07(\n\x08\n\x01\x08\x12\x03\x06\0(\n\x0b\n\
    \x04\x08\xa9\xec\x03\x12\x03\x06\0(\n\x08\n\x01\x08\x12\x03\x07\0*\n\x0b\
    \n\x04\x08\xaa\xec\x03\x12\x03\x07\0*\n\x08\n\x01\x08\x12\x03\x08\0/\n\
    \x0b\n\x04\x08\x99\xec\x03\x12\x03\x08\0/\n\n\n\x02\x04\0\x12\x04\n\0\r\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x0f\n-\n\x04\x04\0\x02\0\x12\x03\
    \x0c\x04\x17\x1a\x20\x20Message\x20is\x20the\x20message\x20to\x20pass\n\
    \n\r\n\x05\x04\0\x02\0\x04\x12\x04\x0c\x04\n\x11\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x0c\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\x0b\x12\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x15\x16\n\n\n\x02\x04\x01\x12\x04\
    \x0f\0\x12\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0f\x08\x10\n-\n\x04\x04\
    \x01\x02\0\x12\x03\x11\x04\x17\x1a\x20\x20Message\x20is\x20the\x20messag\
    e\x20to\x20pass\n\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x11\x04\x0f\x12\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x11\x04\n\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x11\x0b\x12\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\x15\
    \x16b\x06proto3\
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
