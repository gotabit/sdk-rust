// @generated
#[cfg(feature = "amino")]
// @@protoc_insertion_point(attribute:amino)
pub mod amino {
    include!("amino.rs");
    // @@protoc_insertion_point(amino)
}
pub mod cosmos {
    pub mod base {
        pub mod query {
            #[cfg(feature = "cosmos-base-query-v1beta1")]
            // @@protoc_insertion_point(attribute:cosmos.base.query.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.query.v1beta1)
            }
        }
        #[cfg(feature = "cosmos-base-v1beta1")]
        // @@protoc_insertion_point(attribute:cosmos.base.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.base.v1beta1)
        }
    }
    pub mod msg {
        #[cfg(feature = "cosmos-msg-v1")]
        // @@protoc_insertion_point(attribute:cosmos.msg.v1)
        pub mod v1 {
            include!("cosmos.msg.v1.rs");
            // @@protoc_insertion_point(cosmos.msg.v1)
        }
    }
}
#[cfg(feature = "cosmos_proto")]
// @@protoc_insertion_point(attribute:cosmos_proto)
pub mod cosmos_proto {
    include!("cosmos_proto.rs");
    // @@protoc_insertion_point(cosmos_proto)
}
pub mod cosmwasm {
    pub mod wasm {
        #[cfg(feature = "cosmwasm-wasm-v1")]
        // @@protoc_insertion_point(attribute:cosmwasm.wasm.v1)
        pub mod v1 {
            include!("cosmwasm.wasm.v1.rs");
            // @@protoc_insertion_point(cosmwasm.wasm.v1)
        }
    }
}
#[cfg(feature = "gogoproto")]
// @@protoc_insertion_point(attribute:gogoproto)
pub mod gogoproto {
    include!("gogoproto.rs");
    // @@protoc_insertion_point(gogoproto)
}
pub mod google {
    #[cfg(feature = "google-api")]
    // @@protoc_insertion_point(attribute:google.api)
    pub mod api {
        include!("google.api.rs");
        // @@protoc_insertion_point(google.api)
    }
}
