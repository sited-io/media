pub mod peoplesmarkets {
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        include_bytes!("./FILE_DESCRIPTOR_SET");

    pub mod media {
        pub mod v1 {
            include!("peoplesmarkets.media.v1.rs");
        }
    }

    pub mod commerce {
        pub mod v1 {
            include!("peoplesmarkets.commerce.v1.rs");
        }
    }

    pub mod payment {
        pub mod v1 {
            include!("peoplesmarkets.payment.v1.rs");
        }
    }

    pub mod ordering {
        pub mod v1 {
            include!("peoplesmarkets.ordering.v1.rs");
        }
    }
    pub mod pagination {
        pub mod v1 {
            include!("peoplesmarkets.pagination.v1.rs");
        }
    }
}
