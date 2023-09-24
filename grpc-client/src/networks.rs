pub const MAIN_NET: NetworkInfo = NetworkInfo {
    rpc_endpoint: "https://rpc.gotabit.dev:443",
    grpc_endpoint: "https://grpc.gotabit.dev:443",
    rest_endpoint: "https://rest.gotabit.dev:443",
    chain_id: "gotabit-alpha",
    name: "GotaBit",
    denom: "ugtb",
    decimals: 6,
};

pub const TEST_NET: NetworkInfo = NetworkInfo {
    rpc_endpoint: "https://rpc-testnet.gotabit.dev:443",
    grpc_endpoint: "https://grpc-testnet.gotabit.dev:443",
    rest_endpoint: "https://rest-testnet.gotabit.dev:443",
    chain_id: "gotabit-test-1",
    name: "GotaBit-test",
    denom: "ugtb",
    decimals: 6,
};

pub const DEV_NET: NetworkInfo = NetworkInfo {
    rpc_endpoint: "https://rpc-devnet.gotabit.dev:443",
    grpc_endpoint: "https://grpc-devnet.gotabit.dev:443",
    rest_endpoint: "https://rest-devnet.gotabit.dev:443",
    chain_id: "otabit-dev-1",
    name: "GotaBit-dev",
    denom: "ugtb",
    decimals: 6,
};

pub struct NetworkInfo {
    pub rpc_endpoint: &'static str,
    pub grpc_endpoint: &'static str,
    pub chain_id: &'static str,
    pub name: &'static str,
    pub rest_endpoint: &'static str,
    pub denom: &'static str,
    pub decimals: u8,
}
