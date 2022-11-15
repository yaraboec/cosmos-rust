import { AppConfig, getAppConfig, NetworkConfigs } from "./logic/config";

const local: AppConfig = {
  chainId: "testing",
  chainName: "Testing",
  addressPrefix: "cosmos",
  rpcUrl: "http://localhost:26657",
  httpUrl: "http://localhost:1317",
  faucetUrl: "http://localhost:8000",
  feeToken: "ucosm",
  stakingToken: "uatom",
  gasPrice: 0.025,
};

const malaga: AppConfig = {
  chainId: "malaga-420",
  chainName: "malaga-420",
  addressPrefix: "wasm",
  rpcUrl: "https://rpc.malaga-420.cosmwasm.com:443",
  httpUrl: "https://api.malaga-420.cosmwasm.com",
  faucetUrl: "https://faucet.malaga-420.cosmwasm.com",
  feeToken: "umlg",
  stakingToken: "uand",
  gasPrice: 0.025,
};

const configs: NetworkConfigs = { local, malaga };
export const config = malaga;
