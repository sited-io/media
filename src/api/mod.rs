pub mod sited_io {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("./FILE_DESCRIPTOR_SET");

    pub mod media {
        pub mod v1 {
            include!("sited_io.media.v1.rs");
        }
    }

    pub mod commerce {
        pub mod v1 {
            include!("sited_io.commerce.v1.rs");
        }
    }

    pub mod payment {
        pub mod v1 {
            include!("sited_io.payment.v1.rs");
        }
    }

    pub mod ordering {
        pub mod v1 {
            include!("sited_io.ordering.v1.rs");
        }
    }
    pub mod pagination {
        pub mod v1 {
            include!("sited_io.pagination.v1.rs");
        }
    }
}
