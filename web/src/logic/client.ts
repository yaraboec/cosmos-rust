import {
  SigningCosmWasmClient,
  SigningCosmWasmClientOptions,
} from "@cosmjs/cosmwasm-stargate";
import { OfflineDirectSigner } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import { AppConfig } from "./config";

export async function createSigningClient(
  config: AppConfig,
  signer: OfflineDirectSigner
): Promise<SigningCosmWasmClient> {
  const options: SigningCosmWasmClientOptions = {
    prefix: config.addressPrefix,
    gasPrice: GasPrice.fromString(`${config.gasPrice}${config.feeToken}`),
  };

  return SigningCosmWasmClient.connectWithSigner(
    config.rpcUrl,
    signer,
    options
  );
}
