use self::api::ApiClient;

struct GetSubDomainsRequest {
    client: &ApiClient,
}

impl GetSubDomainsRequest {
    pub fn new(client: &ApiClient, domain: &str) {
    }
}

struct AddSubDomainRequest {
    client: &ApiClient,
}

impl AddSubDomainRequest {
    pub fn new(client: &ApiClient, domain: &str, subdomain: &str) {
    }
}
