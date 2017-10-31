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

const METHOD_IMPORT_INGEST_SST: ::grpcio::Method<super::importpb::IngestSSTRequest, super::importpb::IngestSSTResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.Import/IngestSST",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_UPLOAD_SST: ::grpcio::Method<super::importpb::UploadSSTRequest, super::importpb::UploadSSTResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/importpb.Import/UploadSST",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ImportClient {
    client: ::grpcio::Client,
}

impl ImportClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn ingest_sst_opt(&self, req: super::importpb::IngestSSTRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::IngestSSTResponse> {
        self.client.unary_call(&METHOD_IMPORT_INGEST_SST, req, opt)
    }

    pub fn ingest_sst(&self, req: super::importpb::IngestSSTRequest) -> ::grpcio::Result<super::importpb::IngestSSTResponse> {
        self.ingest_sst_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ingest_sst_async_opt(&self, req: super::importpb::IngestSSTRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::IngestSSTResponse> {
        self.client.unary_call_async(&METHOD_IMPORT_INGEST_SST, req, opt)
    }

    pub fn ingest_sst_async(&self, req: super::importpb::IngestSSTRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::IngestSSTResponse> {
        self.ingest_sst_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn upload_sst_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientCStreamSender<super::importpb::UploadSSTRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::UploadSSTResponse>) {
        self.client.client_streaming(&METHOD_IMPORT_UPLOAD_SST, opt)
    }

    pub fn upload_sst(&self) -> (::grpcio::ClientCStreamSender<super::importpb::UploadSSTRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::UploadSSTResponse>) {
        self.upload_sst_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Import {
    fn ingest_sst(&self, ctx: ::grpcio::RpcContext, req: super::importpb::IngestSSTRequest, sink: ::grpcio::UnarySink<super::importpb::IngestSSTResponse>);
    fn upload_sst(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::importpb::UploadSSTRequest>, sink: ::grpcio::ClientStreamingSink<super::importpb::UploadSSTResponse>);
}

pub fn create_import<S: Import + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_INGEST_SST, move |ctx, req, resp| {
        instance.ingest_sst(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_UPLOAD_SST, move |ctx, req, resp| {
        instance.upload_sst(ctx, req, resp)
    });
    builder.build()
}