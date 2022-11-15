import { makeCosmoshubPath } from "@cosmjs/stargate";
import { Random } from "@cosmjs/crypto";
import { Bip39 } from "@cosmjs/crypto";
import {
  DirectSecp256k1HdWallet,
  OfflineDirectSigner,
} from "@cosmjs/proto-signing";

export async function loadOrCreateWallet(
  addressPrefix?: string
): Promise<OfflineDirectSigner> {
  const mnemonic = loadOrCreateMnemonic();
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    hdPaths: [makeCosmoshubPath(0)],
    prefix: addressPrefix,
  });

  return wallet;
}

export function loadOrCreateMnemonic(): string {
  const key = "mnemonic";

  const loaded = localStorage.getItem(key);
  if (loaded) {
    return loaded;
  }

  const generated = generateMnemonic();
  localStorage.setItem(key, generated);

  return generated;
}

export function generateMnemonic(): string {
  return Bip39.encode(Random.getBytes(16)).toString();
}
