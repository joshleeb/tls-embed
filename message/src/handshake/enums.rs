#[macro_use]
mod macros;

msg_enum! {
    HandshakeType, u8;
    {
        HelloRequest => 0x00,
        ClientHello => 0x01,
        ServerHello => 0x02,
        NewSessionTicket => 0x04,
        EndOfEarlyData => 0x05,
        HelloRetryRequest => 0x06,
        EncryptedExtensions => 0x08,
        Certificate => 0x0b,
        ServerKeyExchange => 0x0c,
        CertificateRequest => 0x0d,
        ServerHelloDone => 0x0e,
        CertificateVerify => 0x0f,
        ClientKeyExchange => 0x10,
        Finished => 0x14,
        CertificateURL => 0x15,
        CertificateStatus => 0x16,
        KeyUpdate => 0x18,
        MessageHash => 0xfe,
    }
}

msg_enum! {
    ProtocolVersion, u16;
    {
        TLSv1_2 => 0x0303,
        TLSv1_3 => 0x0304,
    }
}
enum_default!(ProtocolVersion, TLSv1_2);

msg_enum! {
    CompressionMethod, u8;
    {
        Null => 0x00,
        Deflate => 0x01,
        LSZ => 0x40
    }
}
enum_default!(CompressionMethod, Null);

msg_enum! {
    CipherSuite, u16;
    {
        TlsAes128GcmSha256 => 0x1301,
        TlsAes256GcmSha384 => 0x1302,
        TlsChaCha20Poly1305Sha256 => 0x1303,
    }
}
enum_default!(CipherSuite, TlsAes128GcmSha256);

msg_enum! {
    ExtensionType, u16;
    {
        SignatureAlgorithms => 0x000d,
        SupportedVersions => 0x002b,
    }
}

msg_enum! {
    SignatureScheme, u16;
    {
        // RSASSA-PKCS1-v1_5 algorithms.
        RsaPkcs1Sha256 => 0x0401,
        RsaPkcs1Sha384 => 0x0501,
        RsaPkcs1Sha512 => 0x0601,

        // ECDSA algorithms.
        EcdsaNistp256Sha256 => 0x0403,
        EcdsaNistp384Sha384 => 0x0503,
        EcdsaNistp521Sha512 => 0x0603,

        // RSASSA-PSS algorithms with public key OID rsaEncryption.
        RsaPssSha256 => 0x0804,
        RsaPssSha384 => 0x0805,
        RsaPssSha512 => 0x0806,

        // EdDSA algorithms.
        Ed25519 => 0x0807,
        Ed448 => 0x0808,

        // Legacy algorithms.
        RsaPkcs1Sha1 => 0x0201,
        EcdsaSha1Legacy => 0x0203,
    }
}
