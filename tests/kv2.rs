mod common;

use common::VaultServer;
use vaultrs::error::ClientError;

mod config {
    use crate::{setup, VaultServer};
    use vaultrs::{api::kv2::requests::SetConfigurationRequest, kv2::config};

    #[test]
    fn test_read() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let resp = config::read(&server.client, endpoint.path.as_str());

        assert!(resp.is_ok());
        assert_eq!(resp.unwrap().max_versions, 0);
    }

    #[test]
    fn test_set() {
        let docker = testcontainers::clients::Cli::default();
        let server = VaultServer::new(&docker);
        let endpoint = setup(&server).unwrap();

        let versions: u64 = 100;
        let resp = config::set(
            &server.client,
            endpoint.path.as_str(),
            Some(
                SetConfigurationRequest::builder()
                    .max_versions(versions)
                    .delete_version_after("768h"),
            ),
        );

        assert!(resp.is_ok());
    }
}

#[derive(Debug)]
struct SecretEndpoint {
    pub path: String,
    pub secret: TestSecret,
}

#[derive(Debug)]
struct TestSecret {
    key: String,
    password: String,
}

fn setup(server: &VaultServer) -> Result<SecretEndpoint, ClientError> {
    let path = "secret_test";
    let secret = TestSecret {
        key: "mykey".to_string(),
        password: "supersecret".to_string(),
    };
    let endpoint = SecretEndpoint {
        path: path.to_string(),
        secret,
    };

    // Mount the PKI engine
    server.mount(path, "kv-v2")?;

    Ok(endpoint)
}
